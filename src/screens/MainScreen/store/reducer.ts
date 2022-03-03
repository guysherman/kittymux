import { WindowListEntry, stateDir } from '../../../connectors/kitty';
import { persistedReducer, restoreState } from '../../../connectors/settings';
import { DefaultMainScreenMode, MainScreenMode, MainScreenState, QuickNavHandle } from '../../../models/MainScreen';

export enum MainScreenActions {
  SetSelectedIndex = 'SET_SELECTED_INDEX',
  SetEntries = 'SET_ENTRIES',
  SetMode = 'SET_MODE',
  SetQuickNav = 'SET_QUICK_NAV',
  PruneQuickNav = 'PRUNE_QUICK_NAV',
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const innerMainScreenReducer = (state: MainScreenState, action: { type: string; payload: any }): MainScreenState => {
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
        quickNavKeys: action.payload as Record<string, QuickNavHandle[]>,
      };
    case MainScreenActions.PruneQuickNav:
      return pruneQuickNav(state);
    default:
      return state;
  }
};

export const QUICKNAVS_STORE_PATH = `${stateDir}/kittymux/quicknavs.json`;
export const mainScreenReducer = persistedReducer(innerMainScreenReducer, QUICKNAVS_STORE_PATH, ['quickNavKeys']);
export const getDefaultState = () =>
  restoreState(
    {
      entries: [] as WindowListEntry[],
      selectedIndex: 0,
      mode: DefaultMainScreenMode,
      quickNavKeys: {} as Record<string, QuickNavHandle[]>,
    },
    QUICKNAVS_STORE_PATH,
    ['quickNavKeys'],
  );

const pruneQuickNav = (state: MainScreenState): MainScreenState => {
  const { entries, quickNavKeys } = state;
  const entryHandles = entries.map((entry) => ({ id: entry.id, type: entry.type }));

  const newQuickNavKeys = Object.fromEntries(
    Object.entries(quickNavKeys)
      .map(([key, handles]) => [
        key,
        handles.filter(
          (oldHandle) => !!entryHandles.find((handle) => oldHandle.id === handle.id && oldHandle.type === handle.type),
        ),
      ])
      .filter(([, handles]) => handles.length),
  );

  return { ...state, quickNavKeys: newQuickNavKeys };
};
