import * as blessed from 'blessed';
import { WindowListEntry, WindowListEntryType, focusEntry, closeEntry } from '../../connectors/kitty';
import { MainScreenState, MainScreenMode } from './model';
import { MainScreenActions } from './reducer';
import { refreshWindowList } from './refreshWindowList';

interface KeyHandler {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  (dispatch: (action: any) => void, state: MainScreenState): void;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const enterCommandMode = (dispatch: (action: any) => void, _state: MainScreenState) => {
  dispatch({ type: MainScreenActions.SetMode, payload: MainScreenMode.Command });
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const nextEntry = (dispatch: (action: any) => void, state: MainScreenState) => {
  const { selectedIndex, entries } = state;
  const newIndex = Math.min(entries.length - 1, selectedIndex + 1);
  dispatch({ type: MainScreenActions.SetSelectedIndex, payload: newIndex });
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const previousEntry = (dispatch: (action: any) => void, state: MainScreenState) => {
  const { selectedIndex } = state;
  const newIndex = Math.max(0, selectedIndex - 1);
  dispatch({ type: MainScreenActions.SetSelectedIndex, payload: newIndex });
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const nextTab = (dispatch: (action: any) => void, state: MainScreenState) => {
  const { entries, selectedIndex } = state;
  const followingEntries = entries.slice(selectedIndex + 1);
  const nextTab: number = followingEntries.findIndex(
    (entry: WindowListEntry) => entry.type === WindowListEntryType.Tab,
  );
  const nextIndex = selectedIndex + 1 + nextTab;
  dispatch({
    type: MainScreenActions.SetSelectedIndex,
    payload: nextIndex > entries.length - 1 ? selectedIndex : nextIndex,
  });
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const previousTab = (dispatch: (action: any) => void, state: MainScreenState) => {
  const { entries, selectedIndex } = state;
  const precedingEntries = entries.slice(0, selectedIndex);
  const nextTab: number = precedingEntries
    .reverse()
    .findIndex((entry: WindowListEntry) => entry.type === WindowListEntryType.Tab);
  const nextIndex = selectedIndex - 1 - nextTab;
  dispatch({ type: MainScreenActions.SetSelectedIndex, payload: nextIndex <= 0 ? selectedIndex : nextIndex });
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const focusSelected = (_dispatch: (action: any) => void, state: MainScreenState) => {
  const { selectedIndex, entries } = state;
  const entry = entries[selectedIndex];
  focusEntry(entry);
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const closeSelected = (dispatch: (action: any) => void, state: MainScreenState) => {
  const { selectedIndex, entries } = state;
  const entry = entries[selectedIndex];
  closeEntry(entry)
    .then(() => refreshWindowList(dispatch))
    .then((windowList: WindowListEntry[]) => {
      dispatch({
        type: MainScreenActions.SetSelectedIndex,
        payload: Math.max(0, Math.min(selectedIndex, windowList.length - 1)),
      });
    });
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const renameSelected = (dispatch: (action: any) => void, _state: MainScreenState) => {
  dispatch({ type: MainScreenActions.SetMode, payload: MainScreenMode.Rename });
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const enterSetQuickNavMode = (dispatch: (action: any) => void, _state: MainScreenState) => {
  dispatch({ type: MainScreenActions.SetMode, payload: MainScreenMode.SetQuickNav });
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const enterQuickNavMode = (dispatch: (action: any) => void, _state: MainScreenState) => {
  dispatch({ type: MainScreenActions.SetMode, payload: MainScreenMode.QuickNav });
};

const commandMap: Record<string, KeyHandler> = {
  j: nextEntry,
  k: previousEntry,
  'S-j': nextTab,
  'S-k': previousTab,
  enter: focusSelected,
  x: closeSelected,
  a: renameSelected,
  ':': enterCommandMode,
  m: enterSetQuickNavMode,
  "'": enterQuickNavMode,
};

export const processListKeyPress = (
  state: MainScreenState,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  dispatch: (action: any) => void,
  _ch: string,
  key: blessed.Widgets.Events.IKeyEventArg,
): void => {
  commandMap[key.full](dispatch, state);
};
