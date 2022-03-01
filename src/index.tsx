/** @jsx TreeCat.createElement **/
import * as TreeCat from '@guysherman/treecat';
import { MainScreen } from './screens/MainScreen';
import minimist, { Opts } from 'minimist';
import loadSession from './connectors/sessions/loadSession';

const main = async () => {
  const cliArgs: Opts = {
    string: ['scope', 'session'],
    default: {
      scope: 'all',
    },
  };
  const parsedArgs = minimist(process.argv.slice(2), cliArgs);

  if (parsedArgs.session) {
    await loadSession(parsedArgs.session);
    process.exit(0);
  }

  const rootScreen: TreeCat.blessed.Widgets.Screen = TreeCat.createRootScreen();
  rootScreen.program.on('keypress', (_ch: string, key: TreeCat.blessed.Widgets.Events.IKeyEventArg) => {
    if (key.full === 'C-c') {
      process.exit(0);
    }
  });

  TreeCat.render(<MainScreen scope={parsedArgs.scope} />, rootScreen);
};

main().catch((e) => {
  console.error('Fatal Error', { error: e });
  process.exit(1);
});
