package kitty

import "strconv"
import "strings"
import "fmt"
import "encoding/json"

type IKittyConnector interface {
	SendCommand(commandExecutor CommandExecutor, argsToSend []string, windowId int)
	CloseEntry(commandExecutor CommandExecutor, entry WindowListEntry)
	FocusEntry(commandExecutor CommandExecutor, entry WindowListEntry)
	CreateWindow(commandExecutor CommandExecutor, title string, tabId int, tabTitle string, newTab bool, cwd string)
	WindowList(commandExecutor CommandExecutor) []KittyOsWindow
	RenameEntry(commandExecutor CommandExecutor, entry WindowListEntry, newName string)
}

type KittyConnector struct{}

func (cs *KittyConnector) SendCommand(commandExecutor CommandExecutor, argsToSend []string, windowId int) {
	for i := 0; i < len(argsToSend); i++ {
		if strings.Contains(argsToSend[i], " ") {
			argsToSend[i] = fmt.Sprintf("\"%s\"", argsToSend[i])
		}
	}

	commandText := fmt.Sprintf("'%s\\n'", strings.Join(argsToSend, " "))
	args := []string{"send-text", "-m", fmt.Sprintf("id:%d", windowId), commandText}
	commandExecutor.ExecuteCommand(args)
}

func (ec *KittyConnector) CloseEntry(commandExecutor CommandExecutor, entry WindowListEntry) {
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

func (ef *KittyConnector) FocusEntry(commandExecutor CommandExecutor, entry WindowListEntry) {
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

func (c *KittyConnector) CreateWindow(commandExecutor CommandExecutor, title string, tabId int, tabTitle string, newTab bool, cwd string) int {
	createArgs := make([]string, 0)
	createArgs = append(createArgs, "new-window")
	if title != "" {
		createArgs = append(createArgs, "--title")
		createArgs = append(createArgs, title)
	}

	if tabId != 0 {
		createArgs = append(createArgs, "-m")
		createArgs = append(createArgs, fmt.Sprintf("id:%d", tabId))

	} else if newTab && tabTitle != "" {
		createArgs = append(createArgs, "--new-tab")
		createArgs = append(createArgs, "--tab-title")
		createArgs = append(createArgs, fmt.Sprintf("\"%s\"", tabTitle))
	} else if tabTitle != "" {
		createArgs = append(createArgs, "-m")
		createArgs = append(createArgs, fmt.Sprintf("title:%s", tabTitle))
	}

	if cwd != "" {
		createArgs = append(createArgs, "--cwd")
		createArgs = append(createArgs, fmt.Sprintf("%s", cwd))
	}

	result := commandExecutor.ExecuteCommand(createArgs)
	windowId, err := strconv.Atoi(result)
	if err != nil {
		return -1
	}
	return windowId
}

func (wl *KittyConnector) WindowList(commandExecutor CommandExecutor) []KittyOsWindow {
	var r []KittyOsWindow

	commandOutput := commandExecutor.ExecuteCommand([]string{"ls"})
	err := json.Unmarshal([]byte(commandOutput), &r)
	if err != nil {
		fmt.Println("JSON Error:", err.Error())
	}

	return r
}

func (er *KittyConnector) RenameEntry(commandExecutor CommandExecutor, entry WindowListEntry, newName string) {
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
