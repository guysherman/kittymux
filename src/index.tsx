/** @jsx TreeCat.createElement **/
import * as TreeCat from 'treecat';
import { MainScreen } from './screens/MainScreen';

const main = async () => {
  const rootScreen: TreeCat.blessed.Widgets.Screen = TreeCat.createRootScreen();
  rootScreen.program.on('keypress', (_ch: string, key: TreeCat.blessed.Widgets.Events.IKeyEventArg) => {
    if (key.full === 'C-c') {
      process.exit(0);
    }
  });

  TreeCat.render(<MainScreen />, rootScreen);
};

main().catch((e) => {
  console.error('Fatal Error', { error: e });
  process.exit(1);
});
