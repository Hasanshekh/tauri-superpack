(function() {
  if (window.__TAURI_UNIFORM_INITIALIZED__) return;
  window.__TAURI_UNIFORM_INITIALIZED__ = true;

  // 1. Polyfill EyeDropper for WebKit (macOS / Linux / iOS)
  if (!('EyeDropper' in window)) {
    window.EyeDropper = class EyeDropper {
      constructor() {}
      async open(options) {
        if (!window.__TAURI_INTERNALS__ || !window.__TAURI_INTERNALS__.invoke) {
          throw new DOMException('EyeDropper failed: Tauri IPC is not available in this context', 'NotAllowedError');
        }
        try {
          const res = await window.__TAURI_INTERNALS__.invoke('plugin:uniform|pick_color', {});
          return { sRGBHex: res.sRgbHex };
        } catch (err) {
          throw new DOMException('The user aborted the request or color selection failed: ' + err, 'AbortError');
        }
      }
    };
  }

  // 2. Normalize CSS & Webview Rendering Differences
  const styleEl = document.createElement('style');
  styleEl.id = 'tauri-uniform-preflight';
  styleEl.textContent = `
    /* Uniform Scrollbar Appearance across Chromium and WebKit */
    ::-webkit-scrollbar {
      width: 8px;
      height: 8px;
    }
    ::-webkit-scrollbar-track {
      background: transparent;
    }
    ::-webkit-scrollbar-thumb {
      background: rgba(128, 128, 128, 0.4);
      border-radius: 4px;
    }
    ::-webkit-scrollbar-thumb:hover {
      background: rgba(128, 128, 128, 0.7);
    }
  `;
  if (document.head) {
    document.head.appendChild(styleEl);
  } else {
    document.addEventListener('DOMContentLoaded', () => {
      document.head.appendChild(styleEl);
    });
  }
})();
