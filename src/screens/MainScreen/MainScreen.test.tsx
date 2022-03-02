/** @jsx TreeCat.createElement **/
// eslint-disable-next-line no-unused-vars
import * as TreeCat from '@guysherman/treecat';
import * as blessed from 'blessed';
import * as fs from 'fs';
import { restoreState, persistedReducer } from '../../connectors/settings';
//jest.mock('../../connectors/settings');

import { MainScreen } from '.';

//const mockedRestoreState = restoreState as jest.MockedFunction<typeof restoreState>;
//const mockedPersistedReducer = persistedReducer as jest.MockedFunction<typeof persistedReducer>;

import {
  KittyOsWindow,
  listWindows,
  renameEntry,
  focusEntry,
  WindowListEntry,
  WindowListEntryType,
} from '../../connectors/kitty';
import { QUICKNAVS_STORE_PATH } from './reducer';
jest.mock('../../connectors/kitty', () => {
  const original = jest.requireActual('../../connectors/kitty');

  return {
    __esModule: true,
    ...original,
    listWindows: jest.fn(),
    renameEntry: jest.fn(),
    focusEntry: jest.fn(),
  };
});

const mockedListWindows = listWindows as jest.MockedFunction<typeof listWindows>;
const mockedRenameEntry = renameEntry as jest.MockedFunction<typeof renameEntry>;
const mockedFocusEntry = focusEntry as jest.MockedFunction<typeof focusEntry>;

describe('MainScreen', () => {
  let rootScreen: blessed.Widgets.Screen;
  let outStream: fs.WriteStream;
  let inStream: fs.ReadStream;

  const windowList: KittyOsWindow[] = [
    {
      id: 1,
      is_focused: true,
      platform_window_id: 80085,
      tabs: [
        {
          id: 1,
          active_window_history: [3, 2, 1],
          is_focused: true,
          layout: 'stack',
          title: 'test tab',
          windows: [
            {
              id: 1,
              is_focused: false,
              is_self: false,
              lines: 41,
              pid: 54311,
              title: 'nvim .',
              cwd: '/home/tester',
              cmdline: ['/usr/bin/sh'],
              env: {},
              foreground_processes: [
                {
                  pid: 1234,
                  cwd: '/home/tester',
                  cmdline: ['nvim', '.'],
                },
              ],
            },
            {
              id: 2,
              is_focused: false,
              is_self: false,
              lines: 41,
              pid: 38389,
              title: 'nvim .',
              cwd: '/home/tester',
              cmdline: ['/usr/bin/sh'],
              env: {},
              foreground_processes: [
                {
                  pid: 38389,
                  cwd: '/home/tester',
                  cmdline: ['/usr/bin/sh'],
                },
              ],
            },
            {
              id: 3,
              is_focused: true,
              is_self: true,
              lines: 41,
              pid: 404040,
              title: 'nvim .',
              cwd: '/home/tester',
              cmdline: ['/usr/bin/sh'],
              env: {},
              foreground_processes: [
                {
                  pid: 505050,
                  cwd: '/home/tester',
                  cmdline: ['/home/tester/.local/bin/km'],
                },
              ],
            },
          ],
        },
      ],
    },
  ];

  beforeEach(async () => {
    fs.rmSync(QUICKNAVS_STORE_PATH, { force: true });
    jest.useFakeTimers();
    outStream = fs.createWriteStream('./.scratch/out');
    inStream = fs.createReadStream('/dev/random');
    rootScreen = TreeCat.blessed.screen({ output: inStream, input: outStream });
  });

  afterEach(() => {
    rootScreen.destroy();
    outStream.close();
    inStream.close();
    jest.useRealTimers();
    jest.resetAllMocks();
  });

  it('should contain a list and box with three boxes', async () => {
    let res: (value: void | PromiseLike<void>) => void;
    const p = new Promise<void>((resolve) => {
      res = resolve;
    });

    mockedListWindows.mockImplementation(() => {
      res();
      return Promise.resolve(windowList);
    });

    const tree = <MainScreen scope={'all'} />;
    TreeCat.render(tree, rootScreen);

    jest.runOnlyPendingTimers();
    await p;
    jest.runOnlyPendingTimers();

    const list = rootScreen.children[0].children[0] as blessed.Widgets.ListElement;
    expect(list.getItem(0).content).toEqual('kitty:1');
  });

  it('should select next item when j is pressed', async () => {
    let res: (value: void | PromiseLike<void>) => void;
    const p = new Promise<void>((resolve) => {
      res = resolve;
    });

    mockedListWindows.mockImplementation(() => {
      res();
      return Promise.resolve(windowList);
    });

    const tree = <MainScreen scope={'all'} />;
    TreeCat.render(tree, rootScreen);

    jest.runOnlyPendingTimers();
    await p;
    jest.runOnlyPendingTimers();

    const list = rootScreen.children[0].children[0] as blessed.Widgets.ListElement;
    list?.emit('keypress', 'j', { sequence: 'j', name: 'j', ctrl: false, meta: false, shift: true, full: 'j' });
    jest.runOnlyPendingTimers();

    const box = rootScreen.children[0].children[1] as blessed.Widgets.BoxElement;
    const {
      children: [title],
    } = box;

    expect((title as blessed.Widgets.BoxElement).content).toEqual('test tab');
  });

  it('should call renameEntry with entered name', async () => {
    let res: (value: void | PromiseLike<void>) => void;
    const p = new Promise<void>((resolve) => {
      res = resolve;
    });

    mockedListWindows.mockImplementation(() => {
      res();
      return Promise.resolve(windowList);
    });

    mockedRenameEntry.mockResolvedValue(undefined);

    const tree = <MainScreen scope={'all'} />;
    TreeCat.render(tree, rootScreen);
    jest.runOnlyPendingTimers();
    await p;
    jest.runOnlyPendingTimers();

    const list = rootScreen.children[0].children[0] as blessed.Widgets.ListElement;
    list?.emit('keypress', 'j', { sequence: 'j', name: 'j', ctrl: false, meta: false, shift: true, full: 'j' });
    jest.runOnlyPendingTimers();

    list?.emit('keypress', 'a', { sequence: 'a', name: 'a', ctrl: false, meta: false, shift: true, full: 'a' });
    jest.runOnlyPendingTimers();

    const box = rootScreen.children[0].children[1] as blessed.Widgets.BoxElement;
    const {
      children: [, , , textInput],
    } = box;

    expect(textInput as blessed.Widgets.TextboxElement).toBeTruthy();
    (textInput as blessed.Widgets.TextboxElement)?.emit('submit', 'test2');
    jest.runOnlyPendingTimers();

    expect(mockedRenameEntry.mock.calls[0][1]).toEqual('test2');
  });

  it('should store and action quick key', async () => {
    process.env.KITTYMUX_STATE_DIR = './.scratch';
    let res: (value: void | PromiseLike<void>) => void;
    const p = new Promise<void>((resolve) => {
      res = resolve;
    });

    mockedListWindows.mockImplementation(() => {
      res();
      return Promise.resolve(windowList);
    });

    const tree = <MainScreen scope={'all'} />;
    TreeCat.render(tree, rootScreen);
    jest.runOnlyPendingTimers();
    await p;
    jest.runOnlyPendingTimers();

    const list = rootScreen.children[0].children[0] as blessed.Widgets.ListElement;
    const box = rootScreen.children[0] as blessed.Widgets.BoxElement;
    list?.emit('keypress', 'j', { sequence: 'j', name: 'j', ctrl: false, meta: false, shift: true, full: 'j' });
    jest.runOnlyPendingTimers();

    list?.emit('keypress', 'm', { sequence: 'm', name: 'm', ctrl: false, meta: false, shift: true, full: 'm' });
    jest.runOnlyPendingTimers();

    box?.emit('keypress', 'a', { sequence: 'a', name: 'a', ctrl: false, meta: false, shift: true, full: 'a' });
    jest.runOnlyPendingTimers();

    list?.emit('keypress', 'k', { sequence: 'k', name: 'k', ctrl: false, meta: false, shift: true, full: 'k' });
    jest.runOnlyPendingTimers();

    list?.emit('keypress', "'", { sequence: "'", name: "'", ctrl: false, meta: false, shift: true, full: "'" });
    jest.runOnlyPendingTimers();

    box?.emit('keypress', 'a', { sequence: 'a', name: 'a', ctrl: false, meta: false, shift: true, full: 'a' });
    jest.runOnlyPendingTimers();

    expect(mockedFocusEntry.mock.calls[0][0].id).toEqual(windowList[0].tabs[0].id);
    expect(mockedFocusEntry.mock.calls[0][0].type).toEqual(WindowListEntryType.Tab);
  });
});
