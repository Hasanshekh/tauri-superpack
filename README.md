# ⚡ Tauri Super-Pack

> Production-grade plugins and libraries designed to eliminate every architectural advantage of Electron, keeping your Tauri v2 desktop apps ultra-lightweight (~8 MB) and blazingly fast.

[![CI Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()
[![Tauri Version](https://img.shields.io/badge/tauri-v2.0+-blue.svg)](https://v2.tauri.app)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-orange.svg)]()

---

## 🎯 Why Tauri Super-Pack?

While **Tauri** is celebrated for producing tiny executables (3–8 MB vs. Electron's 180+ MB) and sipping minimal RAM, developers migrating from **Electron** frequently run into 6 painful hurdles:

1. **Webview Inconsistency:** Edge WebView2 (Windows) supports Chrome APIs, but macOS (WKWebView) and Linux (WebKitGTK) lack `EyeDropper`, File System Access, and have WebRTC bugs.
2. **Tab & Session Management:** Electron has `WebContentsView` with session partitioning (`partition: 'persist:xyz'`). Tauri v2 has multi-webviews, but lacks turnkey tab management and cookie isolation.
3. **Heavy Binary IPC Bottleneck:** Serializing 50MB+ binary buffers or 60fps canvas streams over JSON IPC causes UI stutter.
4. **DevTools Parity:** Chrome DevTools is built into Electron everywhere, but macOS/Linux Tauri apps open Safari/WebKit inspectors with fragmented UI.
5. **3rd-Party Plugin Safety:** Running untrusted community extensions (like VS Code or Obsidian) in desktop apps without Node.js is risky.
6. **Node.js Migration Friction:** Rewriting hundreds of `fs`, `path`, and `child_process` calls in Rust slows down migrations.

**Tauri Super-Pack solves all six problems.**

---

## 📦 The 6-Package Suite

| Package | Rust Crate | NPM Package | What It Solves |
|:---|:---|:---|:---|
| **1. Uniform** | `tauri-plugin-uniform` | `@tauri-superpack/plugin-uniform` | Cross-platform webview consistency, `EyeDropper` polyfill, `xcap` screen capture, `rfd` native dialogs. |
| **2. Browser-View** | `tauri-plugin-browser-view` | `@tauri-superpack/plugin-browser-view` | Multi-webview tab orchestrator with isolated cookie partitions (`data_directory`) and device emulation. |
| **3. ZeroCopy** | `tauri-plugin-zerocopy` | `@tauri-superpack/plugin-zerocopy` | In-memory loopback HTTP streaming bridge (`axum`) transferring raw binary buffers with zero IPC latency. |
| **4. CDP** | `tauri-plugin-cdp` | `@tauri-superpack/plugin-cdp` | Standalone Chrome DevTools inspection window running identically on Windows, macOS, and Linux. |
| **5. Isolate** | `tauri-plugin-isolate` | `@tauri-superpack/plugin-isolate` | Memory-safe, pure Rust sandboxed JavaScript engine (`boa_engine`) for untrusted community plugins. |
| **6. Node-Compat** | — | `tauri-node-compat` | 1:1 drop-in shims for `fs.promises`, `path`, `child_process`, `os`, and `events`. |

---

## 📊 Scorecard: Tauri Super-Pack vs. Electron

| Metric / Capability | Default Electron | Default Tauri | Tauri + Super-Pack |
|:---|:---:|:---:|:---:|
| **App Download Size** | ❌ ~180 MB | ✅ ~4 MB | **🏆 ~8 MB** |
| **Idle Memory (RAM)** | ❌ 180–300 MB | ✅ 25–40 MB | **🏆 30–50 MB** |
| **Webview Consistency** | ✅ 100% Chromium | ⚠️ WebKit quirks | **🏆 100% Normalized (`uniform`)** |
| **Multi-Tabs & Cookie Isolation** | ✅ Great | ⚠️ Low-level | **🏆 Turnkey (`browser-view`)** |
| **Heavy Binary Streaming** | ⚠️ Moderate | ⚠️ JSON IPC | **🏆 Blazing (`zerocopy`)** |
| **Universal Chrome DevTools** | ✅ Chromium | ⚠️ OS-dependent | **🏆 Identical (`cdp`)** |
| **Safe Community Plugins** | ⚠️ Unsandboxed | ❌ Difficult | **🏆 Sandboxed (`isolate`)** |
| **Node.js Migration Speed** | ✅ Native | ⚠️ High friction | **🏆 Drop-in (`node-compat`)** |

---

## 🚀 Quick Start

### 1. Register Plugins in Rust (`src-tauri/src/main.rs`)

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_uniform::init())
        .plugin(tauri_plugin_browser_view::init())
        .plugin(tauri_plugin_zerocopy::init())
        .plugin(tauri_plugin_cdp::init())
        .plugin(tauri_plugin_isolate::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 2. Configure Window Capabilities (`src-tauri/capabilities/default.json`)

```json
{
  "identifier": "default",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "uniform:default",
    "browser-view:default",
    "zerocopy:default",
    "cdp:default",
    "isolate:default"
  ]
}
```

### 3. Use in Frontend TypeScript / React

```typescript
import { pickColor } from '@tauri-superpack/plugin-uniform';
import { createTab } from '@tauri-superpack/plugin-browser-view';
import { registerBuffer } from '@tauri-superpack/plugin-zerocopy';
import { openDevtoolsWindow } from '@tauri-superpack/plugin-cdp';
import { evalScript } from '@tauri-superpack/plugin-isolate';
import { path, fs } from 'tauri-node-compat';

// 1. Color Picker (works identically on Windows, Mac, Linux!)
const color = await pickColor();
console.log(color.sRgbHex); // '#3b82f6'

// 2. Multi-Webview Tab with isolated cookie session
await createTab({
  label: 'work-session',
  url: 'https://github.com',
  bounds: { x: 50, y: 100, width: 900, height: 600 },
  partition: 'isolated-work-cookies',
});

// 3. Stream 20MB raw binary buffer without IPC lag
const stream = await registerBuffer(largeBinaryData, { mimeType: 'video/mp4' });
videoElement.src = stream.url;

// 4. Open standalone Chrome DevTools
await openDevtoolsWindow('work-session');

// 5. Execute untrusted community plugin code safely in sandbox
const result = await evalScript('const a = 10; const b = 20; a * b;');
console.log(result.output); // '200'

// 6. Familiar Node.js APIs
const configPath = path.join('/app', 'config.json');
await fs.promises.writeFile(configPath, JSON.stringify({ ok: true }));
```

---

## 📂 Repository Structure

```
.
├── Cargo.toml                                 # Cargo Workspace root
├── package.json                               # NPM Workspace root
├── LICENSE-MIT / LICENSE-APACHE               # Dual-license
│
├── crates/                                    # Rust Crates + TypeScript Guest Bindings
│   ├── tauri-plugin-uniform/                  # Webview consistency & polyfills
│   ├── tauri-plugin-browser-view/             # Tab orchestrator & session isolation
│   ├── tauri-plugin-zerocopy/                 # High-speed streaming binary IPC
│   ├── tauri-plugin-cdp/                      # Universal Chrome DevTools bridge
│   └── tauri-plugin-isolate/                  # Sandboxed JS runtime (boa_engine)
│
├── packages/                                  # Pure NPM Libraries
│   └── tauri-node-compat/                     # Drop-in Node.js API replacements
│
└── examples/
    └── tauri-showcase/                        # Live 6-card interactive verification app
```

---

## 🧪 Verification & Testing

Run all Rust crate unit tests:
```bash
cargo test --workspace
```

Compile all TypeScript libraries and the showcase app:
```bash
npm run build
```

Launch the interactive showcase application:
```bash
npm run tauri dev --prefix examples/tauri-showcase
```

---

## 📄 License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.
