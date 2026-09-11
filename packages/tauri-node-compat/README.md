# tauri-node-compat

> Drop-in Node.js API shims (`fs`, `path`, `child_process`, `os`, `events`) for Tauri v2 and modern webview runtimes.

[![npm](https://img.shields.io/npm/v/tauri-node-compat)](https://npmjs.com/package/tauri-node-compat)

## Why?
When porting existing Electron or Node.js applications and libraries to Tauri v2, hundreds of dependencies rely on Node built-in modules (`import fs from 'node:fs'`, `import path from 'path'`, `import { spawn } from 'child_process'`).

In Tauri v2, these APIs are exposed through native Tauri plugins and APIs (`@tauri-apps/plugin-fs`, `@tauri-apps/api/path`, `@tauri-apps/plugin-shell`, etc.). `tauri-node-compat` provides 1:1 drop-in polyfill shims with standard subpath exports.

## Features
- **Drop-in Subpath Exports:**
  - `tauri-node-compat/fs` (and `tauri-node-compat/fs/promises`)
  - `tauri-node-compat/path`
  - `tauri-node-compat/child_process`
  - `tauri-node-compat/os`
  - `tauri-node-compat/events` (`EventEmitter`)
- **Dual Format:** Fully typed ESM (`.mjs`) and CommonJS (`.js`) outputs with complete TypeScript `.d.ts` declaration maps.
- **Bundler Aliasing:** Point your Vite, Webpack, or Rollup bundler configs directly to `tauri-node-compat` to migrate Electron codebases with zero refactoring.

## Installation

```bash
npm install tauri-node-compat
```

## Usage

### Direct Import
```typescript
import { promises as fs } from 'tauri-node-compat/fs';
import path from 'tauri-node-compat/path';
import { EventEmitter } from 'tauri-node-compat/events';
import os from 'tauri-node-compat/os';

// Works just like Node.js:
const appDir = path.join(os.homedir(), '.config', 'myapp');
await fs.mkdir(appDir, { recursive: true });
await fs.writeFile(path.join(appDir, 'settings.json'), JSON.stringify({ theme: 'dark' }));

const contents = await fs.readFile(path.join(appDir, 'settings.json'), 'utf-8');
console.log(contents);
```

### Vite / Bundler Aliasing (Migrate Electron projects instantly!)
In your `vite.config.ts`:

```typescript
import { defineConfig } from 'vite';

export default defineConfig({
  resolve: {
    alias: {
      'fs': 'tauri-node-compat/fs',
      'node:fs': 'tauri-node-compat/fs',
      'path': 'tauri-node-compat/path',
      'node:path': 'tauri-node-compat/path',
      'child_process': 'tauri-node-compat/child_process',
      'node:child_process': 'tauri-node-compat/child_process',
      'os': 'tauri-node-compat/os',
      'node:os': 'tauri-node-compat/os',
      'events': 'tauri-node-compat/events',
      'node:events': 'tauri-node-compat/events',
    },
  },
});
```

## License
MIT OR Apache-2.0
