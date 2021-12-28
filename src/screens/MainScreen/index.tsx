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
  renameEntry,
} from '../../connectors/kitty';
import { getInstructions } from './getInstructions';
//└─
//
// eslint-disable-next-line @typescript-eslint/no-explicit-any
const createKeypress = (
  selectedIndex: number,
  setSelectedIndex: any,
  entries: WindowListEntry[],
  setEntries: any,
  setIsEditingName: any,
) => {
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
    } else if (ch === 'a') {
      setIsEditingName(true);
    }
  };
};

export const MainScreen = () => {
  const [entries, setEntries] = useState([]);
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [isEditingName, setIsEditingName] = useState(false);
  const items = entries.map((entry: WindowListEntry) => entry.text);
  const selectedEntry = entries[selectedIndex] ?? { type: WindowListEntryType.None };
  const instructions = getInstructions(selectedEntry.type);

  useEffect(() => {
    listWindows().then((windowList: KittyOsWindow[]) => {
      const entries = processWindowList(windowList);
      setEntries(entries);
    });
  }, []);

  const listKeyPress = createKeypress(selectedIndex, setSelectedIndex, entries, setEntries, setIsEditingName);

  const inputSubmitted = (value: string) => {
    console.error('nameSubmitted', { value });
    renameEntry(selectedEntry, value).then((windowList: WindowListEntry[]) => {
      console.error('renameEntry', { windowList });
      setEntries(windowList);
      setSelectedIndex(selectedIndex);
      setIsEditingName(false);
    });
  };

  const listOpts = {
    top: 0,
    left: 0,
    width: '100%',
    height: '50%',
    tags: true,
    border: 'line',
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
    focused: !isEditingName,
    onkeypress: listKeyPress,
    selected: selectedIndex,
  };

  return (
    <box>
      <list {...listOpts} />
      <box bottom={0} left={0} width={'100%'} height={3} border={'line'}>
        {isEditingName ? (
          <textbox
            left={0}
            width={'33%-1'}
            focused={isEditingName}
            inputOnFocus={true}
            onsubmit={inputSubmitted}
          ></textbox>
        ) : (
          <box left={0} width={'33%-1'}>
            {`${selectedEntry.title ?? ''}`}
          </box>
        )}
        <box left={'33%-1'} width={'34%'}>
          {`${selectedEntry.cmdline ?? ''}${selectedEntry.cmdline ? '|' : ''}${selectedEntry.cwd ?? ''}${
            selectedEntry.cwd ? '|' : ''
          }${selectedEntry.pid ?? ''}`}
        </box>
        <box left={'67%-1'} width={'33%-2'}>
          {instructions}
        </box>
      </box>
    </box>
  );
};
