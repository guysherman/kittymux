import { listWindows, KittyOsWindow, processWindowList } from '../../connectors/kitty';
import { MainScreenActions } from './reducer';

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const refreshWindowList = (dispatch: (action: any) => void) => {
  return listWindows().then((windowList: KittyOsWindow[]) => {
    const entries = processWindowList(windowList);
    dispatch({ type: MainScreenActions.SetEntries, payload: entries });
    return entries;
  });
};
