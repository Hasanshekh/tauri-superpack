export * as fs from './fs';
export * as path from './path';
export * as child_process from './child_process';
export * as os from './os';
export { EventEmitter } from './events';

export default {
  fs: import('./fs'),
  path: import('./path'),
  child_process: import('./child_process'),
  os: import('./os'),
};
