import { exec } from 'child_process';
import { ExecError } from '../../models/Kitty';

const kittyCommand = (args: string[]): Promise<string> => {
  const argString = args.join(' ');
  const command = `kitty @ ${argString}`;

  return new Promise<string>((resolve, reject) => {
    exec(command, (error, stdout, stderror) => {
      if (error) {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        reject(new ExecError((error as any).code, stderror));
      } else {
        resolve(stdout);
      }
    });
  });
};

export default kittyCommand;
