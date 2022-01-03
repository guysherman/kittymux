import { WindowListEntry } from '../../connectors/kitty';
import { MainScreenMode, MainScreenState, QuickNavHandle } from './model';

export enum MainScreenActions {
  SetSelectedIndex = 'SET_SELECTED_INDEX',
  SetEntries = 'SET_ENTRIES',
  SetMode = 'SET_MODE',
  SetQuickNav = 'SET_QUICK_NAV',
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const mainScreenReducer = (state: MainScreenState, action: { type: string; payload: any }): MainScreenState => {
  const { type } = action;
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
    case MainScreenActions.SetMode:
      return {
        ...state,
        mode: action.payload as MainScreenMode,
      };
    case MainScreenActions.SetQuickNav:
      return {
        ...state,
        quickNavKeys: action.payload as Record<string, QuickNavHandle>,
      };
    default:
      return state;
  }
};

export const moveUp = (state: MainScreenState, dispatch: (action: any) => void) => {
  const { entries, selectedIndex } = state;
  const newIndex = Math.min(entries.length, selectedIndex + 1);
  dispatch({ type: MainScreenActions.SetSelectedIndex, payload: newIndex });
};
