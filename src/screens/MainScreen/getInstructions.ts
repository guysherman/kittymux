import { WindowListEntryType } from '../../connectors/kitty';
import { MainScreenMode, MainScreenState } from '../../models/MainScreen';

const getNavModeInstructions = (entryType: WindowListEntryType): string => {
  switch (entryType) {
    case WindowListEntryType.Tab:
    case WindowListEntryType.Window:
      return 'j/k:next/prev item|J/K:next/prev tab|x:close item';
    case WindowListEntryType.OsWindow:
    case WindowListEntryType.None:
    default:
      return '';
  }
};

export const getInstructions = (state: MainScreenState): string => {
  const { entries, selectedIndex, mode } = state;
  const entry = entries[selectedIndex];

  if (!entry) {
    return '';
  }

  switch (mode) {
    case MainScreenMode.Navigate:
      return getNavModeInstructions(entry.type);
    case MainScreenMode.Command:
      return 'Enter command...';
    case MainScreenMode.Rename:
      return 'Enter new tab/window name';
    case MainScreenMode.QuickNav:
      return 'Press a shortcut key to focus the window/tab';
    case MainScreenMode.SetQuickNav:
      return 'Press a key (a-z1-9) to set a shortcut';
    default:
      return '???';
  }
};
