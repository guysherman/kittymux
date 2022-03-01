import kittyCommand from './kittyCommand';
import { KittyOsWindow } from '../../models/Kitty';

export const listWindows = (): Promise<KittyOsWindow[]> => {
  const args = ['ls'];
  return kittyCommand(args).then((stdout) => JSON.parse(stdout as any));
};
