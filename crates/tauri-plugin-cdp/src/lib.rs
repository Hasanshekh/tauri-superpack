use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub mod commands;
pub mod error;
pub mod server;

pub use error::{Error, Result};
pub use server::CdpServerState;

/// Initialize the Universal Chrome DevTools Protocol & Remote Debugging bridge for Tauri v2.
/// Runs a local DevTools dashboard with Console, Elements, Network, and Storage inspectors,
/// providing identical DevTools capabilities across Windows, macOS, and Linux.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("cdp")
        .invoke_handler(tauri::generate_handler![
            commands::get_cdp_server_info,
            commands::open_devtools_window,
            commands::get_devtools_url,
        ])
        .setup(|app, _api| {
            let state = tauri::async_runtime::block_on(server::start_cdp_server())
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            app.manage(state);
            Ok(())
        })
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use server::DevToolsStore;

    #[tokio::test]
    async fn test_devtools_store_logging() {
        let store = DevToolsStore::default();
        store
            .push_log(
                "main".to_string(),
                server::LogMessage {
                    level: "info".to_string(),
                    message: "Test message from CDP probe".to_string(),
                    timestamp: 123456789,
                    target: "main".to_string(),
                },
            )
            .await;

        let logs = store.get_logs("main").await;
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].message, "Test message from CDP probe");

        store.clear_logs("main").await;
        assert_eq!(store.get_logs("main").await.len(), 0);
    }
}
