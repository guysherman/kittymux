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
    entries: [],
    selectedIndex: 0,
    isEditingName: false,
  },
  // eslint-disable-next-line @typescript-eslint/no-empty-function
  dispatch: () => {},
});

export enum MainScreenActions {
  SetSelectedIndex = 'SET_SELECTED_INDEX',
  SetEntries = 'SET_ENTRIES',
  SetIsEditingName = 'SET_IS_EDITING_NAME',
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const mainScreenReducer = (state: MainScreenState, action: { type: string; payload: any }): MainScreenState => {
  const { type } = action;
  console.error('mainScreenReducer', { action });
  switch (type) {
    case MainScreenActions.SetSelectedIndex:
      return {
        ...state,
        selectedIndex: action.payload as number,
      };
    case MainScreenActions.SetEntries:
      return {
        ...state,
        entries: action.payload as WindowListEntry[],
      };
    case MainScreenActions.SetIsEditingName:
      return {
        ...state,
        isEditingName: action.payload as boolean,
      };
    default:
      return state;
  }
};
