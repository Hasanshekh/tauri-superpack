const COMMANDS: &[&str] = &[
    "get_server_info",
    "register_buffer",
    "release_buffer",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
