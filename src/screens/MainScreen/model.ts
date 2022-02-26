import { createContext } from '@guysherman/treecat';
import { WindowListEntry, WindowListEntryType } from '../../connectors/kitty';

export enum MainScreenMode {
  Navigate = 'NAVIGATE',
  Rename = 'RENAME',
  Command = 'COMMAND',
  QuickNav = 'QUICK_NAV',
  SetQuickNav = 'SET_QUICK_NAV',
}

export enum MainScreenScope {
  All = 'ALL',
  Window = 'WINDOW',
  Tab = 'TAB',
}

export const DefaultMainScreenMode = MainScreenMode.Navigate;

export interface QuickNavHandle {
  id: number;
  type: WindowListEntryType;
}

export interface MainScreenState {
  entries: WindowListEntry[];
  selectedIndex: number;
  mode: MainScreenMode;
  quickNavKeys: Record<string, QuickNavHandle>;
}

export interface MainScreenContext {
  state: MainScreenState;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  dispatch: (action: any) => void;
}

export const mainScreenContext = createContext({
  state: {
    entries: [] as WindowListEntry[],
    selectedIndex: 0,
    mode: MainScreenMode.Navigate,
    quickNavKeys: {} as Record<string, QuickNavHandle>,
  },
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  dispatch: (action: any): void => {},
});
