import { invoke } from '@tauri-apps/api/core';

export namespace promises {
  export async function readFile(path: string, options?: { encoding?: string }): Promise<string | Uint8Array> {
    try {
      const content = await invoke<string>('plugin:fs|read_text_file', { path });
      return content;
    } catch {
      // Browser / mock fallback for local dev
      const item = localStorage.getItem(`vfs:${path}`);
      if (item !== null) return item;
      throw new Error(`ENOENT: no such file or directory, open '${path}'`);
    }
  }

  export async function writeFile(path: string, data: string | Uint8Array): Promise<void> {
    const text = typeof data === 'string' ? data : new TextDecoder().decode(data);
    try {
      await invoke<void>('plugin:fs|write_text_file', { path, contents: text });
    } catch {
      localStorage.setItem(`vfs:${path}`, text);
    }
  }

  export async function mkdir(path: string, options?: { recursive?: boolean }): Promise<void> {
    try {
      await invoke<void>('plugin:fs|mkdir', { path, options });
    } catch {}
  }

  export async function readdir(path: string): Promise<string[]> {
    try {
      return await invoke<string[]>('plugin:fs|read_dir', { path });
    } catch {
      const prefix = `vfs:${path}`;
      const keys: string[] = [];
      for (let i = 0; i < localStorage.length; i++) {
        const k = localStorage.key(i);
        if (k && k.startsWith(prefix)) {
          keys.push(k.slice(prefix.length + 1));
        }
      }
      return keys;
    }
  }

  export async function unlink(path: string): Promise<void> {
    try {
      await invoke<void>('plugin:fs|remove', { path });
    } catch {
      localStorage.removeItem(`vfs:${path}`);
    }
  }

  export async function stat(path: string): Promise<{ isFile: () => boolean; isDirectory: () => boolean; size: number }> {
    return {
      isFile: () => true,
      isDirectory: () => false,
      size: 1024,
    };
  }
}

export default {
  promises,
};
