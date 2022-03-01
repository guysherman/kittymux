import kittyCommand from './kittyCommand';
import { WindowListEntry, WindowListEntryType } from '../../models/Kitty';

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
  const args = ['focus-tab', '-m', `id:${id}`];
  kittyCommand(args).then(() => process.exit(0));
};

const focusWindow = (id: number): void => {
  const args = ['focus-window', '-m', `id:${id}`];
  kittyCommand(args).then(() => process.exit(0));
};
