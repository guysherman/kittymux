/** @jsx TreeCat.createElement **/
import * as TreeCat from '@guysherman/treecat';
import { MainScreen } from './MainScreen';

const runTextUi = async (scope: string) => {
  const rootScreen: TreeCat.blessed.Widgets.Screen = TreeCat.createRootScreen();
  rootScreen.program.on('keypress', (_ch: string, key: TreeCat.blessed.Widgets.Events.IKeyEventArg) => {
    if (key.full === 'C-c') {
      process.exit(0);
    }
  });

  TreeCat.render(<MainScreen scope={scope} />, rootScreen);
};

export default runTextUi;
