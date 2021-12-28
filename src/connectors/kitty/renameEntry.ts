import { exec } from 'child_process';
import { listWindows } from './listWindows';
import { ExecError, WindowListEntry, WindowListEntryType } from './model';
import { processWindowList } from './processWindowList';

export const renameEntry = (entry: WindowListEntry, newName: string): Promise<WindowListEntry[]> => {
  switch (entry.type) {
    case WindowListEntryType.Tab:
      return renameTab(entry.id, newName);
    case WindowListEntryType.Window:
      return renameWindow(entry.id, newName);
    case WindowListEntryType.OsWindow:
      return listWindows().then(processWindowList);
    case WindowListEntryType.None:
    default:
      throw new Error(`Can't rename entry with id: ${entry.id}`);
  }
};

const renameTab = (id: number, newName: string): Promise<WindowListEntry[]> => {
  return new Promise<void>((resolve, reject) => {
    exec(`kitty @ set-tab-title -m id:${id} ${newName}`, (error, _stdout, stderror) => {
      if (error) {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        reject(new ExecError((error as any).code, stderror));
      } else {
        resolve();
      }
    });
  })
    .then(listWindows)
    .then(processWindowList);
};

const renameWindow = (id: number, newName: string): Promise<WindowListEntry[]> => {
  return new Promise<void>((resolve, reject) => {
    exec(`kitty @ set-window-title -m id:${id} ${newName}`, (error, _stdout, stderror) => {
      if (error) {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        reject(new ExecError((error as any).code, stderror));
      } else {
        resolve();
      }
    });
  })
    .then(listWindows)
    .then(processWindowList);
};
