export {
  KittyOsWindow,
  KittyTab,
  KittyWindow,
  KittyForegroundProcessHandle,
  ExecError,
  WindowListEntry,
  WindowListEntryType,
} from './model';
export { listWindows } from './listWindows';
export { processWindowList } from './processWindowList';
export { focusEntry } from './focusEntry';
export { closeEntry } from './closeEntry';
export { renameEntry } from './renameEntry';
export { default as kittyCommand } from './kittyCommand';
export { default as createWindow } from './createWindow';
export { default as sendCommand } from './sendCommand';
export const stateDir = process.env.KITTYMUX_STATE_DIR ?? process.env.XDG_STATE_HOME ?? '~/.local/state';
