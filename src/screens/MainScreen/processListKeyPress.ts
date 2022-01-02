import { WindowListEntry, WindowListEntryType, focusEntry, closeEntry } from '../../connectors/kitty';
import { MainScreenState, MainScreenMode } from './model';
import { MainScreenActions } from './reducer';

export const processListKeyPress = (
  state: MainScreenState,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  dispatch: (action: any) => void,
  ch: string /*, key: blessed.Widgets.Events.IKeyEventArg*/,
): void => {
  const { entries, selectedIndex } = state;
  if (ch === ':') {
    dispatch({ type: MainScreenActions.SetMode, payload: MainScreenMode.Command });
  } else if (ch === 'j') {
    const newIndex = Math.min(entries.length - 1, selectedIndex + 1);
    dispatch({ type: MainScreenActions.SetSelectedIndex, payload: newIndex });
  } else if (ch === 'k') {
    const newIndex = Math.max(0, selectedIndex - 1);
    dispatch({ type: MainScreenActions.SetSelectedIndex, payload: newIndex });
  } else if (ch === 'J') {
    const followingEntries = entries.slice(selectedIndex + 1);
    const nextTab: number = followingEntries.findIndex(
      (entry: WindowListEntry) => entry.type === WindowListEntryType.Tab,
    );
    const nextIndex = selectedIndex + 1 + nextTab;
    dispatch({
      type: MainScreenActions.SetSelectedIndex,
      payload: nextIndex > entries.length - 1 ? selectedIndex : nextIndex,
    });
  } else if (ch === 'K') {
    const precedingEntries = entries.slice(0, selectedIndex);
    const nextTab: number = precedingEntries
      .reverse()
      .findIndex((entry: WindowListEntry) => entry.type === WindowListEntryType.Tab);
    const nextIndex = selectedIndex - 1 - nextTab;
    dispatch({ type: MainScreenActions.SetSelectedIndex, payload: nextIndex <= 0 ? selectedIndex : nextIndex });
  } else if (ch === '\r') {
    const entry = entries[selectedIndex];
    focusEntry(entry);
  } else if (ch === 'x') {
    const entry = entries[selectedIndex];
    closeEntry(entry).then((windowList: WindowListEntry[]) => {
      dispatch({ type: MainScreenActions.SetEntries, payload: windowList });
      dispatch({
        type: MainScreenActions.SetSelectedIndex,
        payload: Math.max(0, Math.min(selectedIndex, windowList.length - 1)),
      });
    });
  } else if (ch === 'a') {
    dispatch({ type: MainScreenActions.SetMode, payload: MainScreenMode.Rename });
  }
};
