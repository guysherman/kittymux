import createWindow from './createWindow';
import kittyCommand from './kittyCommand';
jest.mock('./kittyCommand');

const mockKittyCommand = kittyCommand as jest.MockedFunction<typeof kittyCommand>;

describe('createWindow', () => {
  beforeEach(() => {
    jest.resetAllMocks();
  });

  it('should issue the correct command for a window in the current tab', async () => {
    mockKittyCommand.mockResolvedValue('99');
    const id = await createWindow('test title');

    expect(mockKittyCommand.mock.calls[0][0]).toEqual(['new-window', '--title', '"test title"']);
    expect(id).toEqual(99);
  });

  it('should issue the correct command for a window in the specified tab', async () => {
    mockKittyCommand.mockResolvedValue('99');
    const id = await createWindow('test title', { tabId: 5 });

    expect(mockKittyCommand.mock.calls[0][0]).toEqual(['new-window', '--title', '"test title"', '-m', 'id:5']);
    expect(id).toEqual(99);
  });

  it('should issue the correct command for a window in a new tab', async () => {
    mockKittyCommand.mockResolvedValue('99');
    const id = await createWindow('test title', { newTab: true, tabTitle: 'test tab' });

    expect(mockKittyCommand.mock.calls[0][0]).toEqual([
      'new-window',
      '--title',
      '"test title"',
      '--new-tab',
      '--tab-title',
      '"test tab"',
    ]);
    expect(id).toEqual(99);
  });

  it('should issue the correct command for creating a window with a specified cwd', async () => {
    mockKittyCommand.mockResolvedValue('99');
    const id = await createWindow('test title', { cwd: '/foo' });

    expect(mockKittyCommand.mock.calls[0][0]).toEqual(['new-window', '--title', '"test title"', '--cwd', '/foo']);
    expect(id).toEqual(99);
  });
});
