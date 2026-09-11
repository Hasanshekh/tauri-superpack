use crate::error::Result;
use crate::eyedropper::{sample_screen_pixel, ColorResult};
use crate::file_dialog::{
    open_file, save_file, OpenDialogOptions, SaveDialogOptions,
};
use crate::screen_capture::{capture_screen_area, CaptureOptions, CaptureResult};

#[tauri::command]
pub async fn pick_color(x: Option<i32>, y: Option<i32>) -> Result<ColorResult> {
    // If specific coordinate is provided, sample directly; otherwise sample center or fallback
    let target_x = x.unwrap_or(0);
    let target_y = y.unwrap_or(0);
    sample_screen_pixel(target_x, target_y)
}

#[tauri::command]
pub async fn capture_screen(options: Option<CaptureOptions>) -> Result<CaptureResult> {
    capture_screen_area(options)
}

#[tauri::command]
pub async fn show_open_file_picker(options: Option<OpenDialogOptions>) -> Result<Vec<String>> {
    open_file(options).await
}

#[tauri::command]
pub async fn show_save_file_picker(options: Option<SaveDialogOptions>) -> Result<String> {
    save_file(options).await
}
