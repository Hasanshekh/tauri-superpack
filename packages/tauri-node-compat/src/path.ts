export const sep = '/';

export function isAbsolute(path: string): boolean {
  return path.startsWith('/') || /^[a-zA-Z]:[\\/]/.test(path);
}

export function normalize(path: string): string {
  const parts = path.replace(/\\/g, '/').split('/');
  const resolved: string[] = [];
  for (const part of parts) {
    if (part === '' || part === '.') continue;
    if (part === '..') {
      resolved.pop();
    } else {
      resolved.push(part);
    }
  }
  const prefix = path.startsWith('/') ? '/' : '';
  return prefix + resolved.join('/');
}

export function join(...paths: string[]): string {
  return normalize(paths.filter(Boolean).join('/'));
}

export function resolve(...paths: string[]): string {
  return normalize(paths.filter(Boolean).join('/'));
}

export function dirname(path: string): string {
  const norm = normalize(path);
  const idx = norm.lastIndexOf('/');
  if (idx === -1) return '.';
  if (idx === 0) return '/';
  return norm.slice(0, idx);
}

export function basename(path: string, ext?: string): string {
  const norm = normalize(path);
  let base = norm.slice(norm.lastIndexOf('/') + 1);
  if (ext && base.endsWith(ext)) {
    base = base.slice(0, -ext.length);
  }
  return base;
}

export function extname(path: string): string {
  const base = basename(path);
  const idx = base.lastIndexOf('.');
  if (idx <= 0) return '';
  return base.slice(idx);
}

export default {
  sep,
  isAbsolute,
  normalize,
  join,
  resolve,
  dirname,
  basename,
  extname,
};
