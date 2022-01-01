import { createContext } from '@guysherman/treecat';
import { WindowListEntry } from '../../connectors/kitty';

export interface MainScreenState {
  entries: WindowListEntry[];
  isEditingName: boolean;
  selectedIndex: number;
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
    isEditingName: false,
  },
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  dispatch: (action: any): void => {},
});
