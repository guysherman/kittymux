import { WindowListEntry } from '../../connectors/kitty';
import { MainScreenState } from './model';

export enum MainScreenActions {
  SetSelectedIndex = 'SET_SELECTED_INDEX',
  SetEntries = 'SET_ENTRIES',
  SetIsEditingName = 'SET_IS_EDITING_NAME',
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const mainScreenReducer = (state: MainScreenState, action: { type: string; payload: any }): MainScreenState => {
  const { type } = action;
  console.error('mainScreenReducer', { action: JSON.stringify(action) });
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

export const moveUp = (state: MainScreenState, dispatch: (action: any) => void) => {
  const { entries, selectedIndex } = state;
  const newIndex = Math.min(entries.length, selectedIndex + 1);
  dispatch({ type: MainScreenActions.SetSelectedIndex, payload: newIndex });
};
