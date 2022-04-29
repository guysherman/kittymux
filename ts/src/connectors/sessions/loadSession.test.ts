import * as fs from 'fs';
import { createWindow, sendCommand, listWindows, KittyOsWindow } from '../kitty';

jest.mock('fs');
const mockedFs = fs as jest.Mocked<typeof fs>;

jest.mock('../kitty');

const mockedListWindows = listWindows as jest.MockedFunction<typeof listWindows>;
const mockCreateWindow = createWindow as jest.MockedFunction<typeof createWindow>;
const mockSendCommand = sendCommand as jest.MockedFunction<typeof sendCommand>;

import loadSession from './loadSession';

describe('serialiseSession', () => {
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
    jest.resetAllMocks();
  });

  it('should load a session correctly', async () => {
    mockedListWindows.mockResolvedValue(windowList);
    mockedFs.readFileSync.mockReturnValueOnce(
      JSON.stringify(
        {
          title: 'test tab',
          shortcutKey: 'a',
          windows: [
            {
              title: 'nvim .',
              cwd: '/home/tester',
              shortcutKey: 'f',
              foregroundProcess: {
                cwd: '/home/tester',
                args: ['nvim', '.'],
              },
            },
            {
              title: 'sh',
              cwd: '/home/tester',
              shortcutKey: 't',
              foregroundProcess: {
                cwd: '/home/tester',
                args: ['/usr/bin/sh'],
              },
            },
            {
              title: 'km',
              cwd: '/home/tester',
              shortcutKey: 'd',
              foregroundProcess: {
                cwd: '/home/tester',
                args: ['/home/tester/.local/bin/km'],
              },
            },
          ],
          layout: 'stack',
        },
        null,
        4,
      ),
    );

    mockedFs.readFileSync.mockReturnValueOnce('{ "quickNavKeys": [] }');

    mockCreateWindow.mockResolvedValueOnce(1);
    mockCreateWindow.mockResolvedValueOnce(2);
    mockCreateWindow.mockResolvedValueOnce(3);

    await loadSession('test tab');

    expect(mockCreateWindow.mock.calls[0]).toEqual([
      'nvim .',
      {
        newTab: true,
        tabTitle: 'test tab',
        cwd: '/home/tester',
      },
    ]);
    expect(mockSendCommand.mock.calls[0]).toEqual([['nvim', '.'], 1]);

    expect(mockCreateWindow.mock.calls[1]).toEqual([
      'sh',
      {
        newTab: false,
        tabId: 1,
        tabTitle: undefined,
        cwd: '/home/tester',
      },
    ]);
    expect(mockSendCommand.mock.calls[1]).toEqual([['/usr/bin/sh'], 2]);

    expect(mockCreateWindow.mock.calls[2]).toEqual([
      'km',
      {
        newTab: false,
        tabId: 1,
        tabTitle: undefined,
        cwd: '/home/tester',
      },
    ]);
    expect(mockSendCommand.mock.calls[2]).toEqual([['/home/tester/.local/bin/km'], 3]);
  });
});
