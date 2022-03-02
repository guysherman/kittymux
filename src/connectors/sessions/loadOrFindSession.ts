import { kittyCommand, listWindows, processWindowList, WindowListEntryType } from '../kitty';
import loadSession from './loadSession';

const loadOrFindSession = async (sessionName: string): Promise<void> => {
  const windowList = await listWindows();
  const entries = processWindowList(windowList);
  const tab = entries.find((entry) => entry.title === sessionName && entry.type === WindowListEntryType.Tab);

  if (tab) {
    await kittyCommand(['focus-tab', '-m', `id:${tab.id}`]);
    return;
  }

  await loadSession(sessionName);
};

export default loadOrFindSession;
