export interface KittyForegroundProcessHandle {
  pid: number;
  cwd: string;
  cmdline: string[];
}

export interface KittyWindow {
  id: number;
  is_focused: boolean;
  is_self: boolean;
  lines: number;
  pid: number;
  title: string;
  cwd: string;
  cmdline: string[];
  env: Record<string, string>;
  foreground_processes: KittyForegroundProcessHandle[];
}

export interface KittyTab {
  active_window_history: number[];
  id: number;
  is_focused: boolean;
  layout: string;
  title: string;
  windows: KittyWindow[];
}

export interface KittyOsWindow {
  id: number;
  is_focused: boolean;
  platform_window_id: number;
  tabs: KittyTab[];
}

export class ExecError extends Error {
  code: number;
  name: string;

  constructor(code: number, message: string) {
    super(message);

    this.code = code;
    this.name = 'ExecError';

    Object.setPrototypeOf(this, ExecError.prototype);
  }
}

export enum WindowListEntryType {
  None = 'KITTY_NONE',
  OsWindow = 'KITTY_OS_WINDOW',
  Tab = 'KITTY_TAB',
  Window = 'KITTY_WINDOW',
}

export interface WindowListEntry {
  id: number;
  text: string;
  type: WindowListEntryType;
  pid?: number;
  cwd?: string;
  cmdline?: string;
  title?: string;
  isFocused: boolean;
  tabIsFocused: boolean;
  osWindowIsFocused: boolean;
}
