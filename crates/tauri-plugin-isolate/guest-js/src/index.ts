import { invoke } from '@tauri-apps/api/core';

export interface PluginManifest {
  id: string;
  name: string;
  version: string;
  permissions: string[];
}

export interface EvalResult {
  output: string;
  success: boolean;
  error?: string;
}

/**
 * Execute a snippet of JavaScript inside an isolated, secure sandbox.
 */
export async function evalScript(code: string, pluginId?: string): Promise<EvalResult> {
  return await invoke<EvalResult>('plugin:isolate|eval_script', { code, pluginId });
}

/**
 * Register and mount a 3rd-party community plugin into the sandbox manager.
 */
export async function registerPlugin(
  manifest: PluginManifest,
  code: string
): Promise<EvalResult> {
  return await invoke<EvalResult>('plugin:isolate|register_plugin', { manifest, code });
}

/**
 * List all currently active sandboxed plugins.
 */
export async function listPlugins(): Promise<PluginManifest[]> {
  return await invoke<PluginManifest[]>('plugin:isolate|list_plugins');
}

/**
 * Unload and deactivate a sandboxed plugin.
 */
export async function unloadPlugin(id: string): Promise<void> {
  return await invoke<void>('plugin:isolate|unload_plugin', { id });
}
