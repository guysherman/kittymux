package kitty

import "fmt"

type EntryCloser interface {
	CloseEntry(commandExecutor *CommandExecutor, entry WindowListEntry)
}

type EntryCloserBase struct{}

func (ec *EntryCloserBase) CloseEntry(commandExecutor CommandExecutor, entry WindowListEntry) {
	switch entry.EntryType {
	case OsWindow:
		break
	case Tab:
		closeTab(commandExecutor, entry)
		break
	case Window:
		closeWindow(commandExecutor, entry)
		break
	}
}

func closeTab(commandExecutor CommandExecutor, entry WindowListEntry) {
	args := make([]string, 0)
	args = append(args, "close-tab", "-m", fmt.Sprintf("id:%d", entry.Id))

	commandExecutor.ExecuteCommand(args)
}

func closeWindow(commandExecutor CommandExecutor, entry WindowListEntry) {
	args := make([]string, 0)
	args = append(args, "close-window", "-m", fmt.Sprintf("id:%d", entry.Id))

	commandExecutor.ExecuteCommand(args)
}
