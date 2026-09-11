const COMMANDS: &[&str] = &[
    "pick_color",
    "capture_screen",
    "show_open_file_picker",
    "show_save_file_picker",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
