import { createWindow, sendCommand, stateDir } from '../kitty';
import { readFileSync } from 'fs';
import { Session } from '../../models/Sessions';

const loadSession = async (sessionName: string): Promise<void> => {
  const sessionPath = `${stateDir}/kittymux/${sessionName}.json`;
  const sessionText = readFileSync(sessionPath, 'utf8');
  const sessionData: Session = JSON.parse(sessionText || '{}');

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
  }

  return;
};

export default loadSession;
