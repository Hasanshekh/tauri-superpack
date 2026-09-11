# @tauri-superpack/plugin-browser-view

> High-level multi-webview tab orchestration & session cookie isolation for Tauri v2.

[![npm](https://img.shields.io/npm/v/@tauri-superpack/plugin-browser-view)](https://npmjs.com/package/@tauri-superpack/plugin-browser-view)
[![crates.io](https://img.shields.io/crates/v/tauri-plugin-browser-view)](https://crates.io/crates/tauri-plugin-browser-view)

## Features
- **Multi-Webview Tabs:** Attach multiple child webviews to a single parent window with turnkey lifecycle management (`createTab`, `switchTab`, `closeTab`).
- **Session Partitioning:** Implements Electron's `partition: 'persist:xyz'` equivalent via dynamic `data_directory` isolation. Cookies, caches, and storage never leak between tabs.
- **Responsive Positioning:** Dynamically resize, reposition, or simulate mobile/tablet device viewports.
- **Cross-Tab Script Injection:** Evaluate scripts inside child tabs on demand.

## Installation

### Rust (`src-tauri/Cargo.toml`)
```toml
[dependencies]
tauri-plugin-browser-view = "0.1"
```

Register in `src-tauri/src/main.rs`:
```rust
tauri::Builder::default()
    .plugin(tauri_plugin_browser_view::init())
```

### TypeScript (`package.json`)
```bash
npm install @tauri-superpack/plugin-browser-view
```

## Usage

```typescript
import {
  createTab,
  switchTab,
  closeTab,
  setTabBounds,
  listTabs,
} from '@tauri-superpack/plugin-browser-view';

// 1. Create an isolated session tab (independent cookies & login)
await createTab({
  label: 'client-session-1',
  url: 'https://github.com',
  bounds: { x: 0, y: 40, width: 1000, height: 700 },
  partition: 'isolated-partition-1',
});

// 2. Switch active tab
await switchTab('client-session-1');

// 3. Resize tab for device simulation
await setTabBounds('client-session-1', { x: 50, y: 50, width: 375, height: 667 });

// 4. Close tab
await closeTab('client-session-1');
```

## License
MIT OR Apache-2.0
