import * as blessed from 'blessed';
import { focusEntry } from '../../connectors/kitty';
import { MainScreenState, DefaultMainScreenMode, MainScreenMode } from './model';
import { MainScreenActions } from './reducer';

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

  const entryHandle = quickNavKeys[key.full];
  if (entryHandle) {
    const entry = entries.find((entry) => entry.id === entryHandle.id && entry.type === entryHandle.type);
    if (entry) {
      focusEntry(entry);
    }
  } else {
    dispatch({ type: MainScreenActions.SetMode, payload: DefaultMainScreenMode });
  }
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

    const existingEntry = Object.entries(newQuickNavKeys).find(
      ([, handle]) => handle.id === id && handle.type === type,
    );
    if (existingEntry) {
      delete newQuickNavKeys[existingEntry[0]];
    }

    newQuickNavKeys[key.full] = { id, type };
    dispatch({ type: MainScreenActions.SetQuickNav, payload: newQuickNavKeys });
    dispatch({ type: MainScreenActions.SetMode, payload: DefaultMainScreenMode });
  }
};
