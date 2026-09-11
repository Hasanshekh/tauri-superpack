use crate::error::Result;
use crate::sandbox::{EvalResult, PluginManifest, SandboxManager};
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn eval_script(
    state: State<'_, Arc<SandboxManager>>,
    plugin_id: Option<String>,
    code: String,
) -> Result<EvalResult> {
    Ok(state.eval_code(plugin_id.as_deref(), &code))
}

#[tauri::command]
pub async fn register_plugin(
    state: State<'_, Arc<SandboxManager>>,
    manifest: PluginManifest,
    code: String,
) -> Result<EvalResult> {
    state.register_plugin(manifest, code)
}

#[tauri::command]
pub async fn list_plugins(state: State<'_, Arc<SandboxManager>>) -> Result<Vec<PluginManifest>> {
    Ok(state.list_plugins())
}

#[tauri::command]
pub async fn unload_plugin(state: State<'_, Arc<SandboxManager>>, id: String) -> Result<()> {
    state.unload_plugin(&id)
}
