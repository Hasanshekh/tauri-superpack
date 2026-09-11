export function platform(): string {
  if (typeof navigator !== 'undefined') {
    if (navigator.userAgent.includes('Win')) return 'win32';
    if (navigator.userAgent.includes('Mac')) return 'darwin';
    if (navigator.userAgent.includes('Linux')) return 'linux';
  }
  return 'win32';
}

export function arch(): string {
  return 'x64';
}

export function type(): string {
  const p = platform();
  if (p === 'win32') return 'Windows_NT';
  if (p === 'darwin') return 'Darwin';
  return 'Linux';
}

export function tmpdir(): string {
  return platform() === 'win32' ? 'C:\\Temp' : '/tmp';
}

export function homedir(): string {
  return platform() === 'win32' ? 'C:\\Users\\User' : '/home/user';
}

export default {
  platform,
  arch,
  type,
  tmpdir,
  homedir,
};
