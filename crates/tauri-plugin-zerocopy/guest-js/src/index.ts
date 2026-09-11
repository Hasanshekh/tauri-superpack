import { invoke } from '@tauri-apps/api/core';

export interface ServerInfo {
  port: number;
  token: string;
}

export interface RegisterOptions {
  id?: string;
  mimeType?: string;
  oneTime?: boolean;
}

export interface RegisterBufferResult {
  id: string;
  url: string;
  size: number;
}

let cachedServerInfo: ServerInfo | null = null;

/**
 * Retrieve local loopback streaming server metadata (port and token).
 */
export async function getServerInfo(): Promise<ServerInfo> {
  if (cachedServerInfo) return cachedServerInfo;
  cachedServerInfo = await invoke<ServerInfo>('plugin:zerocopy|get_server_info');
  return cachedServerInfo;
}

/**
 * Register in-memory binary data with the local streaming server.
 * Returns an instant HTTP streaming URL directly consumable by <img>, <video>, or fetch().
 */
export async function registerBuffer(
  data: Uint8Array | ArrayBuffer,
  options: RegisterOptions = {}
): Promise<RegisterBufferResult> {
  const bytes = data instanceof Uint8Array ? Array.from(data) : Array.from(new Uint8Array(data));
  return await invoke<RegisterBufferResult>('plugin:zerocopy|register_buffer', {
    id: options.id,
    data: bytes,
    mimeType: options.mimeType,
    oneTime: options.oneTime,
  });
}

/**
 * Release and purge a registered buffer from memory.
 */
export async function releaseBuffer(id: string): Promise<boolean> {
  return await invoke<boolean>('plugin:zerocopy|release_buffer', { id });
}

/**
 * High-speed binary upload directly from webview to the Rust streaming server,
 * bypassing IPC serialization bottlenecks.
 */
export async function uploadStream(
  id: string,
  data: Uint8Array | Blob
): Promise<{ id: string; size: number }> {
  const info = await getServerInfo();
  const url = `http://127.0.0.1:${info.port}/upload/${encodeURIComponent(id)}?token=${encodeURIComponent(info.token)}`;
  const res = await fetch(url, {
    method: 'POST',
    body: data as BodyInit,
    headers: { 'Content-Type': 'application/octet-stream' },
  });
  if (!res.ok) {
    throw new Error(`Upload failed with status: ${res.status}`);
  }
  return await res.json();
}
