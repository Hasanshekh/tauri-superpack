use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use xcap::Monitor;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorResult {
    pub s_rgb_hex: String,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// Sample a single pixel color at the given global desktop screen coordinates (x, y).
pub fn sample_screen_pixel(x: i32, y: i32) -> Result<ColorResult> {
    let monitors = Monitor::all().map_err(|e| Error::Capture(e.to_string()))?;
    if monitors.is_empty() {
        return Err(Error::NoMonitor);
    }

    // Find the monitor that contains the requested (x, y) coordinate
    let target_monitor = monitors.into_iter().find(|m| {
        let mx = m.x().unwrap_or(0);
        let my = m.y().unwrap_or(0);
        let mw = m.width().unwrap_or(0) as i32;
        let mh = m.height().unwrap_or(0) as i32;
        x >= mx && x < mx + mw && y >= my && y < my + mh
    });

    let monitor = match target_monitor {
        Some(m) => m,
        None => {
            // Default to primary or first monitor if coordinate not strictly inside
            let fallback_monitors = Monitor::all().map_err(|e| Error::Capture(e.to_string()))?;
            fallback_monitors.into_iter().next().ok_or(Error::NoMonitor)?
        }
    };

    let mx = monitor.x().unwrap_or(0);
    let my = monitor.y().unwrap_or(0);
    let mw = monitor.width().unwrap_or(0) as i32;
    let mh = monitor.height().unwrap_or(0) as i32;

    let local_x = (x - mx).clamp(0, mw - 1) as u32;
    let local_y = (y - my).clamp(0, mh - 1) as u32;

    let image = monitor
        .capture_image()
        .map_err(|e| Error::Capture(e.to_string()))?;

    let pixel = image.get_pixel(local_x, local_y);
    let r = pixel[0];
    let g = pixel[1];
    let b = pixel[2];
    let a = pixel[3];
    let s_rgb_hex = format!("#{r:02x}{g:02x}{b:02x}");

    Ok(ColorResult {
        s_rgb_hex,
        r,
        g,
        b,
        a,
    })
}
