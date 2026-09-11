use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub mod commands;
pub mod error;
pub mod server;
pub mod store;

pub use error::{Error, Result};
pub use server::AppState;
pub use store::BufferStore;

/// Initialize the ZeroCopy High-Speed Binary & Streaming IPC plugin for Tauri v2.
/// Spins up an embedded in-memory loopback HTTP server with token authentication,
/// allowing gigabytes-per-second data transfers and media streaming with zero IPC serialization overhead.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("zerocopy")
        .invoke_handler(tauri::generate_handler![
            commands::get_server_info,
            commands::register_buffer,
            commands::release_buffer,
        ])
        .setup(|app, _api| {
            let store = BufferStore::new();
            let auth_token = uuid::Uuid::new_v4().to_string();
            let state = tauri::async_runtime::block_on(server::start_server(store, auth_token))
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            app.manage(state);
            Ok(())
        })
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    #[tokio::test]
    async fn test_buffer_store_operations() {
        let store = BufferStore::new();
        let test_data = Bytes::from_static(b"Hello ZeroCopy Buffer");
        store
            .insert("test-key".to_string(), test_data.clone(), "text/plain".to_string(), false)
            .await;

        let retrieved = store.get("test-key").await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().data, test_data);

        let removed = store.remove("test-key").await;
        assert!(removed);
        assert!(store.get("test-key").await.is_none());
    }
}
