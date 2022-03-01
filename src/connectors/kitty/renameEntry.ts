import { exec } from 'child_process';
import { ExecError, WindowListEntry, WindowListEntryType } from '../../models/Kitty';

export const renameEntry = (entry: WindowListEntry, newName: string): Promise<void> => {
  switch (entry.type) {
    case WindowListEntryType.Tab:
      return renameTab(entry.id, newName);
    case WindowListEntryType.Window:
      return renameWindow(entry.id, newName);
    case WindowListEntryType.OsWindow:
      return Promise.resolve(undefined);
    case WindowListEntryType.None:
    default:
      throw new Error(`Can't rename entry with id: ${entry.id}`);
  }
};

const renameTab = (id: number, newName: string): Promise<void> => {
  return new Promise<void>((resolve, reject) => {
    exec(`kitty @ set-tab-title -m id:${id} ${newName}`, (error, _stdout, stderror) => {
      if (error) {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        reject(new ExecError((error as any).code, stderror));
      } else {
        resolve();
      }
    });
  });
};

const renameWindow = (id: number, newName: string): Promise<void> => {
  return new Promise<void>((resolve, reject) => {
    exec(`kitty @ set-window-title -m id:${id} ${newName}`, (error, _stdout, stderror) => {
      if (error) {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        reject(new ExecError((error as any).code, stderror));
      } else {
        resolve();
      }
    });
  });
};
