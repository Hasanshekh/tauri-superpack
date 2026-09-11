const COMMANDS: &[&str] = &[
    "get_cdp_server_info",
    "open_devtools_window",
    "get_devtools_url",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
