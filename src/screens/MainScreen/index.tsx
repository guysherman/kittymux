/** @jsx TreeCat.createElement **/
import * as blessed from 'blessed';
// eslint-disable-next-line no-unused-vars
import * as TreeCat from 'treecat';
import { useState, useEffect } from 'treecat';
import { listWindows, KittyOsWindow, KittyTab, KittyWindow } from '../../connectors/kitty';
//└─
//

interface WindowListEntry {
  id: number;
  text: string;
  type: string;
}

const PRE_INDENT = '    ';
const INDENT = ' └─ ';

const processWindow = (window: KittyWindow): WindowListEntry => {
  const entry: WindowListEntry = {
    id: window.id,
    text: `${PRE_INDENT}${INDENT}${window.title} (id:${window.id}; pid:${window.pid}) ${window.is_focused ? '*' : ''}`,
    type: 'Window',
  };

  return entry;
};

const processTab = (tab: KittyTab): WindowListEntry[] => {
  const entry: WindowListEntry = {
    id: tab.id,
    text: `${INDENT}${tab.title} (tab:${tab.id}) ${tab.is_focused ? '*' : ''}`,
    type: 'Tab',
  };

  const windows = tab.windows.map((window) => processWindow(window));
  return [entry, ...windows];
};

const processOsWindow = (window: KittyOsWindow): WindowListEntry[] => {
  const entry: WindowListEntry = {
    id: window.id,
    text: `kitty:${window.id}`,
    type: 'OsWindow',
  };

  const tabs = window.tabs.flatMap((tab) => processTab(tab));
  return [entry, ...tabs];
};

export function MainScreen() {
  const [entries, setEntries] = useState([]);
  const items = entries.length ? entries.map((entry: WindowListEntry) => entry.text) : ['No windows found'];

  useEffect(() => {
    listWindows().then((windowList: KittyOsWindow[]) => {
      const entries = windowList.flatMap((osWindow) => processOsWindow(osWindow));
      setEntries(entries);
    });
  }, []);

  const listOpts = {
    top: 0,
    left: 0,
    width: '100%',
    height: '50%',
    tags: true,
    border: {
      type: 'line' as const,
    },
    style: {
      border: {
        fg: '#f0f0f0',
      },
      selected: {
        fg: 'white',
        bg: 'blue',
      },
      item: {
        fg: 'white',
        bg: 'black',
      },
    },
    keys: true,
    vi: true,
    items,
    label: 'Windows',
    //'onselect item': handleSelectItem,
  };
  return (
    <box>
      <list {...listOpts} />
    </box>
  );
}
