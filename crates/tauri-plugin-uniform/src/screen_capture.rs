use crate::error::{Error, Result};
use image::{DynamicImage, ImageFormat};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use xcap::Monitor;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptureOptions {
    pub monitor_index: Option<usize>,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptureResult {
    pub data_url: String,
    pub width: u32,
    pub height: u32,
}

/// Capture full screen or cropped rect, returning a base64 image data URL.
pub fn capture_screen_area(options: Option<CaptureOptions>) -> Result<CaptureResult> {
    let monitors = Monitor::all().map_err(|e| Error::Capture(e.to_string()))?;
    if monitors.is_empty() {
        return Err(Error::NoMonitor);
    }

    let opts = options.unwrap_or(CaptureOptions {
        monitor_index: None,
        x: None,
        y: None,
        width: None,
        height: None,
    });

    let monitor_idx = opts.monitor_index.unwrap_or(0);
    let monitor = monitors
        .into_iter()
        .nth(monitor_idx)
        .ok_or(Error::NoMonitor)?;

    let rgba_image = monitor
        .capture_image()
        .map_err(|e| Error::Capture(e.to_string()))?;

    let mut dyn_image = DynamicImage::ImageRgba8(rgba_image);

    // Optional crop if bounds specified
    if let (Some(x), Some(y), Some(w), Some(h)) = (opts.x, opts.y, opts.width, opts.height) {
        let mw = dyn_image.width();
        let mh = dyn_image.height();
        let safe_x = (x.max(0) as u32).min(mw);
        let safe_y = (y.max(0) as u32).min(mh);
        let safe_w = w.min(mw.saturating_sub(safe_x));
        let safe_h = h.min(mh.saturating_sub(safe_y));

        if safe_w > 0 && safe_h > 0 {
            dyn_image = dyn_image.crop_imm(safe_x, safe_y, safe_w, safe_h);
        }
    }

    let final_w = dyn_image.width();
    let final_h = dyn_image.height();

    // Encode to PNG in memory
    let mut bytes = Vec::new();
    dyn_image
        .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
        .map_err(|e| Error::Capture(e.to_string()))?;

    // Fast base64 encoding without extra dependency
    const BASE64_ALPHABET: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut base64_str = String::with_capacity(bytes.len() * 4 / 3 + 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = chunk.get(1).copied().unwrap_or(0);
        let b2 = chunk.get(2).copied().unwrap_or(0);

        let n = ((b0 as u32) << 16) | ((b1 as u32) << 8) | (b2 as u32);
        base64_str.push(BASE64_ALPHABET[((n >> 18) & 63) as usize] as char);
        base64_str.push(BASE64_ALPHABET[((n >> 12) & 63) as usize] as char);
        if chunk.len() > 1 {
            base64_str.push(BASE64_ALPHABET[((n >> 6) & 63) as usize] as char);
        } else {
            base64_str.push('=');
        }
        if chunk.len() > 2 {
            base64_str.push(BASE64_ALPHABET[(n & 63) as usize] as char);
        } else {
            base64_str.push('=');
        }
    }

    let data_url = format!("data:image/png;base64,{base64_str}");

    Ok(CaptureResult {
        data_url,
        width: final_w,
        height: final_h,
    })
}
