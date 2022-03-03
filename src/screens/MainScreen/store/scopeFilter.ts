import { MainScreenScope } from '../../../models/MainScreen';
import { WindowListEntry } from '../../../connectors/kitty';

const allScopeFilter = (entries: WindowListEntry[]) => entries;
const windowScopeFilter = (entries: WindowListEntry[]) => entries.filter((entry) => entry.osWindowIsFocused);
const tabScopeFilter = (entries: WindowListEntry[]) => entries.filter((entry) => entry.tabIsFocused);

export const filterEntries = (scope: string, entries: WindowListEntry[]) => {
  switch (scope.toUpperCase()) {
    case MainScreenScope.Window:
      return windowScopeFilter(entries);
    case MainScreenScope.Tab:
      return tabScopeFilter(entries);
    case MainScreenScope.All:
    default:
      return allScopeFilter(entries);
  }
};
