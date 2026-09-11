# @tauri-superpack/plugin-zerocopy

> High-speed in-memory loopback HTTP streaming IPC for Tauri v2.

[![npm](https://img.shields.io/npm/v/@tauri-superpack/plugin-zerocopy)](https://npmjs.com/package/@tauri-superpack/plugin-zerocopy)
[![crates.io](https://img.shields.io/crates/v/tauri-plugin-zerocopy)](https://crates.io/crates/tauri-plugin-zerocopy)

## Features
- **Zero JSON IPC Serialization:** Bypasses IPC bottlenecks when streaming large binaries, video/audio tracks, or 60fps canvas frames.
- **Embedded Loopback Server:** In-memory `axum` HTTP server bound to `127.0.0.1:{ephemeral_port}` with per-session token authorization.
- **Native Browser Compatibility:** Stream URLs work seamlessly with `fetch()`, `<img src="...">`, and `<video src="...">`.
- **Bi-directional Streaming:** Fast chunked uploads from webview to Rust backend.

## Installation

### Rust (`src-tauri/Cargo.toml`)
```toml
[dependencies]
tauri-plugin-zerocopy = "0.1"
```

Register in `src-tauri/src/main.rs`:
```rust
tauri::Builder::default()
    .plugin(tauri_plugin_zerocopy::init())
```

### TypeScript (`package.json`)
```bash
npm install @tauri-superpack/plugin-zerocopy
```

## Usage

```typescript
import {
  registerBuffer,
  releaseBuffer,
  uploadStream,
} from '@tauri-superpack/plugin-zerocopy';

// 1. Register a 50MB binary buffer in Rust memory
const reg = await registerBuffer(largeBinaryData, {
  mimeType: 'video/mp4',
  oneTime: true,
});

// 2. Consume directly in HTML5 media elements (hardware accelerated!)
videoElement.src = reg.url;

// 3. Fast binary upload bypassing IPC JSON serialization
await uploadStream('snapshot-buffer', canvasBlob);
```

## License
MIT OR Apache-2.0
