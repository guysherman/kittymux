package kitty

import "fmt"

type EntryRenamer interface {
	RenameEntry(commandExecutor *CommandExecutor, entry WindowListEntry, newName string)
}

type EntryRenamerBase struct{}

func (er *EntryRenamerBase) RenameEntry(commandExecutor CommandExecutor, entry WindowListEntry, newName string) {
	switch entry.EntryType {
	case OsWindow:
		break
	case Tab:
		renameCmd("set-tab-title", commandExecutor, entry, newName)
		break
	case Window:
		renameCmd("set-window-title", commandExecutor, entry, newName)
	}
}

func renameCmd(renameType string, commandExecutor CommandExecutor, entry WindowListEntry, newName string) {
	args := make([]string, 0)
	args = append(args, renameType, "-m", fmt.Sprintf("id:%d", entry.Id), newName)

	commandExecutor.ExecuteCommand(args)
}
