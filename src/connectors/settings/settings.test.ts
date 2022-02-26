import * as fs from 'fs';
import { restoreState } from '.';
import { MainScreenMode, MainScreenState } from '../../screens/MainScreen/model';
import { MainScreenActions, mainScreenReducer } from '../../screens/MainScreen/reducer';
import { WindowListEntryType } from '../kitty';
jest.mock('fs');

const mockedFs = fs as jest.Mocked<typeof fs>;

describe('persistedReducer', () => {
  it('should only write chosen keys', () => {
    const state: MainScreenState = {
      entries: [],
      selectedIndex: 0,
      mode: MainScreenMode.Navigate,
      quickNavKeys: { a: [{ id: 1, type: WindowListEntryType.Tab }] },
    };

    mainScreenReducer(state, { type: MainScreenActions.SetMode, payload: MainScreenMode.Command });

    expect(mockedFs.writeFileSync.mock.calls[0][1]).toEqual(JSON.stringify({ quickNavKeys: state.quickNavKeys }));
  });
});

describe('restoreState', () => {
  it('should populate only selectedFields', () => {
    const quickNav = {
      quickNavKeys: { a: [{ id: 1, type: WindowListEntryType.Tab }] },
      someOtherKey: 'foo',
    };
    mockedFs.readFileSync.mockReturnValue(JSON.stringify(quickNav));

    const defaultState: MainScreenState = {
      entries: [],
      selectedIndex: 0,
      mode: MainScreenMode.Navigate,
      quickNavKeys: {},
    };

    const state = restoreState(defaultState, 'somePath', ['quickNavKeys']);

    expect(state.quickNavKeys).toEqual(quickNav.quickNavKeys);
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    expect((state as any).someOtherKey).toBeFalsy();
  });
});
