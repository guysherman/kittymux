package sessions

import "github.com/guysherman/kittymux/kitty"

type Session struct {
	Title       string
	ShortcutKey string
	Windows     []Window
	Layout      string
}

type Window struct {
	Title             string
	ShortcutKey       string
	ForegroundProcess ProcessHandle
	Cwd               string
}

type ProcessHandle struct {
	Args []string
	Cwd  string
}

func (s *Session) Load(kittyConnector kitty.IKittyConnector, sessionDao ISessionDao, sessionName string) {

}
