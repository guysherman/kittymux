import kittyCommand from './kittyCommand';

export interface CreateWindowOpts {
  tabId?: number;
  newTab?: boolean;
  tabTitle?: string;
  cwd?: string;
}

const createWindow = (title: string, opts: CreateWindowOpts | undefined = undefined): Promise<number> => {
  console.log('createWindow', { title, opts });
  const args = ['new-window', '--title', `"${title}"`];
  if (opts?.tabId) {
    args.push('-m', `id:${opts.tabId}`);
  } else if (opts?.newTab) {
    args.push('--new-tab', '--tab-title', `"${opts.tabTitle}"`);
  } else if (opts?.tabTitle) {
    args.push('-m', `title:'${opts.tabTitle}'`);
  }

  if (opts?.cwd) {
    args.push('--cwd', opts.cwd);
  }

  return kittyCommand(args).then((stdout) => parseInt(stdout as string));
};

export default createWindow;
