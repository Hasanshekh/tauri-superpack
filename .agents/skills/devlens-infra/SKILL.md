---
name: devlens-infra
description: >-
  Build pipeline, Manifest V3 configuration, testing strategy, security,
  performance guidelines, and publishing workflow for the DevLens extension.
---

# DevLens Infrastructure Skill

This document covers the build pipeline, Manifest V3 configuration, testing strategy, security and privacy guidelines, performance rules, and publishing workflows for the DevLens extension.

## 1. Manifest V3 Configuration

The DevLens extension requires a carefully configured `manifest.json` for Manifest V3 compliance. Below is the complete manifest configuration:

```json
{
  "manifest_version": 3,
  "name": "DevLens",
  "version": "1.0.0",
  "description": "See through any website — inspect, extract, clone, debug, audit.",
  "permissions": [
    "activeTab",
    "storage",
    "scripting",
    "sidePanel",
    "contextMenus",
    "cookies",
    "downloads"
  ],
  "optional_permissions": [
    "webRequest",
    "browsingData",
    "debugger"
  ],
  "host_permissions": ["<all_urls>"],
  "background": {
    "service_worker": "src/extension/worker/index.ts",
    "type": "module"
  },
  "content_scripts": [{
    "matches": ["<all_urls>"],
    "js": ["src/extension/content/index.ts"],
    "css": ["src/styles/overlay.css"],
    "run_at": "document_start"
  }],
  "side_panel": {
    "default_path": "sidebar.html"
  },
  "action": {
    "default_popup": "popup.html",
    "default_icon": {
      "16": "icons/icon-16.png",
      "32": "icons/icon-32.png",
      "48": "icons/icon-48.png",
      "128": "icons/icon-128.png"
    }
  },
  "icons": {
    "16": "icons/icon-16.png",
    "32": "icons/icon-32.png",
    "48": "icons/icon-48.png",
    "128": "icons/icon-128.png"
  },
  "content_security_policy": {
    "extension_pages": "script-src 'self'; object-src 'self'"
  }
}
```

### Permissions Explanation
- **`activeTab`**: Allows temporary access to the currently active tab when the user invokes the extension. Essential for injecting content scripts without broad upfront warnings.
- **`storage`**: Used to persist user preferences, customized command palettes, and module settings locally via `chrome.storage.local`.
- **`scripting`**: Required to inject modules and styles dynamically into the active tab.
- **`sidePanel`**: Enables the DevLens React application to run in the browser's side panel, persisting across tabs.
- **`contextMenus`**: Allows adding right-click menu items to quickly invoke specific DevLens modules on selected elements.
- **`cookies`**: Needed for specific modules (e.g., storage/cookie inspector) to read/modify cookies.
- **`downloads`**: Required for exporting assets (images, generated ZIPs, code snippets) to the local filesystem.
- **`host_permissions`**: `["<all_urls>"]` is necessary since DevLens needs to inspect and interact with any website the user visits.

### Optional Permissions
Requested on-demand to adhere to the principle of least privilege:
- **`webRequest`**: Requested when using advanced network interception features.
- **`browsingData`**: Requested by storage clearing utilities.
- **`debugger`**: Requested only for advanced layout/network inspection using the Chrome DevTools Protocol.

## 2. Build Pipeline

The project uses a modern, fast ESM-native pipeline powered by Vite.

- **Vite** as bundler (fast, ESM-native)
- **CRXJS Vite Plugin** for Chrome extension support (HMR, manifest handling)
- Code splitting is employed: each module in `src/core/modules/` is a dynamic import chunk.
- Output structure in `dist/`:
  - `sidebar.html` (React side panel app)
  - `popup.html` (Minimal popup UI)
  - `manifest.json` (Processed by CRXJS)
  - `assets/` (Fonts, split JS chunks, CSS)
  - `content.js` (Single bundle, minimal)
  - `worker.js` (Service worker)
- Dev mode: CRXJS enables Hot Module Replacement (HMR) for the sidebar and auto-reloads content/worker scripts.
- Production: Minified, tree-shaken, source maps stripped.

### `vite.config.ts` Example

```typescript
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { crx } from '@crxjs/vite-plugin';
import manifest from './manifest.json';
import path from 'path';

export default defineConfig({
  plugins: [
    react(),
    crx({ manifest }),
  ],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  build: {
    target: 'esnext',
    outDir: 'dist',
    emptyOutDir: true,
    rollupOptions: {
      input: {
        sidebar: path.resolve(__dirname, 'sidebar.html'),
        popup: path.resolve(__dirname, 'popup.html'),
      },
      output: {
        manualChunks(id) {
          if (id.includes('node_modules')) {
            return 'vendor';
          }
          if (id.includes('src/core/modules/')) {
            // Split core modules into lazy-loaded chunks
            const parts = id.split('src/core/modules/')[1].split('/');
            return `module-${parts[0]}`;
          }
        },
      },
    },
  },
});
```

## 3. Package Dependencies

Required npm packages in `package.json`:

### Dependencies
- `react`, `react-dom`
- `tailwindcss`, `@tailwindcss/vite` (v4)
- `zustand` (State management)
- `cmdk` (Command palette integration)
- `lucide-react` (Icons)
- `jszip` (ZIP generation for asset downloads)
- `qrcode` (QR code generation)
- `turndown` (HTML to Markdown conversion)
- `fuse.js` (Fuzzy search capabilities - optional)
- `shadcn/ui` components (installed via `npx shadcn@latest add ...`)

### Dev Dependencies
- `typescript`
- `vite`, `@crxjs/vite-plugin`
- `vitest` (Unit testing)
- `playwright` (E2E testing)
- `eslint`, `prettier`

### `package.json` Scripts

```json
"scripts": {
  "dev": "vite",
  "build": "tsc --noEmit && vite build",
  "lint": "eslint src --ext ts,tsx --report-unused-disable-directives --max-warnings 0",
  "format": "prettier --write \"src/**/*.{ts,tsx,css,json}\"",
  "typecheck": "tsc --noEmit",
  "test": "vitest run",
  "test:watch": "vitest",
  "test:e2e": "playwright test",
  "zip": "node scripts/zip.js"
}
```

## 4. Performance Rules

- **Content script must initialize in <100ms**
- **Sidebar must render in <200ms**
- **Element highlight must update at 60fps**: Do not use React in Layer 1 (content script DOM overlay). Use vanilla DOM APIs.
- **Lazy-load modules**: Load module code into the sidebar only when the user opens the specific tool.
- **Memory budget**: Total extension footprint <50MB.
- **Content script CSS**: <10KB (only essential overlay styles).
- **No blocking operations**: The content script main thread must remain unblocked.
- **Use `requestAnimationFrame`**: Ensure all visual updates in Layer 1 are synced with the display refresh rate.
- **Debounce events**: Resize and scroll handlers must be debounced at ~16ms (60fps).
- **Clean up**: Ensure complete removal of all overlays, listeners, and intervals when DevLens is closed.

### Lazy Loading Code Pattern

```typescript
// Dynamically import module core logic only when needed
const loadModule = async (moduleName: string) => {
  try {
    const mod = await import(`@/core/modules/${moduleName}/index.ts`);
    return mod;
  } catch (err) {
    console.error(`Failed to load module: ${moduleName}`, err);
    throw err;
  }
};
```

## 5. Security & Privacy

- **Local-only processing**: ALL feature computation happens in the browser. Zero data leaves the user's machine.
- **No telemetry**: Absolutely no analytics, crash reporting, or tracking.
- **Minimal permissions**: Prioritize `activeTab`. Request optional permissions only on-demand with a clear user explanation.
- **CSP compliance**: Extension pages use a strict Content Security Policy. Content scripts cannot execute inline scripts.
- **No `eval()`**: Never use `eval` or `new Function()` in any context.
- **XSS prevention**: All DOM content inserted via `textContent` or robust sanitization, never raw `innerHTML` with user data.
- **License validation**: Offline-capable. License keys are validated locally against a hash. No server calls occur after the initial activation.
- **CORS in API tester**: The extension can bypass CORS via the service worker `fetch` API, but users must be warned about the security implications of executing untrusted requests.
- **Third-party dependencies**: Continually audit all npm packages. Pin versions exactly. No dependencies with known vulnerabilities allowed.

## 6. Error Handling

- **Use Result pattern**: Avoid throwing errors across module boundaries.
```typescript
export type Result<T, E = Error> = { ok: true; value: T } | { ok: false; error: E };

export function extractCSS(element: Element): Result<CSSData> {
  try {
    // ... extraction logic
    return { ok: true, value: cssData };
  } catch (e) {
    return { ok: false, error: e instanceof Error ? e : new Error(String(e)) };
  }
}
```
- **React Error Boundaries**: Implement boundaries per sidebar module so one crashed module doesn't take down the entire extension.
- **Content script errors**: Catch and log. A content script must never crash the host page.
- **Service worker**: Handle message passing failures gracefully.
- **User feedback**: Show user-friendly error messages in the sidebar UI, reserving detailed developer stack traces for the console.

## 7. Testing Strategy

### Unit Tests (Vitest)
- Place tests in `src/core/modules/<name>/__tests__/`.
- Focus on pure functions: CSS extraction, color conversion, font parsing, HTML-to-JSX generation.
- Mock browser APIs (`chrome.*`, DOM window objects) using `vi.mock()`.
- **Target coverage**: 80%+ for core modules.
- Run locally via `npm run test`.

### Integration Tests (Playwright)
- Test full extension flows: load target page → open DevLens → inspect element → verify sidebar output.
- Test across multiple browsers (Chromium, Edge).
- Verify behavior against diverse architectures: static HTML, React SPAs, and Next.js SSR apps.
- Run locally via `npm run test:e2e`.

### Manual Test Checklist
- **M1 (CSS Inspector)**: Inspect a complex element with nested children, pseudo-elements, and CSS variables → verify correct extracted CSS.
- **M4 (Color Palette)**: Extract colors from a gradient-heavy page → verify all hex/rgba colors found.
- **M7 (Asset Extractor)**: Extract from a page utilizing lazy-loaded or background images → verify all sources are captured.
- **M8 (Screenshots)**: Take a full-page screenshot of a 5000px tall page → verify no stitching gaps exist.
- **M12 (Console)**: Inject `console.log(complexObject)` → verify toast notification displays expandable content correctly.

## 8. Chrome Web Store Publishing

- **Store listing**: Title (45 char max), clear description, screenshots (1280×800 or 640×400), promo images (440x280).
- **Required screenshots**:
  1) Command palette active
  2) CSS Inspector in use
  3) Color palette breakdown
  4) Code export view
  5) Screenshots tool
- **Privacy practices**: Explicitly state that no data is collected, and justify every requested permission.
- **Review process**: Expect 1-3 days for approval.
- **Update process**: Bump `version` in `manifest.json`, run `npm run zip`, upload the resulting ZIP to the Chrome Developer Dashboard.
- **Pricing**: Utilize an external licensing system (e.g., Gumroad, LemonSqueezy) with local key validation.

## 9. Firefox Add-ons (Future)

- **Manifest V3 differences**: Requires `browser_specific_settings` with an addon ID. Requires `sidebar_action` instead of `sidePanel`.
- **Polyfill**: Use `webextension-polyfill` for cross-browser `chrome.*` to `browser.*` API compatibility.
- **Firefox Add-on review**: Typically 1-2 days.
- **Note**: The Firefox Side Panel API implementation differs significantly; a robust fallback using `sidebar_action` will be necessary.

## 10. CI/CD (GitHub Actions)

Workflow configuration `.github/workflows/ci.yml`:

```yaml
name: DevLens CI
on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - uses: actions/setup-node@v4
        with:
          node-version: 20
          
      - name: Install dependencies
        run: npm ci
        
      - name: Linting
        run: npm run lint
        
      - name: Type checking
        run: npm run typecheck
        
      - name: Run unit tests
        run: npm run test
        
      - name: Build extension
        run: npm run build
        
      - name: Upload Artifact
        uses: actions/upload-artifact@v4
        with:
          name: devlens-extension
          path: dist/
```
- Includes required lint, typecheck, test, and build steps.
- Uploads the production `dist/` directory as an artifact.
- *Future*: Tag-triggered releases for automated store uploads via API.

## 11. Development Workflow

- `npm run dev` → Starts Vite dev server with CRXJS hot module replacement.
- `npm run build` → Creates a production-ready build in `dist/`.
- `npm run test` → Runs Vitest unit tests.
- `npm run test:e2e` → Runs Playwright E2E tests.
- `npm run lint` → Performs ESLint and Prettier checks.
- `npm run typecheck` → Runs TypeScript compilation check without emitting files.
- `npm run zip` → Packages the `dist/` folder into a ZIP for store upload.
- **Loading in Chrome**: Navigate to `chrome://extensions` → Enable Developer mode → Click "Load unpacked" → Select the project's `dist/` directory.

## 12. Future Desktop App Notes (Phase 2)

When planning the desktop application version of DevLens:
- **Framework**: Use Tauri (Rust-based, lighter footprint) or Electron.
- **Code Reuse**: Entirely reuse `src/core/modules/` (shared core logic) and `src/ui/components/` (shadcn works natively in Electron/Tauri webviews).
- **API Replacement**: Replace extension-specific APIs (`chrome.*`) with Node.js or Tauri equivalents.
- **New Modules**: Add desktop-only capabilities like multi-viewport rendering, native terminal access, and direct file system manipulation.
- **Build Pipeline**: Create a separate Vite configuration specifically targeting the desktop shell.
- **Distribution**: Distribute via direct download utilizing built-in auto-updaters.
