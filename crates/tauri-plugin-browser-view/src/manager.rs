use crate::error::{Error, Result};
use crate::models::{CreateTabOptions, TabBounds, TabInfo};
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::{
    AppHandle, LogicalPosition, LogicalSize, Manager, Runtime, WebviewUrl, WebviewWindowBuilder,
};

pub struct TabManager {
    tabs: Mutex<HashMap<String, TabInfo>>,
    active_tab: Mutex<Option<String>>,
}

impl TabManager {
    pub fn new() -> Self {
        Self {
            tabs: Mutex::new(HashMap::new()),
            active_tab: Mutex::new(None),
        }
    }

    pub fn create_tab<R: Runtime>(
        &self,
        app: &AppHandle<R>,
        options: CreateTabOptions,
    ) -> Result<TabInfo> {
        let mut tabs = self.tabs.lock().unwrap();
        if tabs.contains_key(&options.label) {
            return Err(Error::TabAlreadyExists(options.label));
        }

        let win_label = options.window_label.as_deref().unwrap_or("main");
        let parent_window = app
            .get_webview_window(win_label)
            .ok_or_else(|| Error::WindowNotFound(win_label.to_string()))?;

        let parsed_url = options
            .url
            .parse::<url::Url>()
            .map_err(|e| Error::InvalidUrl(e.to_string()))?;
        let webview_url = WebviewUrl::External(parsed_url);

        // Build child WebviewWindow
        let mut builder = WebviewWindowBuilder::new(app, &options.label, webview_url)
            .decorations(false)
            .position(options.bounds.x, options.bounds.y)
            .inner_size(options.bounds.width, options.bounds.height);

        // Attach as child to the parent window
        builder = builder
            .parent(&parent_window)
            .map_err(|e| Error::Webview(e.to_string()))?;

        if let Some(ua) = &options.user_agent {
            builder = builder.user_agent(ua);
        }

        if let Some(partition) = &options.partition {
            if let Ok(app_dir) = app.path().app_data_dir() {
                let partition_dir = app_dir.join("partitions").join(partition);
                builder = builder.data_directory(partition_dir);
            }
        }

        builder
            .build()
            .map_err(|e| Error::Webview(e.to_string()))?;

        let tab_info = TabInfo {
            label: options.label.clone(),
            url: options.url,
            bounds: options.bounds,
            is_active: true,
            partition: options.partition,
        };

        tabs.insert(options.label.clone(), tab_info.clone());
        let mut active = self.active_tab.lock().unwrap();
        *active = Some(options.label);

        Ok(tab_info)
    }

    pub fn close_tab<R: Runtime>(&self, app: &AppHandle<R>, label: &str) -> Result<()> {
        let mut tabs = self.tabs.lock().unwrap();
        if tabs.remove(label).is_none() {
            return Err(Error::TabNotFound(label.to_string()));
        }

        if let Some(window) = app.get_webview_window(label) {
            window.close().map_err(|e| Error::Webview(e.to_string()))?;
        }

        let mut active = self.active_tab.lock().unwrap();
        if active.as_deref() == Some(label) {
            *active = tabs.keys().next().cloned();
        }

        Ok(())
    }

    pub fn set_tab_bounds<R: Runtime>(
        &self,
        app: &AppHandle<R>,
        label: &str,
        bounds: TabBounds,
    ) -> Result<()> {
        let mut tabs = self.tabs.lock().unwrap();
        let tab = tabs
            .get_mut(label)
            .ok_or_else(|| Error::TabNotFound(label.to_string()))?;

        if let Some(window) = app.get_webview_window(label) {
            let pos = LogicalPosition::new(bounds.x, bounds.y);
            let size = LogicalSize::new(bounds.width, bounds.height);
            let _ = window.set_position(pos);
            let _ = window.set_size(size);
        }

        tab.bounds = bounds;
        Ok(())
    }

    pub fn switch_tab<R: Runtime>(&self, app: &AppHandle<R>, label: &str) -> Result<()> {
        let mut tabs = self.tabs.lock().unwrap();
        if !tabs.contains_key(label) {
            return Err(Error::TabNotFound(label.to_string()));
        }

        for (name, tab) in tabs.iter_mut() {
            if let Some(window) = app.get_webview_window(name) {
                if name == label {
                    tab.is_active = true;
                    let _ = window.show();
                    let _ = window.set_focus();
                } else {
                    tab.is_active = false;
                    let _ = window.hide();
                }
            }
        }

        let mut active = self.active_tab.lock().unwrap();
        *active = Some(label.to_string());

        Ok(())
    }

    pub fn navigate_tab<R: Runtime>(
        &self,
        app: &AppHandle<R>,
        label: &str,
        url: &str,
    ) -> Result<()> {
        let mut tabs = self.tabs.lock().unwrap();
        let tab = tabs
            .get_mut(label)
            .ok_or_else(|| Error::TabNotFound(label.to_string()))?;

        let _parsed = url
            .parse::<url::Url>()
            .map_err(|e| Error::InvalidUrl(e.to_string()))?;

        if let Some(window) = app.get_webview_window(label) {
            let script = format!("window.location.href = {};", serde_json::to_string(url).unwrap());
            window
                .eval(&script)
                .map_err(|e| Error::Webview(e.to_string()))?;
        }

        tab.url = url.to_string();
        Ok(())
    }

    pub fn eval_script<R: Runtime>(
        &self,
        app: &AppHandle<R>,
        label: &str,
        script: &str,
    ) -> Result<()> {
        let tabs = self.tabs.lock().unwrap();
        if !tabs.contains_key(label) {
            return Err(Error::TabNotFound(label.to_string()));
        }

        if let Some(window) = app.get_webview_window(label) {
            window
                .eval(script)
                .map_err(|e| Error::Webview(e.to_string()))?;
        }

        Ok(())
    }

    pub fn list_tabs(&self) -> Vec<TabInfo> {
        let tabs = self.tabs.lock().unwrap();
        tabs.values().cloned().collect()
    }
}
