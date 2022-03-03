import * as blessed from 'blessed';
import { focusEntry } from '../../../connectors/kitty';
import { MainScreenState, DefaultMainScreenMode, MainScreenMode } from '../store/model';
import { MainScreenActions } from '../store/reducer';
import { filterEntries } from '../store/scopeFilter';

export const processQuickNavKeypress = (
  state: MainScreenState,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  dispatch: (action: any) => void,
  key: blessed.Widgets.Events.IKeyEventArg,
) => {
  const { mode } = state;

  if (key.full === 'escape') {
    dispatch({ type: MainScreenActions.SetMode, payload: DefaultMainScreenMode });
    return;
  }

  switch (mode) {
    case MainScreenMode.QuickNav:
      actionQuickNavKey(state, dispatch, key);
      break;
    case MainScreenMode.SetQuickNav:
      setQuickNavKey(state, dispatch, key);
      break;
    default:
      console.error('quickNavKeyPress - unknown mode', { mode });
  }
};

const actionQuickNavKey = (
  state: MainScreenState,
  dispatch: (action: any) => void,
  key: blessed.Widgets.Events.IKeyEventArg,
) => {
  const { quickNavKeys, entries } = state;

  const entryHandles = quickNavKeys[key.full];
  if (entryHandles?.length) {
    for (let i = 0; i < entryHandles.length; i++) {
      const entryHandle = entryHandles[i];
      const entry = filterEntries('tab', entries).find(
        (entry) => entry.id === entryHandle.id && entry.type === entryHandle.type,
      );

      if (entry) {
        focusEntry(entry);
        return;
      }
    }
  }

  dispatch({ type: MainScreenActions.SetMode, payload: DefaultMainScreenMode });
};

const setQuickNavKey = (
  state: MainScreenState,
  dispatch: (action: any) => void,
  key: blessed.Widgets.Events.IKeyEventArg,
) => {
  const { quickNavKeys, entries, selectedIndex } = state;
  const selectedEntry = entries[selectedIndex];

  const validateKey = /^[a-zA-Z1-9]$/;
  if (key.full.match(validateKey)?.length !== 1) {
    dispatch({ type: MainScreenActions.SetMode, payload: DefaultMainScreenMode });
    return;
  }

  if (selectedEntry) {
    const { id, type } = selectedEntry;
    const newQuickNavKeys = { ...quickNavKeys };

    const existingEntry = Object.entries(newQuickNavKeys).find(([, handles]) =>
      handles.find((handle) => handle.id === id && handle.type === type),
    );

    // remove the existing entry from the key it was found under
    if (existingEntry) {
      newQuickNavKeys[existingEntry[0]] = newQuickNavKeys[existingEntry[0]].filter(
        (handle) => handle.id !== id && handle.type !== type,
      );
    }

    const entriesForKey = newQuickNavKeys[key.full] ?? [];
    newQuickNavKeys[key.full] = [...entriesForKey, { id, type }];
    dispatch({ type: MainScreenActions.SetQuickNav, payload: newQuickNavKeys });
    dispatch({ type: MainScreenActions.SetMode, payload: DefaultMainScreenMode });
  }
};
