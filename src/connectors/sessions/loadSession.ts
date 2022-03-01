import { createWindow, sendCommand, stateDir, WindowListEntryType } from '../kitty';
import { readFileSync, writeFileSync } from 'fs';
import { Session } from '../../models/Sessions';

const loadSession = async (sessionName: string): Promise<void> => {
  const storePath = `${stateDir}/kittymux/quicknavs.json`;
  const sessionPath = `${stateDir}/kittymux/${sessionName}.json`;
  const sessionText = readFileSync(sessionPath, 'utf8');
  const sessionData: Session = JSON.parse(sessionText || '{}');

  const storeText = readFileSync(storePath, 'utf8');
  const storeData = JSON.parse(storeText || '{}');
  const { quickNavKeys } = storeData;
  console.log('quickNavKeys  -pre', { quickNavKeys });

  const { windows } = sessionData;
  for (let i = 0; i < windows.length; i++) {
    const window = windows[i];
    const windowId = await createWindow(window.title, {
      newTab: i === 0,
      tabTitle: sessionData.title,
      cwd: window.cwd,
    });

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

  console.log('quickNavKeys', { quickNavKeys });
  writeFileSync(storePath, JSON.stringify({ quickNavKeys }));

  return;
};

export default loadSession;
