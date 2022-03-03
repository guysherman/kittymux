import minimist, { Opts } from 'minimist';
import { loadOrFindSession } from '../connectors/sessions';

const minimistOpts: Opts = {
  string: ['scope', 'session'],
  default: {
    scope: 'all',
  },
};

const runCli = async (): Promise<[shouldContinue: boolean, scope: string]> => {
  const parsedArgs = minimist(process.argv.slice(2), minimistOpts);

  let shouldExit = false;
  if (parsedArgs.session) {
    await loadOrFindSession(parsedArgs.session);
    shouldExit = true;
  }
  return [shouldExit, parsedArgs.scope];
};

export default runCli;
