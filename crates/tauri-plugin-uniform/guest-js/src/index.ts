import { invoke } from '@tauri-apps/api/core';

export interface ColorResult {
  sRgbHex: string;
  r: number;
  g: number;
  b: number;
  a: number;
}

export interface CaptureOptions {
  monitorIndex?: number;
  x?: number;
  y?: number;
  width?: number;
  height?: number;
}

export interface CaptureResult {
  dataUrl: string;
  width: number;
  height: number;
}

export interface OpenFileFilter {
  name: string;
  extensions: string[];
}

export interface OpenDialogOptions {
  title?: string;
  defaultPath?: string;
  filters?: OpenFileFilter[];
  multiple?: boolean;
  directory?: boolean;
}

export interface SaveDialogOptions {
  title?: string;
  defaultPath?: string;
  defaultName?: string;
  filters?: OpenFileFilter[];
}

/**
 * Sample a pixel color at the specified screen coordinates or interactive fallback.
 */
export async function pickColor(x?: number, y?: number): Promise<ColorResult> {
  return await invoke<ColorResult>('plugin:uniform|pick_color', { x, y });
}

/**
 * Capture an entire screen or a cropped region into a Base64 PNG data URL.
 */
export async function captureScreen(options?: CaptureOptions): Promise<CaptureResult> {
  return await invoke<CaptureResult>('plugin:uniform|capture_screen', { options });
}

/**
 * Open a native cross-platform file selection dialog.
 */
export async function showOpenFilePicker(options?: OpenDialogOptions): Promise<string[]> {
  return await invoke<string[]>('plugin:uniform|show_open_file_picker', { options });
}

/**
 * Open a native cross-platform save file dialog.
 */
export async function showSaveFilePicker(options?: SaveDialogOptions): Promise<string> {
  return await invoke<string>('plugin:uniform|show_save_file_picker', { options });
}

/**
 * Ensure all Web standards polyfills (like EyeDropper) are installed in the current window.
 */
export function ensurePolyfills(): void {
  if (typeof window === 'undefined') return;

  if (!('EyeDropper' in window)) {
    // @ts-ignore - dynamic class definition for browser compatibility
    window.EyeDropper = class EyeDropper {
      async open(): Promise<{ sRGBHex: string }> {
        const res = await pickColor();
        return { sRGBHex: res.sRgbHex };
      }
    };
  }
}
