const COMMANDS: &[&str] = &[
    "eval_script",
    "register_plugin",
    "list_plugins",
    "unload_plugin",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
