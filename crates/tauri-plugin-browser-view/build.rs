const COMMANDS: &[&str] = &[
    "create_tab",
    "close_tab",
    "set_tab_bounds",
    "switch_tab",
    "navigate_tab",
    "eval_script_in_tab",
    "list_tabs",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
