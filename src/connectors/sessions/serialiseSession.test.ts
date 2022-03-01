import * as fs from 'fs';
import { KittyTab, WindowListEntryType } from '../../models/Kitty';
import { serialiseSession } from './serialiseSession';
jest.mock('fs');

const mockedFs = fs as jest.Mocked<typeof fs>;

describe('serialiseSession', () => {
  it('should serialise a session correctly', async () => {
    const tab: KittyTab = {
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
    };

    const entry = {
      id: 1,
      text: 'test tab',
      type: WindowListEntryType.Tab,
      title: 'test tab',
      isFocused: true,
      tabIsFocused: true,
      osWindowIsFocused: true,
      kittyTab: tab,
    };

    const quickNavKeys = {
      a: [{ id: 1, type: WindowListEntryType.Tab }],
      f: [{ id: 1, type: WindowListEntryType.Window }],
      t: [{ id: 2, type: WindowListEntryType.Window }],
      d: [{ id: 3, type: WindowListEntryType.Window }],
    };

    await serialiseSession(entry, quickNavKeys);

    expect(mockedFs.writeFileSync.mock.calls[0][1]).toEqual(
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
              title: 'nvim .',
              cwd: '/home/tester',
              shortcutKey: 't',
              foregroundProcess: {
                cwd: '/home/tester',
                args: ['/usr/bin/sh'],
              },
            },
            {
              title: 'nvim .',
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
  });
});
