import { writeFileSync } from 'fs';
import { QuickNavHandle } from '../../models/MainScreen';
import { WindowListEntry, WindowListEntryType, stateDir } from '../kitty';

export const serialiseSession = (entry: WindowListEntry, quickNavKeys: Record<string, QuickNavHandle[]>): void => {
  const { kittyTab } = entry;

  const windows = kittyTab?.windows.map(({ title, cwd, foreground_processes, id }) => {
    const entry = Object.entries(quickNavKeys).find(([, handles]) =>
      handles.find((handle) => handle.id === id && handle.type === WindowListEntryType.Window),
    );
    const foregroundProcess = foreground_processes.length
      ? {
          cwd: foreground_processes[0].cwd,
          args: foreground_processes[0].cmdline,
        }
      : undefined;

    return {
      title,
      cwd,
      shortcutKey: entry?.[0] ?? '',
      foregroundProcess,
    };
  });

  const tabEntry = Object.entries(quickNavKeys).find(([, handles]) =>
    handles.find((handle) => handle.id === entry.id && handle.type === WindowListEntryType.Tab),
  );

  const session = {
    title: entry.title,
    shortcutKey: tabEntry?.[0],
    windows,
    layout: kittyTab?.layout,
  };

  const sessionPath = `${stateDir}/kittymux/${entry.title}.json`;

  writeFileSync(sessionPath, JSON.stringify(session, null, 4));
};
