#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_uniform::init())
        .plugin(tauri_plugin_browser_view::init())
        .plugin(tauri_plugin_zerocopy::init())
        .plugin(tauri_plugin_cdp::init())
        .plugin(tauri_plugin_isolate::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
