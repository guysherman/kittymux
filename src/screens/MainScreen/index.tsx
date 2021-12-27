/** @jsx TreeCat.createElement **/
// eslint-disable-next-line no-unused-vars
import * as TreeCat from '@guysherman/treecat';
import { useState, useEffect } from '@guysherman/treecat';
import {
  listWindows,
  KittyOsWindow,
  WindowListEntry,
  WindowListEntryType,
  processWindowList,
  focusEntry,
  closeEntry,
} from '../../connectors/kitty';
//└─
//
// eslint-disable-next-line @typescript-eslint/no-explicit-any
const createKeypress = (selectedIndex: number, setSelectedIndex: any, entries: WindowListEntry[], setEntries: any) => {
  return (ch: string) => {
    if (ch === 'j') {
      const newIndex = Math.min(entries.length, selectedIndex + 1);
      setSelectedIndex(newIndex);
    } else if (ch === 'k') {
      const newIndex = Math.max(0, selectedIndex - 1);
      setSelectedIndex(newIndex);
    } else if (ch === 'J') {
      const followingEntries = entries.slice(selectedIndex + 1);
      const nextTab: number = followingEntries.findIndex(
        (entry: WindowListEntry) => entry.type === WindowListEntryType.Tab,
      );
      const nextIndex = selectedIndex + 1 + nextTab;
      setSelectedIndex(nextIndex >= entries.length ? selectedIndex : nextIndex);
    } else if (ch === 'K') {
      const precedingEntries = entries.slice(0, selectedIndex);
      const nextTab: number = precedingEntries
        .reverse()
        .findIndex((entry: WindowListEntry) => entry.type === WindowListEntryType.Tab);
      const nextIndex = selectedIndex - 1 - nextTab;
      setSelectedIndex(nextIndex <= 0 ? selectedIndex : nextIndex);
    } else if (ch === '\r') {
      const entry = entries[selectedIndex];
      focusEntry(entry);
    } else if (ch === 'x') {
      const entry = entries[selectedIndex];
      closeEntry(entry).then((windowList: WindowListEntry[]) => {
        setEntries(windowList);
        setSelectedIndex(Math.max(0, Math.min(selectedIndex - 1, windowList.length - 1)));
      });
    }
  };
};

export const MainScreen = () => {
  const [entries, setEntries] = useState([]);
  const [selectedIndex, setSelectedIndex] = useState(0);
  const items = entries.map((entry: WindowListEntry) => entry.text);

  useEffect(() => {
    listWindows().then((windowList: KittyOsWindow[]) => {
      const entries = processWindowList(windowList);
      setEntries(entries);
    });
  }, []);

  const listKeyPress = createKeypress(selectedIndex, setSelectedIndex, entries, setEntries);

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
    items,
    label: 'Windows',
    focused: true,
    onkeypress: listKeyPress,
    selected: selectedIndex,
  };

  return (
    <box>
      <list {...listOpts} />
    </box>
  );
};
