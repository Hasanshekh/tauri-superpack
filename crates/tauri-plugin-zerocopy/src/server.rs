use crate::error::{Error, Result};
use crate::store::BufferStore;
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use bytes::Bytes;
use serde::Deserialize;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
pub struct AppState {
    pub store: BufferStore,
    pub auth_token: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct AuthQuery {
    pub token: Option<String>,
}

pub async fn start_server(store: BufferStore, auth_token: String) -> Result<Arc<AppState>> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .map_err(|e| Error::Server(e.to_string()))?;

    let port = listener
        .local_addr()
        .map_err(|e| Error::Server(e.to_string()))?
        .port();

    let state = Arc::new(AppState {
        store,
        auth_token,
        port,
    });

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/blob/{id}", get(get_blob_handler))
        .route("/upload/{id}", post(upload_blob_handler))
        .layer(CorsLayer::permissive())
        .with_state(state.clone());

    tokio::spawn(async move {
        if let Err(err) = axum::serve(listener, app).await {
            log::error!("ZeroCopy local HTTP streaming server error: {err}");
        }
    });

    Ok(state)
}

async fn health_handler() -> &'static str {
    "tauri-zerocopy-active"
}

async fn get_blob_handler(
    Path(id): Path<String>,
    Query(query): Query<AuthQuery>,
    State(state): State<Arc<AppState>>,
) -> Response {
    // Validate security token
    if query.token.as_deref() != Some(&state.auth_token) {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }

    if let Some(entry) = state.store.get(&id).await {
        let mut headers = HeaderMap::new();
        if let Ok(content_type) = entry.mime_type.parse() {
            headers.insert(header::CONTENT_TYPE, content_type);
        }
        headers.insert(header::CONTENT_LENGTH, entry.data.len().into());
        headers.insert(
            header::CACHE_CONTROL,
            "no-cache, no-store, must-revalidate".parse().unwrap(),
        );

        (StatusCode::OK, headers, Body::from(entry.data)).into_response()
    } else {
        (StatusCode::NOT_FOUND, "Blob not found").into_response()
    }
}

async fn upload_blob_handler(
    Path(id): Path<String>,
    Query(query): Query<AuthQuery>,
    State(state): State<Arc<AppState>>,
    body: Bytes,
) -> Response {
    if query.token.as_deref() != Some(&state.auth_token) {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }

    let len = body.len();
    state
        .store
        .insert(id.clone(), body, "application/octet-stream".to_string(), false)
        .await;

    (StatusCode::CREATED, format!("{{\"id\":\"{id}\",\"size\":{len}}}")).into_response()
}
