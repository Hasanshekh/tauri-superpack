# @tauri-superpack/plugin-uniform

> Cross-platform webview consistency and native OS polyfills for Tauri v2.

[![npm](https://img.shields.io/npm/v/@tauri-superpack/plugin-uniform)](https://npmjs.com/package/@tauri-superpack/plugin-uniform)
[![crates.io](https://img.shields.io/crates/v/tauri-plugin-uniform)](https://crates.io/crates/tauri-plugin-uniform)

## Features
- **EyeDropper Polyfill:** Auto-injects `window.EyeDropper` into WebKit on macOS and Linux, using native screen pixel sampling (`xcap`).
- **Screen & Rectangle Capture:** High-performance desktop screen capture encoded to Base64 PNG data URLs.
- **Native File Dialogs:** Asynchronous open/save dialogues via `rfd` (replacing missing WebKit File System Access APIs).
- **CSS Normalization:** Standardizes scrollbar appearance and styling quirks across Chromium and WebKit.

## Installation

### Rust (`src-tauri/Cargo.toml`)
```toml
[dependencies]
tauri-plugin-uniform = "0.1"
```

Register in `src-tauri/src/main.rs`:
```rust
tauri::Builder::default()
    .plugin(tauri_plugin_uniform::init())
```

### TypeScript (`package.json`)
```bash
npm install @tauri-superpack/plugin-uniform
```

## Usage

```typescript
import {
  ensurePolyfills,
  pickColor,
  captureScreen,
  showOpenFilePicker,
  showSaveFilePicker,
} from '@tauri-superpack/plugin-uniform';

// Initialize Web standards polyfills
ensurePolyfills();

// 1. Color Picker (works across all platforms!)
const color = await pickColor();
console.log(color.sRgbHex); // '#3b82f6'

// 2. Desktop Screenshot
const screenshot = await captureScreen({ width: 800, height: 600 });
console.log(screenshot.dataUrl);

// 3. Native File Picker
const files = await showOpenFilePicker({ title: 'Select Image' });
```

## License
MIT OR Apache-2.0
