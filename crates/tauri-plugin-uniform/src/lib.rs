use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

pub mod commands;
pub mod error;
pub mod eyedropper;
pub mod file_dialog;
pub mod screen_capture;

pub use error::{Error, Result};
pub use eyedropper::ColorResult;
pub use screen_capture::{CaptureOptions, CaptureResult};

const PREFLIGHT_JS: &str = include_str!("injected_preflight.js");

/// Initialize the Uniform compatibility plugin for Tauri v2.
/// Automatically injects web standards polyfills (EyeDropper, CSS normalization)
/// and exposes native cross-platform screen capture and file picker commands.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("uniform")
        .invoke_handler(tauri::generate_handler![
            commands::pick_color,
            commands::capture_screen,
            commands::show_open_file_picker,
            commands::show_save_file_picker,
        ])
        .js_init_script(PREFLIGHT_JS.to_string())
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_result_formatting() {
        let res = ColorResult {
            s_rgb_hex: "#3b82f6".to_string(),
            r: 59,
            g: 130,
            b: 246,
            a: 255,
        };
        let json = serde_json::to_string(&res).expect("serialization succeeds");
        assert!(json.contains("\"sRgbHex\":\"#3b82f6\""));
        assert!(json.contains("\"r\":59"));
    }

    #[test]
    fn test_preflight_script_content() {
        assert!(PREFLIGHT_JS.contains("EyeDropper"));
        assert!(PREFLIGHT_JS.contains("tauri-uniform-preflight"));
    }
}

