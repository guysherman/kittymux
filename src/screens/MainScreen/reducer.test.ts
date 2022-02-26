import { WindowListEntryType } from '../../connectors/kitty';
import { MainScreenMode, MainScreenState } from './model';
import { MainScreenActions, mainScreenReducer } from './reducer';

describe('MainScreenReducer', () => {
  it('should prune quick nav keys for entries that nolonger exist', () => {
    const initialState: MainScreenState = {
      entries: [
        {
          id: 1,
          type: WindowListEntryType.Tab,
          text: 'Blah',
          isFocused: true,
          tabIsFocused: true,
          osWindowIsFocused: true,
        },
      ],
      selectedIndex: 0,
      mode: MainScreenMode.Navigate,
      quickNavKeys: {
        a: [{ id: 2, type: WindowListEntryType.Window }],
        b: [{ id: 3, type: WindowListEntryType.Window }],
        c: [{ id: 1, type: WindowListEntryType.Window }],
        d: [{ id: 1, type: WindowListEntryType.Tab }],
      },
    };

    const newState = mainScreenReducer(initialState, { type: MainScreenActions.PruneQuickNav, payload: undefined });

    expect(Object.entries(newState.quickNavKeys)).toHaveLength(1);
    expect(newState.quickNavKeys['d']).toBeTruthy();
  });
});
