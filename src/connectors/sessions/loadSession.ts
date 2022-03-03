import {
  createWindow,
  kittyCommand,
  KittyTab,
  listWindows,
  sendCommand,
  stateDir,
  WindowListEntryType,
} from '../kitty';
import { readFileSync, writeFileSync } from 'fs';
import { Session } from './model';

const findTab = async (): Promise<KittyTab | undefined> => {
  const windowList = await listWindows();
  const window = windowList.find((window) => window.is_focused);
  if (window) {
    const tab = window.tabs.slice(-1)[0];
    return tab;
  }

  return undefined;
};

const delay = (timeout: number): Promise<void> => {
  const p = new Promise<void>((resolve) => {
    setTimeout(resolve, 3000);
  });
  return p;
};

const loadSession = async (sessionName: string): Promise<void> => {
  const storePath = `${stateDir}/kittymux/quicknavs.json`;
  const sessionPath = `${stateDir}/kittymux/${sessionName}.json`;
  const sessionText = readFileSync(sessionPath, 'utf8');
  const sessionData: Session = JSON.parse(sessionText || '{}');

  const storeText = readFileSync(storePath, 'utf8');
  const storeData = JSON.parse(storeText || '{}');
  const { quickNavKeys } = storeData;

  const { windows } = sessionData;
  let tab: KittyTab | undefined = undefined;
  for (let i = 0; i < windows.length; i++) {
    const window = windows[i];
    // This is a bit gross, but on the first iteration we set newTab, and a title,
    // on the subsequent iterations we set an id.
    const windowId = await createWindow(window.title, {
      newTab: i === 0,
      tabTitle: i === 0 ? sessionData.title : undefined,
      tabId: i !== 0 ? tab?.id : undefined,
      cwd: window.cwd,
    });

    // On the first window, we need to grab the tab, and use it's id.
    if (i === 0) {
      do {
        tab = await findTab();
        if (!tab || tab.title !== sessionData.title || !tab.is_focused) {
          await delay(1000);
          continue;
        }
        await kittyCommand(['focus-tab', '-m', `id:${tab?.id}`]);
      } while (!tab);
    }

    if (window?.foregroundProcess) {
      await sendCommand(window.foregroundProcess.args, windowId);
    }

    if (window.shortcutKey) {
      const { shortcutKey } = window;
      if (!quickNavKeys[shortcutKey]) {
        quickNavKeys[shortcutKey] = [];
      }

      quickNavKeys[shortcutKey].push({ id: windowId, type: WindowListEntryType.Window });
    }
  }

  writeFileSync(storePath, JSON.stringify({ quickNavKeys }));

  return;
};

export default loadSession;
