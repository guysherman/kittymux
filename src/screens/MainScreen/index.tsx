/** @jsx TreeCat.createElement **/
// eslint-disable-next-line no-unused-vars
import * as TreeCat from '@guysherman/treecat';
import { useEffect, useReducer } from '@guysherman/treecat';
import { WindowListEntry, WindowListEntryType, focusEntry, closeEntry, renameEntry } from '../../connectors/kitty';
import { getInstructions } from './getInstructions';
import { MainScreenState, mainScreenContext, MainScreenMode, DefaultMainScreenMode } from './model';
import { processCommand } from './processCommand';
import { MainScreenActions, mainScreenReducer } from './reducer';
import { refreshWindowList } from './refreshWindowList';
//└─
//
// eslint-disable-next-line @typescript-eslint/no-explicit-any
const createKeypress = (state: MainScreenState, dispatch: (action: any) => void) => {
  const { entries, selectedIndex } = state;
  return (ch: string /*, key: blessed.Widgets.Events.IKeyEventArg*/) => {
    if (ch === ':') {
      dispatch({ type: MainScreenActions.SetMode, payload: MainScreenMode.Command });
    } else if (ch === 'j') {
      const newIndex = Math.min(entries.length - 1, selectedIndex + 1);
      dispatch({ type: MainScreenActions.SetSelectedIndex, payload: newIndex });
    } else if (ch === 'k') {
      const newIndex = Math.max(0, selectedIndex - 1);
      dispatch({ type: MainScreenActions.SetSelectedIndex, payload: newIndex });
    } else if (ch === 'J') {
      const followingEntries = entries.slice(selectedIndex + 1);
      const nextTab: number = followingEntries.findIndex(
        (entry: WindowListEntry) => entry.type === WindowListEntryType.Tab,
      );
      const nextIndex = selectedIndex + 1 + nextTab;
      dispatch({
        type: MainScreenActions.SetSelectedIndex,
        payload: nextIndex > entries.length - 1 ? selectedIndex : nextIndex,
      });
    } else if (ch === 'K') {
      const precedingEntries = entries.slice(0, selectedIndex);
      const nextTab: number = precedingEntries
        .reverse()
        .findIndex((entry: WindowListEntry) => entry.type === WindowListEntryType.Tab);
      const nextIndex = selectedIndex - 1 - nextTab;
      dispatch({ type: MainScreenActions.SetSelectedIndex, payload: nextIndex <= 0 ? selectedIndex : nextIndex });
    } else if (ch === '\r') {
      const entry = entries[selectedIndex];
      focusEntry(entry);
    } else if (ch === 'x') {
      const entry = entries[selectedIndex];
      closeEntry(entry).then((windowList: WindowListEntry[]) => {
        dispatch({ type: MainScreenActions.SetEntries, payload: windowList });
        dispatch({
          type: MainScreenActions.SetSelectedIndex,
          payload: Math.max(0, Math.min(selectedIndex, windowList.length - 1)),
        });
      });
    } else if (ch === 'a') {
      dispatch({ type: MainScreenActions.SetMode, payload: MainScreenMode.Rename });
    }
  };
};

export const MainScreen = () => {
  const [state, dispatch] = useReducer(mainScreenReducer, {
    entries: [] as WindowListEntry[],
    selectedIndex: 0,
    mode: MainScreenMode.Navigate,
  });

  const { entries, selectedIndex, mode } = state;
  const items = entries.map((entry: WindowListEntry) => entry.text);
  const selectedEntry = entries[selectedIndex] ?? { type: WindowListEntryType.None };
  const instructions = getInstructions(selectedEntry.type);

  useEffect(() => {
    refreshWindowList(dispatch);
  }, []);

  const listKeyPress = createKeypress(state, dispatch);

  const onRename = (value: string) => {
    renameEntry(selectedEntry, value).then((windowList: WindowListEntry[]) => {
      dispatch({ type: MainScreenActions.SetEntries, payload: windowList });
      dispatch({ type: MainScreenActions.SetSelectedIndex, payload: selectedIndex });
      dispatch({ type: MainScreenActions.SetMode, payload: DefaultMainScreenMode });
    });
  };

  const onCommand = (value: string) => {
    processCommand(value, dispatch);
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
    focused: mode === MainScreenMode.Navigate,
    onkeypress: listKeyPress,
    selected: selectedIndex,
  };

  const getInputHandler = (): ((value: string) => void) => {
    switch (mode) {
      case MainScreenMode.Rename:
        return onRename;
      case MainScreenMode.Command:
        return onCommand;
      default:
        // eslint-disable-next-line @typescript-eslint/no-unused-vars
        return (_value: string) => {
          return;
        };
    }
  };

  const showInputBox = mode === MainScreenMode.Command || mode === MainScreenMode.Rename;

  return (
    <box>
      <mainScreenContext.Provider value={{ state, dispatch }}>
        <list {...listOpts} />
        <box bottom={0} left={0} width={'100%'} height={3} border={'line'}>
          {showInputBox ? (
            <textbox
              left={0}
              width={'33%-1'}
              focused={showInputBox}
              inputOnFocus={true}
              onsubmit={getInputHandler()}
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
      </mainScreenContext.Provider>
    </box>
  );
};
