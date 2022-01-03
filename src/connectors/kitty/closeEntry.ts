import { exec } from 'child_process';
import { ExecError, WindowListEntry, WindowListEntryType } from './model';

export const closeEntry = (entry: WindowListEntry): Promise<void> => {
  switch (entry.type) {
    case WindowListEntryType.OsWindow:
      return Promise.resolve(undefined);
    case WindowListEntryType.Tab:
      return closeTab(entry.id);
    case WindowListEntryType.Window:
      return closeWindow(entry.id);
    default:
      throw new Error(`Can't focus entry with id: ${entry.id}`);
  }
};

const closeWindow = (id: number): Promise<void> => {
  return new Promise<void>((resolve, reject) => {
    exec(`kitty @ close-window -m id:${id}`, (error, _stdout, stderror) => {
      if (error) {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        reject(new ExecError((error as any).code, stderror));
      } else {
        resolve();
      }
    });
  });
};

const closeTab = (id: number): Promise<void> => {
  return new Promise<void>((resolve, reject) => {
    exec(`kitty @ close-tab -m id:${id}`, (error, _stdout, stderror) => {
      if (error) {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        reject(new ExecError((error as any).code, stderror));
      } else {
        resolve();
      }
    });
  });
};
