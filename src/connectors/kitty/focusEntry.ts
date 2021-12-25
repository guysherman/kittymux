import { exec } from 'child_process';
import { ExecError, WindowListEntry, WindowListEntryType } from './model';

export const focusEntry = (entry: WindowListEntry): void => {
  switch (entry.type) {
    case WindowListEntryType.OsWindow:
      break;
    case WindowListEntryType.Tab:
      focusTab(entry.id);
      break;
    case WindowListEntryType.Window:
      focusWindow(entry.id);
      break;
    default:
      throw new Error(`Can't focus entry with id: ${entry.id}`);
  }
};

const focusTab = (id: number): void => {
  exec(`kitty @ focus-tab -m id:${id}`, (error, _stdout, stderror) => {
    if (error) {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      throw new ExecError((error as any).code, stderror);
    } else {
      process.exit(0);
    }
  });
};

const focusWindow = (id: number): void => {
  exec(`kitty @ focus-window -m id:${id}`, (error, _stdout, stderror) => {
    if (error) {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      throw new ExecError((error as any).code, stderror);
    } else {
      process.exit(0);
    }
  });
};
