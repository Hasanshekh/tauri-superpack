import {
  ensurePolyfills,
  pickColor,
  captureScreen,
  showOpenFilePicker,
} from '@tauri-superpack/plugin-uniform';
import {
  createTab,
  closeTab,
  switchTab,
  listTabs,
} from '@tauri-superpack/plugin-browser-view';
import {
  getServerInfo,
  registerBuffer,
} from '@tauri-superpack/plugin-zerocopy';
import {
  getCdpServerInfo,
  openDevtoolsWindow,
} from '@tauri-superpack/plugin-cdp';
import {
  evalScript,
  registerPlugin,
} from '@tauri-superpack/plugin-isolate';
import { path, fs } from 'tauri-node-compat';

// 1. Initialize web standards polyfills
ensurePolyfills();

const uniformOutput = document.getElementById('uniform-output')!;
const browserOutput = document.getElementById('browser-output')!;
const zerocopyOutput = document.getElementById('zerocopy-output')!;
const cdpOutput = document.getElementById('cdp-output')!;
const isolateOutput = document.getElementById('isolate-output')!;
const nodeCompatOutput = document.getElementById('node-compat-output')!;
const tabsContainer = document.getElementById('tabs-container')!;
const urlInput = document.getElementById('tab-url') as HTMLInputElement;

// EyeDropper Test
document.getElementById('btn-eyedropper')?.addEventListener('click', async () => {
  try {
    uniformOutput.textContent = 'Sampling pixel color...';
    const result = await pickColor(100, 100);
    uniformOutput.innerHTML = `<span class="swatch" style="background:${result.sRgbHex}"></span>Sampled Hex: <b>${result.sRgbHex}</b> (RGBA: ${result.r}, ${result.g}, ${result.b}, ${result.a})`;
  } catch (err) {
    uniformOutput.textContent = 'Error: ' + err;
  }
});

// Screen Capture Test
document.getElementById('btn-capture')?.addEventListener('click', async () => {
  try {
    uniformOutput.textContent = 'Capturing screen...';
    const result = await captureScreen({ width: 300, height: 200, x: 0, y: 0 });
    uniformOutput.innerHTML = `Screen captured (${result.width}x${result.height})<br><img src="${result.dataUrl}" style="max-height:80px; margin-top:6px; border-radius:4px;" />`;
  } catch (err) {
    uniformOutput.textContent = 'Error: ' + err;
  }
});

// Native Dialog Test
document.getElementById('btn-dialog')?.addEventListener('click', async () => {
  try {
    uniformOutput.textContent = 'Opening native dialog...';
    const files = await showOpenFilePicker({ title: 'Select any file' });
    uniformOutput.textContent = 'Selected files: ' + JSON.stringify(files);
  } catch (err) {
    uniformOutput.textContent = 'Cancelled or Error: ' + err;
  }
});

// Browser-view Tab Counter
let tabIndex = 1;

async function refreshTabList() {
  const tabs = await listTabs();
  tabsContainer.innerHTML = '';
  tabs.forEach((tab) => {
    const tag = document.createElement('div');
    tag.className = 'tab-tag';
    tag.innerHTML = `<span>${tab.label}</span> <span class="close">&times;</span>`;
    tag.querySelector('span')?.addEventListener('click', () => switchTab(tab.label));
    tag.querySelector('.close')?.addEventListener('click', async (e) => {
      e.stopPropagation();
      await closeTab(tab.label);
      refreshTabList();
    });
    tabsContainer.appendChild(tag);
  });
  browserOutput.textContent = `Active tabs (${tabs.length}): ` + tabs.map((t) => t.label).join(', ');
}

async function handleCreateTab(isIsolated: boolean) {
  const label = `tab-${tabIndex++}`;
  const url = urlInput.value.trim() || 'https://example.com';
  try {
    browserOutput.textContent = `Creating tab ${label}...`;
    await createTab({
      label,
      url,
      bounds: { x: 50, y: 440, width: 880, height: 350 },
      partition: isIsolated ? `session-${label}` : undefined,
    });
    await refreshTabList();
  } catch (err) {
    browserOutput.textContent = 'Error creating tab: ' + err;
  }
}

document.getElementById('btn-create-tab')?.addEventListener('click', () => handleCreateTab(false));
document.getElementById('btn-isolated-tab')?.addEventListener('click', () => handleCreateTab(true));

// 3. ZeroCopy Streaming Test
document.getElementById('btn-stream-info')?.addEventListener('click', async () => {
  try {
    const info = await getServerInfo();
    zerocopyOutput.textContent = `Local Loopback Server: http://127.0.0.1:${info.port} (Token: ${info.token.slice(0, 8)}...)`;
  } catch (err) {
    zerocopyOutput.textContent = 'Error: ' + err;
  }
});

document.getElementById('btn-stream-10mb')?.addEventListener('click', async () => {
  try {
    zerocopyOutput.textContent = 'Generating & streaming 10 MB synthetic binary buffer...';
    const size = 10 * 1024 * 1024;
    const buffer = new Uint8Array(size);
    for (let i = 0; i < 1000; i++) buffer[i] = i % 256;

    const startReg = performance.now();
    const reg = await registerBuffer(buffer, { mimeType: 'application/octet-stream', oneTime: true });
    const regTime = (performance.now() - startReg).toFixed(1);

    const startFetch = performance.now();
    const res = await fetch(reg.url);
    const fetchedBuffer = await res.arrayBuffer();
    const fetchTime = (performance.now() - startFetch).toFixed(1);

    zerocopyOutput.innerHTML = `✅ Streamed <b>${(fetchedBuffer.byteLength / (1024 * 1024)).toFixed(1)} MB</b> in <b>${fetchTime}ms</b>! (Rust registration: ${regTime}ms)`;
  } catch (err) {
    zerocopyOutput.textContent = 'Streaming Error: ' + err;
  }
});

// 4. CDP Universal DevTools Test
document.getElementById('btn-open-devtools')?.addEventListener('click', async () => {
  try {
    cdpOutput.textContent = 'Launching standalone Chrome DevTools window...';
    await openDevtoolsWindow('main');
    cdpOutput.textContent = 'DevTools window opened for target "main".';
  } catch (err) {
    cdpOutput.textContent = 'Error: ' + err;
  }
});

document.getElementById('btn-emit-log')?.addEventListener('click', async () => {
  const cdpInfo = await getCdpServerInfo();
  console.log('Sample Log event emitted at', new Date().toISOString());
  console.warn('Sample Warning event for CDP inspector!');
  cdpOutput.innerHTML = `Logs emitted to console and captured by CDP probe (Server port: ${cdpInfo.port}). Check the DevTools window!`;
});

// 5. Isolate Sandboxed JS Engine Test
document.getElementById('btn-eval-math')?.addEventListener('click', async () => {
  try {
    isolateOutput.textContent = 'Evaluating expression in pure Rust JS sandbox...';
    const code = 'const a = 5; const b = 10; const c = 2; a * b + c;';
    const res = await evalScript(code);
    isolateOutput.innerHTML = `Sandbox Result: <b>${res.output}</b> (Success: ${res.success})`;
  } catch (err) {
    isolateOutput.textContent = 'Sandbox Execution Error: ' + err;
  }
});

document.getElementById('btn-eval-plugin')?.addEventListener('click', async () => {
  try {
    isolateOutput.textContent = 'Installing mock plugin in sandbox...';
    const pluginCode = `
      function calculateStats(items) {
        return items.reduce((acc, x) => acc + x, 0);
      }
      calculateStats([10, 20, 30, 40]);
    `;
    const res = await registerPlugin(
      {
        id: 'analytics-plugin',
        name: 'Stats Analyzer',
        version: '1.0.0',
        permissions: ['read-dom'],
      },
      pluginCode
    );
    isolateOutput.innerHTML = `Plugin mounted & executed safely! Output: <b>${res.output}</b>`;
  } catch (err) {
    isolateOutput.textContent = 'Plugin Error: ' + err;
  }
});

// 6. Node.js Compatibility Shims Test
document.getElementById('btn-test-path')?.addEventListener('click', () => {
  const joined = path.join('/usr', 'local', 'bin', 'tauri-app');
  const base = path.basename('/path/to/my-file.spec.ts', '.spec.ts');
  const ext = path.extname('/path/to/my-file.spec.ts');
  nodeCompatOutput.innerHTML = `<b>path.join:</b> ${joined}<br><b>path.basename:</b> ${base}<br><b>path.extname:</b> ${ext}`;
});

document.getElementById('btn-test-fs')?.addEventListener('click', async () => {
  try {
    nodeCompatOutput.textContent = 'Writing and reading file via fs.promises shim...';
    const testPath = '/data/test.json';
    const testPayload = JSON.stringify({ hello: 'world', tauri: 'v2', nodeCompat: true });
    await fs.promises.writeFile(testPath, testPayload);
    const readBack = await fs.promises.readFile(testPath);
    nodeCompatOutput.innerHTML = `✅ <b>fs.promises</b> write & read successful!<br>Content: <code>${readBack}</code>`;
  } catch (err) {
    nodeCompatOutput.textContent = 'FS Error: ' + err;
  }
});
