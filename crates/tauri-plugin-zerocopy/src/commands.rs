use crate::error::Result;
use crate::server::AppState;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
    pub port: u16,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterBufferResult {
    pub id: String,
    pub url: String,
    pub size: usize,
}

#[tauri::command]
pub async fn get_server_info(state: State<'_, Arc<AppState>>) -> Result<ServerInfo> {
    Ok(ServerInfo {
        port: state.port,
        token: state.auth_token.clone(),
    })
}

#[tauri::command]
pub async fn register_buffer(
    state: State<'_, Arc<AppState>>,
    id: Option<String>,
    data: Vec<u8>,
    mime_type: Option<String>,
    one_time: Option<bool>,
) -> Result<RegisterBufferResult> {
    let key = id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let mime = mime_type.unwrap_or_else(|| "application/octet-stream".to_string());
    let size = data.len();

    state
        .store
        .insert(key.clone(), Bytes::from(data), mime, one_time.unwrap_or(false))
        .await;

    let url = format!(
        "http://127.0.0.1:{}/blob/{}?token={}",
        state.port, key, state.auth_token
    );

    Ok(RegisterBufferResult { id: key, url, size })
}

#[tauri::command]
pub async fn release_buffer(state: State<'_, Arc<AppState>>, id: String) -> Result<bool> {
    Ok(state.store.remove(&id).await)
}
