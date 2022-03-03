/** @jsx TreeCat.createElement **/
// eslint-disable-next-line no-unused-vars
import * as TreeCat from '@guysherman/treecat';
import * as blessed from 'blessed';
import { useEffect, useReducer } from '@guysherman/treecat';
import { WindowListEntry, WindowListEntryType, renameEntry } from '../../connectors/kitty';
import { getInstructions } from './store/getInstructions';
import { mainScreenContext, MainScreenMode, DefaultMainScreenMode, QuickNavHandle } from './store/model';
import { processCommand } from './actions/processCommand';
import { processListKeyPress } from './actions/processListKeyPress';
import { getDefaultState, MainScreenActions, mainScreenReducer } from './store/reducer';
import { refreshWindowList } from './actions/refreshWindowList';
import { processQuickNavKeypress } from './actions/processQuickNavKeypress';
import { filterEntries } from './store/scopeFilter';
import getItems from './store/selectors';
//└─
//
// eslint-disable-next-line @typescript-eslint/no-explicit-any
interface MainScreenProps {
  scope: string;
}

export const MainScreen = (props: MainScreenProps) => {
  const { scope } = props;
  const [state, dispatch] = useReducer(mainScreenReducer, getDefaultState());

  const { entries, selectedIndex, mode } = state;
  const items = getItems(state, filterEntries(scope, entries));
  const selectedEntry = entries[selectedIndex] ?? { type: WindowListEntryType.None };
  const instructions = getInstructions(state);

  useEffect(() => {
    refreshWindowList(dispatch);
  }, []);

  const listKeyPress = (ch: string, key: blessed.Widgets.Events.IKeyEventArg): void => {
    if (entries.length) {
      processListKeyPress(state, dispatch, ch, key);
    }
  };

  const onRename = (value: string) => {
    renameEntry(selectedEntry, value)
      .then(() => refreshWindowList(dispatch))
      .then(() => {
        dispatch({ type: MainScreenActions.SetSelectedIndex, payload: selectedIndex });
        dispatch({ type: MainScreenActions.SetMode, payload: DefaultMainScreenMode });
      });
  };

  const onCommand = (value: string) => {
    processCommand(value, dispatch);
  };

  const quickNavKeyPress = (_ch: string, key: blessed.Widgets.Events.IKeyEventArg) => {
    if (entries.length) {
      processQuickNavKeypress(state, dispatch, key);
    }
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

  const getModeString = () => {
    switch (mode) {
      case MainScreenMode.Navigate:
        return 'NAV';
      case MainScreenMode.QuickNav:
        return 'QNV';
      case MainScreenMode.SetQuickNav:
        return 'SQN';
      case MainScreenMode.Command:
        return 'CMD';
      case MainScreenMode.Rename:
        return 'RNE';
      default:
        return '???';
    }
  };

  const showInputBox = mode === MainScreenMode.Command || mode === MainScreenMode.Rename;
  const quickNavMode = mode === MainScreenMode.QuickNav || mode === MainScreenMode.SetQuickNav;

  return (
    <box onkeypress={quickNavKeyPress} focused={quickNavMode}>
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
          <box left={'67%-1'} width={'33%-7'}>
            {instructions}
          </box>
          <box left={'100%-7'} width={5} tags={true}>
            {`{inverse} ${getModeString()} {/inverse}`}
          </box>
        </box>
      </mainScreenContext.Provider>
    </box>
  );
};
