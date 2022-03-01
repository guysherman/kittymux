import kittyCommand from './kittyCommand';
import { WindowListEntry, WindowListEntryType } from '../../models/Kitty';

export const closeEntry = (entry: WindowListEntry): Promise<unknown> => {
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

const closeWindow = (id: number): Promise<unknown> => {
  const args = ['close-window', '-m', `id:${id}`];
  return kittyCommand(args);
};

const closeTab = (id: number): Promise<unknown> => {
  const args = ['close-tab', '-m', `id:${id}`];
  return kittyCommand(args);
};
