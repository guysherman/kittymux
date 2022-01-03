/** @jsx TreeCat.createElement **/
// eslint-disable-next-line no-unused-vars
import * as TreeCat from '@guysherman/treecat';
import * as blessed from 'blessed';
import { useEffect, useReducer } from '@guysherman/treecat';
import { WindowListEntry, WindowListEntryType, renameEntry } from '../../connectors/kitty';
import { getInstructions } from './getInstructions';
import { mainScreenContext, MainScreenMode, DefaultMainScreenMode } from './model';
import { processCommand } from './processCommand';
import { processListKeyPress } from './processListKeyPress';
import { MainScreenActions, mainScreenReducer } from './reducer';
import { refreshWindowList } from './refreshWindowList';
//└─
//
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const MainScreen = () => {
  const [state, dispatch] = useReducer(mainScreenReducer, {
    entries: [] as WindowListEntry[],
    selectedIndex: 0,
    mode: DefaultMainScreenMode,
  });

  const { entries, selectedIndex, mode } = state;
  const items = entries.map((entry: WindowListEntry) => entry.text);
  const selectedEntry = entries[selectedIndex] ?? { type: WindowListEntryType.None };
  const instructions = getInstructions(selectedEntry.type);

  useEffect(() => {
    refreshWindowList(dispatch);
  }, []);

  const listKeyPress = (ch: string, key: blessed.Widgets.Events.IKeyEventArg): void => {
    processListKeyPress(state, dispatch, ch, key);
  };

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
