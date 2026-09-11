import { invoke } from '@tauri-apps/api/core';

export interface TabBounds {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface CreateTabOptions {
  label: string;
  url: string;
  bounds: TabBounds;
  windowLabel?: string;
  partition?: string;
  userAgent?: string;
  devtools?: boolean;
}

export interface TabInfo {
  label: string;
  url: string;
  bounds: TabBounds;
  isActive: boolean;
  partition?: string;
}

/**
 * Create a new child webview tab attached to the parent window.
 */
export async function createTab(options: CreateTabOptions): Promise<TabInfo> {
  return await invoke<TabInfo>('plugin:browser-view|create_tab', { options });
}

/**
 * Close and destroy an existing child webview tab.
 */
export async function closeTab(label: string): Promise<void> {
  return await invoke<void>('plugin:browser-view|close_tab', { label });
}

/**
 * Dynamically resize and reposition a child webview tab.
 */
export async function setTabBounds(label: string, bounds: TabBounds): Promise<void> {
  return await invoke<void>('plugin:browser-view|set_tab_bounds', { label, bounds });
}

/**
 * Switch focus to a specific tab, hiding other tabs.
 */
export async function switchTab(label: string): Promise<void> {
  return await invoke<void>('plugin:browser-view|switch_tab', { label });
}

/**
 * Navigate a specific tab to a new URL.
 */
export async function navigateTab(label: string, url: string): Promise<void> {
  return await invoke<void>('plugin:browser-view|navigate_tab', { label, url });
}

/**
 * Evaluate arbitrary JavaScript inside a child webview tab.
 */
export async function evalScriptInTab(label: string, script: string): Promise<void> {
  return await invoke<void>('plugin:browser-view|eval_script_in_tab', { label, script });
}

/**
 * Retrieve all currently registered webview tabs.
 */
export async function listTabs(): Promise<TabInfo[]> {
  return await invoke<TabInfo[]>('plugin:browser-view|list_tabs');
}
