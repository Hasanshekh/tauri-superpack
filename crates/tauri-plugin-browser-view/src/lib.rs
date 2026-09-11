use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub mod commands;
pub mod error;
pub mod manager;
pub mod models;

pub use error::{Error, Result};
pub use manager::TabManager;
pub use models::{CreateTabOptions, TabBounds, TabInfo};

/// Initialize the BrowserView Tab & Session Orchestration plugin for Tauri v2.
/// Enables dynamic child webview tabs, cookie partition isolation, and responsive device bounds.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("browser-view")
        .invoke_handler(tauri::generate_handler![
            commands::create_tab,
            commands::close_tab,
            commands::set_tab_bounds,
            commands::switch_tab,
            commands::navigate_tab,
            commands::eval_script_in_tab,
            commands::list_tabs,
        ])
        .setup(|app, _api| {
            app.manage(TabManager::new());
            Ok(())
        })
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_manager_initial_state() {
        let manager = TabManager::new();
        let tabs = manager.list_tabs();
        assert_eq!(tabs.len(), 0);
    }

    #[test]
    fn test_tab_bounds_serialization() {
        let bounds = TabBounds {
            x: 10.0,
            y: 50.0,
            width: 800.0,
            height: 600.0,
        };
        let json = serde_json::to_string(&bounds).expect("bounds serialize");
        assert!(json.contains("\"x\":10.0"));
        assert!(json.contains("\"width\":800.0"));
    }
}

