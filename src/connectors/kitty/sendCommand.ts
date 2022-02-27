import kittyCommand from './kittyCommand';

const sendCommand = (args: string[], windowId: number): Promise<unknown> => {
  const quotedArgs = args.map((arg) => (arg.includes(' ') ? `"${arg}"` : arg));
  const commandString = quotedArgs.join(' ');

  return kittyCommand(['send-text', '-m', `id:${windowId}`, `${commandString}\n`]);
};

export default sendCommand;
