/** @jsx TreeCat.createElement **/
// eslint-disable-next-line no-unused-vars
import * as TreeCat from '@guysherman/treecat';
import * as blessed from 'blessed';
import * as fs from 'fs';
import { MainScreen } from '.';

import { KittyOsWindow, listWindows } from '../../connectors/kitty';
jest.mock('../../connectors/kitty', () => {
  const original = jest.requireActual('../../connectors/kitty');

  return {
    __esModule: true,
    ...original,
    listWindows: jest.fn(),
  };
});

const mockedListWindows = listWindows as jest.MockedFunction<typeof listWindows>;

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

  beforeEach(() => {
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

    const tree = <MainScreen />;
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

    const tree = <MainScreen />;
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
});
