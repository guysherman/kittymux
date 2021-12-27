import { KittyOsWindow, KittyTab, KittyWindow } from '.';
import { WindowListEntry, WindowListEntryType } from './model';

const PRE_INDENT = '    ';
const INDENT = ' └─ ';

const TAB_INDENT = ' ├─ ';
const LAST_TAB_INDENT = ' └─ ';
const WINDOW_INDENT = ' │  ├─ ';
const LAST_WINDOW_INDENT = ' │  └─ ';
const LAST_TAB_WINDOW_INDENT = '    ├─ ';
const LAST_TAB_LAST_WINDOW_INDENT = '    └─ ';

const processWindow = (window: KittyWindow, isLast: boolean, parentIsLast: boolean): WindowListEntry => {
  let indent = '';
  if (parentIsLast) {
    indent = isLast ? LAST_TAB_LAST_WINDOW_INDENT : LAST_TAB_WINDOW_INDENT;
  } else {
    indent = isLast ? LAST_WINDOW_INDENT : WINDOW_INDENT;
  }
  const entry: WindowListEntry = {
    id: window.id,
    text: `${indent}${window.title} (id:${window.id}; pid:${window.pid}) ${window.is_focused ? '*' : ''}`,
    type: WindowListEntryType.Window,
    title: window.title,
    pid: window.pid,
    cwd: window.cwd,
    cmdline: window.cmdline.join(' '),
  };

  return entry;
};

const processTab = (tab: KittyTab, isLast: boolean): WindowListEntry[] => {
  const entry: WindowListEntry = {
    id: tab.id,
    text: `${isLast ? LAST_TAB_INDENT : TAB_INDENT}${tab.title} (tab:${tab.id}) ${tab.is_focused ? '*' : ''}`,
    type: WindowListEntryType.Tab,
    title: tab.title,
  };

  const windows = tab.windows.map((window, index, array) => processWindow(window, index === array.length - 1, isLast));
  return [entry, ...windows];
};

const processOsWindow = (window: KittyOsWindow): WindowListEntry[] => {
  const entry: WindowListEntry = {
    id: window.id,
    text: `kitty:${window.id}`,
    type: WindowListEntryType.OsWindow,
  };

  const tabs = window.tabs.flatMap((tab, index, array) => processTab(tab, index === array.length - 1));
  return [entry, ...tabs];
};

export const processWindowList = (windowList: KittyOsWindow[]): WindowListEntry[] => {
  return windowList.flatMap((osWindow) => processOsWindow(osWindow));
};
