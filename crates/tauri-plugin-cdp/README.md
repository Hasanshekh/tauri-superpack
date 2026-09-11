# @tauri-superpack/plugin-cdp

> Universal Chrome DevTools inspector window & live console bridge for Tauri v2.

[![npm](https://img.shields.io/npm/v/@tauri-superpack/plugin-cdp)](https://npmjs.com/package/@tauri-superpack/plugin-cdp)
[![crates.io](https://img.shields.io/crates/v/tauri-plugin-cdp)](https://crates.io/crates/tauri-plugin-cdp)

## Features
- **Cross-Platform DevTools:** Replaces OS-dependent devtools (WebView2 F12 on Windows, WebKit Web Inspector on macOS/Linux) with a unified, standalone Chrome DevTools dashboard.
- **In-Memory Inspector Host:** Embedded micro-HTTP server providing instant inspection interfaces across Windows, macOS, and Linux.
- **Live Console & Exception Bridge:** Injects an auto-syncing agent into webview contexts, streaming `console.log`, `warn`, `error`, uncaught exceptions, and unhandled promise rejections directly to the inspector.
- **Multi-Target Inspection:** Inspect main windows, child tabs, or headless background webviews independently.

## Installation

### Rust (`src-tauri/Cargo.toml`)
```toml
[dependencies]
tauri-plugin-cdp = "0.1"
```

Register in `src-tauri/src/main.rs`:
```rust
tauri::Builder::default()
    .plugin(tauri_plugin_cdp::init())
```

### TypeScript (`package.json`)
```bash
npm install @tauri-superpack/plugin-cdp
```

## Usage

```typescript
import {
  getCdpServerInfo,
  getDevtoolsUrl,
  openDevtoolsWindow,
} from '@tauri-superpack/plugin-cdp';

// 1. Check local inspector server metadata
const info = await getCdpServerInfo();
console.log(`CDP Bridge listening on port ${info.port}`);

// 2. Launch a standalone DevTools inspector window for the main app
await openDevtoolsWindow('main');

// 3. Or obtain the raw inspection URL to embed in an iframe or tab
const devtoolsUrl = await getDevtoolsUrl('tab-1');
```

## License
MIT OR Apache-2.0
