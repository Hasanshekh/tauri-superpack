use crate::error::{Error, Result};
use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogMessage {
    pub level: String,
    pub message: String,
    pub timestamp: u64,
    pub target: String,
}

#[derive(Clone, Default)]
pub struct DevToolsStore {
    logs: Arc<RwLock<HashMap<String, Vec<LogMessage>>>>,
}

impl DevToolsStore {
    pub async fn push_log(&self, target: String, log: LogMessage) {
        let mut map = self.logs.write().await;
        map.entry(target).or_default().push(log);
    }

    pub async fn get_logs(&self, target: &str) -> Vec<LogMessage> {
        let map = self.logs.read().await;
        map.get(target).cloned().unwrap_or_default()
    }

    pub async fn clear_logs(&self, target: &str) {
        let mut map = self.logs.write().await;
        map.remove(target);
    }
}

#[derive(Clone)]
pub struct CdpServerState {
    pub port: u16,
    pub store: DevToolsStore,
}

const DEVTOOLS_HTML: &str = include_str!("devtools_ui.html");
const AGENT_JS: &str = include_str!("agent.js");

pub async fn start_cdp_server() -> Result<Arc<CdpServerState>> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .map_err(|e| Error::Server(e.to_string()))?;

    let port = listener
        .local_addr()
        .map_err(|e| Error::Server(e.to_string()))?
        .port();

    let state = Arc::new(CdpServerState {
        port,
        store: DevToolsStore::default(),
    });

    let app = Router::new()
        .route("/devtools/{target}", get(devtools_ui_handler))
        .route("/agent.js", get(agent_js_handler))
        .route("/api/logs/{target}", get(get_logs_handler).post(push_log_handler))
        .layer(CorsLayer::permissive())
        .with_state(state.clone());

    tokio::spawn(async move {
        if let Err(err) = axum::serve(listener, app).await {
            log::error!("CDP remote debugging server error: {err}");
        }
    });

    Ok(state)
}

async fn devtools_ui_handler(Path(target): Path<String>) -> Html<String> {
    let html = DEVTOOLS_HTML.replace("{{TARGET_LABEL}}", &target);
    Html(html)
}

async fn agent_js_handler() -> Response {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/javascript".parse().unwrap());
    (StatusCode::OK, headers, AGENT_JS).into_response()
}

async fn get_logs_handler(
    Path(target): Path<String>,
    State(state): State<Arc<CdpServerState>>,
) -> Json<Vec<LogMessage>> {
    let logs = state.store.get_logs(&target).await;
    Json(logs)
}

#[derive(Deserialize)]
pub struct NewLogPayload {
    pub level: String,
    pub message: String,
}

async fn push_log_handler(
    Path(target): Path<String>,
    State(state): State<Arc<CdpServerState>>,
    Json(payload): Json<NewLogPayload>,
) -> StatusCode {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    state
        .store
        .push_log(
            target.clone(),
            LogMessage {
                level: payload.level,
                message: payload.message,
                timestamp,
                target,
            },
        )
        .await;

    StatusCode::CREATED
}
