(function() {
  if (window.__TAURI_CDP_PROBE_INITIALIZED__) return;
  window.__TAURI_CDP_PROBE_INITIALIZED__ = true;

  const targetLabel = window.__TAURI_INTERNALS__?.currentWindow?.label || 'main';

  function sendLog(level, args) {
    try {
      const message = Array.from(args).map(arg => {
        if (typeof arg === 'object' && arg !== null) {
          try { return JSON.stringify(arg); } catch (e) { return String(arg); }
        }
        return String(arg);
      }).join(' ');

      // Post to local CDP server (runs on dynamic port, discovered via metadata or fallback)
      if (window.__TAURI_CDP_PORT__) {
        fetch(`http://127.0.0.1:${window.__TAURI_CDP_PORT__}/api/logs/${encodeURIComponent(targetLabel)}`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ level, message })
        }).catch(() => {});
      }
    } catch (e) {}
  }

  const originalLog = console.log;
  const originalWarn = console.warn;
  const originalError = console.error;
  const originalInfo = console.info;

  console.log = function(...args) {
    originalLog.apply(console, args);
    sendLog('info', args);
  };
  console.warn = function(...args) {
    originalWarn.apply(console, args);
    sendLog('warn', args);
  };
  console.error = function(...args) {
    originalError.apply(console, args);
    sendLog('error', args);
  };
  console.info = function(...args) {
    originalInfo.apply(console, args);
    sendLog('info', args);
  };

  window.addEventListener('error', (e) => {
    sendLog('error', [`Uncaught Exception: ${e.message} at ${e.filename}:${e.lineno}`]);
  });
})();
