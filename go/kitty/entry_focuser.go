package kitty

import "fmt"

type EntryFocuser interface {
	FocusEntry(commandExecutor *CommandExecutor, entry WindowListEntry)
}

type EntryFocuserBase struct{}

func (ef *EntryFocuserBase) FocusEntry(commandExecutor CommandExecutor, entry WindowListEntry) {
	switch entry.EntryType {
	case OsWindow:
		break
	case Tab:
		focusCommand("focus-tab", commandExecutor, entry)
		break
	case Window:
		focusCommand("focus-window", commandExecutor, entry)
	}
}

func focusCommand(focusType string, commandExecutor CommandExecutor, entry WindowListEntry) {
	args := make([]string, 0)
	args = append(args, focusType, "-m", fmt.Sprintf("id:%d", entry.Id))

	commandExecutor.ExecuteCommand(args)
}
