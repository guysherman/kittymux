import { createContext } from '@guysherman/treecat';
import { WindowListEntry } from '../../connectors/kitty';

export enum MainScreenMode {
  Navigate = 'NAVIGATE',
  Rename = 'RENAME',
  Command = 'COMMAND',
  QuickNav = 'QUICK_NAV',
}

export const DefaultMainScreenMode = MainScreenMode.Navigate;

export interface MainScreenState {
  entries: WindowListEntry[];
  selectedIndex: number;
  mode: MainScreenMode;
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
  },
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  dispatch: (action: any): void => {},
});
