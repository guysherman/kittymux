import { WindowListEntryType } from '../../connectors/kitty';

export const getInstructions = (entryType: WindowListEntryType): string => {
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
