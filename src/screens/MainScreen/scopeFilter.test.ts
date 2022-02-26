import { KittyOsWindow, processWindowList } from '../../connectors/kitty';
import { filterEntries } from './scopeFilter';

describe('scopeFilter', () => {
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
          title: 'test tab 1',
          windows: [
            {
              id: 1,
              is_focused: false,
              is_self: false,
              lines: 41,
              pid: 54311,
              title: 'nvim . 1',
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
              title: 'nvim . 2',
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
              title: 'nvim . 3',
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
        {
          id: 1,
          active_window_history: [3, 2, 1],
          is_focused: false,
          layout: 'stack',
          title: 'test tab 2',
          windows: [
            {
              id: 1,
              is_focused: false,
              is_self: false,
              lines: 41,
              pid: 54311,
              title: 'nvim . 4',
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
              title: 'nvim . 5',
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
              is_focused: false,
              is_self: false,
              lines: 41,
              pid: 404040,
              title: 'nvim . 6',
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
    {
      id: 2,
      is_focused: false,
      platform_window_id: 80086,
      tabs: [
        {
          id: 1,
          active_window_history: [3, 2, 1],
          is_focused: false,
          layout: 'stack',
          title: 'test tab 3',
          windows: [
            {
              id: 1,
              is_focused: false,
              is_self: false,
              lines: 41,
              pid: 54311,
              title: 'nvim . 7',
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
              title: 'nvim . 8',
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
              is_focused: false,
              is_self: true,
              lines: 41,
              pid: 404040,
              title: 'nvim . 9',
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
        {
          id: 1,
          active_window_history: [3, 2, 1],
          is_focused: false,
          layout: 'stack',
          title: 'test tab 4',
          windows: [
            {
              id: 1,
              is_focused: false,
              is_self: false,
              lines: 41,
              pid: 54311,
              title: 'nvim . 10',
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
              title: 'nvim . 11',
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
              is_focused: false,
              is_self: true,
              lines: 41,
              pid: 404040,
              title: 'nvim . 12',
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

  it('should return everything', () => {
    const entries = processWindowList(windowList);
    const filteredEntries = filterEntries('all', entries);

    expect(filteredEntries).toHaveLength(18);
  });

  it('should return only current tab', () => {
    const entries = processWindowList(windowList);
    const filteredEntries = filterEntries('tab', entries);

    expect(filteredEntries).toHaveLength(5);
    expect(filteredEntries[2].title).toEqual('nvim . 1');
  });

  it('should return only current os window', () => {
    const entries = processWindowList(windowList);
    const filteredEntries = filterEntries('window', entries);

    expect(filteredEntries).toHaveLength(9);
    expect(filteredEntries[5].title).toEqual('test tab 2');
  });
});
