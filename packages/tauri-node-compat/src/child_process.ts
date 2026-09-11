import { invoke } from '@tauri-apps/api/core';
import { EventEmitter } from './events';

export interface ExecResult {
  stdout: string;
  stderr: string;
}

export function exec(
  command: string,
  callback?: (error: Error | null, stdout: string, stderr: string) => void
): EventEmitter {
  const emitter = new EventEmitter();

  (async () => {
    try {
      const parts = command.split(' ');
      const program = parts[0];
      const args = parts.slice(1);
      const res = await invoke<ExecResult>('plugin:shell|execute', {
        program,
        args,
      });
      callback?.(null, res.stdout, res.stderr);
      emitter.emit('exit', 0);
    } catch (err: any) {
      const error = err instanceof Error ? err : new Error(String(err));
      callback?.(error, '', error.message);
      emitter.emit('error', error);
      emitter.emit('exit', 1);
    }
  })();

  return emitter;
}

export function spawn(program: string, args: string[] = []): EventEmitter {
  const emitter = new EventEmitter();
  (async () => {
    try {
      const res = await invoke<ExecResult>('plugin:shell|execute', {
        program,
        args,
      });
      emitter.emit('data', res.stdout);
      emitter.emit('close', 0);
    } catch (err) {
      emitter.emit('error', err);
      emitter.emit('close', 1);
    }
  })();
  return emitter;
}

export default {
  exec,
  spawn,
};
