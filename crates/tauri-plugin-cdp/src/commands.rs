use crate::error::{Error, Result};
use crate::server::CdpServerState;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{AppHandle, Manager, Runtime, State, WebviewUrl, WebviewWindowBuilder};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CdpServerInfo {
    pub port: u16,
    pub devtools_base_url: String,
}

#[tauri::command]
pub async fn get_cdp_server_info(state: State<'_, Arc<CdpServerState>>) -> Result<CdpServerInfo> {
    Ok(CdpServerInfo {
        port: state.port,
        devtools_base_url: format!("http://127.0.0.1:{}/devtools", state.port),
    })
}

#[tauri::command]
pub async fn get_devtools_url(
    state: State<'_, Arc<CdpServerState>>,
    target: String,
) -> Result<String> {
    Ok(format!(
        "http://127.0.0.1:{}/devtools/{}",
        state.port,
        urlencoding::encode(&target)
    ))
}

#[tauri::command]
pub async fn open_devtools_window<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, Arc<CdpServerState>>,
    target: String,
) -> Result<()> {
    let devtools_label = format!("devtools-{}", target);

    // If window already open, focus it
    if let Some(existing) = app.get_webview_window(&devtools_label) {
        let _ = existing.show();
        let _ = existing.set_focus();
        return Ok(());
    }

    let devtools_url_str = format!(
        "http://127.0.0.1:{}/devtools/{}",
        state.port,
        urlencoding::encode(&target)
    );

    let parsed_url = devtools_url_str
        .parse::<url::Url>()
        .map_err(|e| Error::Server(e.to_string()))?;

    WebviewWindowBuilder::new(
        &app,
        &devtools_label,
        WebviewUrl::External(parsed_url),
    )
    .title(format!("DevTools — {target}"))
    .inner_size(800.0, 600.0)
    .build()
    .map_err(|e| Error::Window(e.to_string()))?;

    Ok(())
}
