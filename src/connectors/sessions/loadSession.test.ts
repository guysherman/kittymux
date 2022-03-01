import * as fs from 'fs';
import { KittyTab, WindowListEntryType } from '../../models/Kitty';
import { serialiseSession } from './serialiseSession';
import { kittyCommand, createWindow, sendCommand } from '../kitty';

jest.mock('fs');
const mockedFs = fs as jest.Mocked<typeof fs>;

jest.mock('../kitty');

const mockKittyCommand = kittyCommand as jest.MockedFunction<typeof kittyCommand>;
const mockCreateWindow = createWindow as jest.MockedFunction<typeof createWindow>;
const mockSendCommand = sendCommand as jest.MockedFunction<typeof sendCommand>;

import loadSession from './loadSession';

describe('serialiseSession', () => {
  beforeEach(() => {
    jest.resetAllMocks();
  });

  it('should load a session correctly', async () => {
    mockedFs.readFileSync.mockReturnValue(
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
        tabTitle: 'test tab',
        cwd: '/home/tester',
      },
    ]);
    expect(mockSendCommand.mock.calls[1]).toEqual([['/usr/bin/sh'], 2]);

    expect(mockCreateWindow.mock.calls[2]).toEqual([
      'km',
      {
        newTab: false,
        tabTitle: 'test tab',
        cwd: '/home/tester',
      },
    ]);
    expect(mockSendCommand.mock.calls[2]).toEqual([['/home/tester/.local/bin/km'], 3]);
  });
});
