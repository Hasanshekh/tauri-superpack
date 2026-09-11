# @tauri-superpack/plugin-isolate

> Pure Rust sandboxed JavaScript execution engine (`boa_engine`) for secure plugin systems in Tauri v2.

[![npm](https://img.shields.io/npm/v/@tauri-superpack/plugin-isolate)](https://npmjs.com/package/@tauri-superpack/plugin-isolate)
[![crates.io](https://img.shields.io/crates/v/tauri-plugin-isolate)](https://crates.io/crates/tauri-plugin-isolate)

## Features
- **Zero C Toolchain Dependencies:** Built with pure Rust [`boa_engine`](https://github.com/boa-dev/boa). Avoids V8/QuickJS native C/C++ compilation headaches on Windows MSVC, macOS, and Linux.
- **Air-Gapped Sandbox:** Script execution takes place strictly inside an isolated ECMAScript context with no access to Node.js, standard web APIs (`window`, `fetch`, `document`), or the OS filesystem.
- **Plugin Lifecycle Manager:** Built-in manifest validation, plugin registration, dynamic unmounting, and memory containment.
- **Deterministic Return Values:** Formats execution results and runtime evaluation errors into structured, type-safe objects.

## Installation

### Rust (`src-tauri/Cargo.toml`)
```toml
[dependencies]
tauri-plugin-isolate = "0.1"
```

Register in `src-tauri/src/main.rs`:
```rust
tauri::Builder::default()
    .plugin(tauri_plugin_isolate::init())
```

### TypeScript (`package.json`)
```bash
npm install @tauri-superpack/plugin-isolate
```

## Usage

```typescript
import {
  evalScript,
  registerPlugin,
  listPlugins,
  unloadPlugin,
} from '@tauri-superpack/plugin-isolate';

// 1. Safely evaluate untrusted JavaScript expressions
const res = await evalScript(`
  function calculateScore(items) {
    return items.reduce((acc, curr) => acc + curr.points, 0);
  }
  calculateScore([{ points: 10 }, { points: 25 }, { points: 5 }]);
`);

if (res.success) {
  console.log("Result:", res.output); // "40"
} else {
  console.error("Evaluation error:", res.error);
}

// 2. Register an untrusted third-party community plugin
await registerPlugin(
  {
    id: 'custom-linter',
    name: 'Custom Linter Plugin',
    version: '1.0.0',
    permissions: ['safe_eval'],
  },
  `const greet = () => "Hello from sandboxed plugin!"; greet();`
);

// 3. Inspect active plugins
const plugins = await listPlugins();
console.log(`Active sandboxed plugins:`, plugins);

// 4. Unload plugin
await unloadPlugin('custom-linter');
```

## License
MIT OR Apache-2.0
