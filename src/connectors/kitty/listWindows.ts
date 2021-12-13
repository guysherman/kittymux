import { exec } from 'child_process';
import { ExecError, KittyOsWindow } from './model';

export const listWindows = (): Promise<KittyOsWindow[]> => {
  return new Promise<KittyOsWindow[]>((resolve, reject) => {
    exec('kitty @ ls', (error, stdout, stderror) => {
      if (error) {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        reject(new ExecError((error as any).code, stderror));
      } else {
        resolve(JSON.parse(stdout));
      }
    });
  });
};
