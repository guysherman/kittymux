import { listWindows, KittyOsWindow, processWindowList } from '../../../connectors/kitty';
import { MainScreenActions } from '../store/reducer';

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const refreshWindowList = (dispatch: (action: any) => void) => {
  return listWindows().then((windowList: KittyOsWindow[]) => {
    const entries = processWindowList(windowList);
    dispatch({ type: MainScreenActions.SetEntries, payload: entries });
    dispatch({ type: MainScreenActions.PruneQuickNav, payload: undefined });
    return entries;
  });
};
