import { invoke } from '@tauri-apps/api/core';

export interface CdpServerInfo {
  port: number;
  devtoolsBaseUrl: string;
}

let cachedInfo: CdpServerInfo | null = null;

/**
 * Retrieve metadata about the local Chrome DevTools bridge server.
 */
export async function getCdpServerInfo(): Promise<CdpServerInfo> {
  if (cachedInfo) return cachedInfo;
  cachedInfo = await invoke<CdpServerInfo>('plugin:cdp|get_cdp_server_info');
  return cachedInfo;
}

/**
 * Get the direct HTTP inspection URL for a specific webview target (e.g. 'main' or 'tab-1').
 */
export async function getDevtoolsUrl(target: string = 'main'): Promise<string> {
  return await invoke<string>('plugin:cdp|get_devtools_url', { target });
}

/**
 * Open a standalone Chrome DevTools inspection window connected to the target webview.
 */
export async function openDevtoolsWindow(target: string = 'main'): Promise<void> {
  return await invoke<void>('plugin:cdp|open_devtools_window', { target });
}
