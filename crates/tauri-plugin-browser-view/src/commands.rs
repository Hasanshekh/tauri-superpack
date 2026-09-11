use crate::error::Result;
use crate::manager::TabManager;
use crate::models::{CreateTabOptions, TabBounds, TabInfo};
use tauri::{AppHandle, Runtime, State};

#[tauri::command]
pub async fn create_tab<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, TabManager>,
    options: CreateTabOptions,
) -> Result<TabInfo> {
    state.create_tab(&app, options)
}

#[tauri::command]
pub async fn close_tab<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, TabManager>,
    label: String,
) -> Result<()> {
    state.close_tab(&app, &label)
}

#[tauri::command]
pub async fn set_tab_bounds<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, TabManager>,
    label: String,
    bounds: TabBounds,
) -> Result<()> {
    state.set_tab_bounds(&app, &label, bounds)
}

#[tauri::command]
pub async fn switch_tab<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, TabManager>,
    label: String,
) -> Result<()> {
    state.switch_tab(&app, &label)
}

#[tauri::command]
pub async fn navigate_tab<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, TabManager>,
    label: String,
    url: String,
) -> Result<()> {
    state.navigate_tab(&app, &label, &url)
}

#[tauri::command]
pub async fn eval_script_in_tab<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, TabManager>,
    label: String,
    script: String,
) -> Result<()> {
    state.eval_script(&app, &label, &script)
}

#[tauri::command]
pub async fn list_tabs(state: State<'_, TabManager>) -> Result<Vec<TabInfo>> {
    Ok(state.list_tabs())
}
