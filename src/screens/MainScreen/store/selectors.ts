import { WindowListEntry } from '../../../models/Kitty';
import { MainScreenMode, MainScreenState } from '../../../models/MainScreen';

const getItems = (state: MainScreenState, entries: WindowListEntry[]): string[] => {
  return entries.map((entry: WindowListEntry) => {
    if (state.mode === MainScreenMode.QuickNav || state.mode === MainScreenMode.SetQuickNav) {
      const entryQuickNav = Object.entries(state.quickNavKeys).find(([, handles]) =>
        handles.find((handle) => handle.id === entry.id && handle.type === entry.type),
      );

      return `{inverse}${entryQuickNav?.[0] ?? ' '}{/inverse}\t${entry.text}`;
    } else {
      return entry.text;
    }
  });
};

export default getItems;
