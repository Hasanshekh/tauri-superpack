use std::sync::Arc;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub mod commands;
pub mod error;
pub mod sandbox;

pub use error::{Error, Result};
pub use sandbox::{EvalResult, PluginManifest, SandboxManager};

/// Initialize the Isolate Sandboxed JavaScript Plugin Engine for Tauri v2.
/// Allows executing untrusted 3rd-party community plugins in secure, memory-isolated sandboxes
/// without Node.js or OS vulnerabilities.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("isolate")
        .invoke_handler(tauri::generate_handler![
            commands::eval_script,
            commands::register_plugin,
            commands::list_plugins,
            commands::unload_plugin,
        ])
        .setup(|app, _api| {
            let manager = Arc::new(SandboxManager::new());
            app.manage(manager);
            Ok(())
        })
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_eval() {
        let sandbox = SandboxManager::new();
        let res = sandbox.eval_code(None, "const x = 10; const y = 20; x + y;");
        assert!(res.success);
        assert_eq!(res.output, "30");
    }

    #[test]
    fn test_sandbox_error_handling() {
        let sandbox = SandboxManager::new();
        let res = sandbox.eval_code(None, "throw new Error('sandbox safety test');");
        assert!(!res.success);
        assert!(res.error.is_some());
    }
}
