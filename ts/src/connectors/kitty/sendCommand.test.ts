import sendCommand from './sendCommand';
import kittyCommand from './kittyCommand';
jest.mock('./kittyCommand');

const mockKittyCommand = kittyCommand as jest.MockedFunction<typeof kittyCommand>;

describe('sendCommand', () => {
  beforeEach(() => {
    jest.resetAllMocks();
  });

  it('should generate correct string including \\n at the end', async () => {
    await sendCommand(['nvim', '.'], 5);

    expect(mockKittyCommand.mock.calls[0][0]).toEqual(['send-text', '-m', `id:5`, "'nvim .\\n'"]);
  });

  it('should surround multi-word arguments in double-quotes', async () => {
    await sendCommand(['echo', 'foo bar baz', '>', 'out.txt'], 23);

    expect(mockKittyCommand.mock.calls[0][0]).toEqual([
      'send-text',
      '-m',
      'id:23',
      `'echo "foo bar baz" > out.txt\\n'`,
    ]);
  });
});
