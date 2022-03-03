import { DefaultMainScreenMode } from '../../../models/MainScreen';
import { MainScreenActions } from '../store/reducer';

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const processCommand = (command: string, dispatch: (action: any) => void): void => {
  switch (command) {
    case 'q':
      process.exit(0);
    default:
      console.error('unknown command', { command });
      dispatch({ type: MainScreenActions.SetMode, payload: DefaultMainScreenMode });
  }
};
