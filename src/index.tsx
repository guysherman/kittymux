/** @jsx TreeCat.createElement **/
import runCli from './cli';
import runTextUi from './screens';

const main = async () => {
  const [shouldExit, scope] = await runCli();

  if (shouldExit) {
    process.exit(0);
  }

  runTextUi(scope);
};

main().catch((e) => {
  console.error('Fatal Error', { error: e });
  process.exit(1);
});
