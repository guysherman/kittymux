export interface Session {
  title: string;
  shortcutKey: string;
  windows: Window[];
  layout: string;
}

export interface Window {
  title: string;
  shortcutKey: string;
  foregroundProcess?: ForegroundProcess;
  cwd: string;
}

export interface ForegroundProcess {
  args: string[];
  cwd: string;
}
