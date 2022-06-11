package kitty

import (
	"encoding/json"
	"fmt"
	"strconv"
	"strings"
)

type IKittyConnector interface {
	SendCommand(argsToSend []string, windowId int)
	CloseEntry(entry WindowListEntry)
	FocusEntry(entry WindowListEntry)
	CreateWindow(title string, tabId int, tabTitle string, newTab bool, cwd string) int
	WindowList() []KittyOsWindow
	RenameEntry(entry WindowListEntry, newName string)
}

type KittyConnector struct {
	commandExecutor CommandExecutor
}

func NewKittyConnector(commandExecutor CommandExecutor) *KittyConnector {
	k := &KittyConnector{commandExecutor: commandExecutor}

	return k
}

func (kc *KittyConnector) SendCommand(argsToSend []string, windowId int) {
	args := make([]string, 0)
	for i := 0; i < len(argsToSend); i++ {
		if strings.Contains(argsToSend[i], " ") {
			argsToSend[i] = fmt.Sprintf("\"%s\"", argsToSend[i])
		}
	}

	commandText := fmt.Sprintf("%s\\n", strings.Join(argsToSend, " "))
	args = append(args, "send-text", "-m", fmt.Sprintf("id:%d", windowId), commandText)
	kc.commandExecutor.ExecuteCommand(args)
}

func (kc *KittyConnector) CloseEntry(entry WindowListEntry) {
	switch entry.EntryType {
	case OsWindow:
		break
	case Tab:
		closeTab(kc.commandExecutor, entry)
		break
	case Window:
		closeWindow(kc.commandExecutor, entry)
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

func (kc *KittyConnector) FocusEntry(entry WindowListEntry) {
	switch entry.EntryType {
	case OsWindow:
		break
	case Tab:
		focusCommand("focus-tab", kc.commandExecutor, entry)
		break
	case Window:
		focusCommand("focus-window", kc.commandExecutor, entry)
	}
}

func focusCommand(focusType string, commandExecutor CommandExecutor, entry WindowListEntry) {
	args := make([]string, 0)
	args = append(args, focusType, "-m", fmt.Sprintf("id:%d", entry.Id))

	commandExecutor.ExecuteCommand(args)
}

func (kc *KittyConnector) CreateWindow(title string, tabId int, tabTitle string, newTab bool, cwd string) int {
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
		createArgs = append(createArgs, fmt.Sprintf("%s", tabTitle))
	} else if tabTitle != "" {
		createArgs = append(createArgs, "-m")
		createArgs = append(createArgs, fmt.Sprintf("title:%s", tabTitle))
	}

	if cwd != "" {
		createArgs = append(createArgs, "--cwd")
		createArgs = append(createArgs, fmt.Sprintf("%s", cwd))
	}

	result := kc.commandExecutor.ExecuteCommand(createArgs)
	windowId, err := strconv.Atoi(result)
	if err != nil {
		return -1
	}
	return windowId
}

func (kc *KittyConnector) WindowList() []KittyOsWindow {
	var r []KittyOsWindow

	commandOutput := kc.commandExecutor.ExecuteCommand([]string{"ls"})
	err := json.Unmarshal([]byte(commandOutput), &r)
	if err != nil {
		fmt.Println("JSON Error:", err.Error())
	}

	return r
}

func (kc *KittyConnector) RenameEntry(entry WindowListEntry, newName string) {
	switch entry.EntryType {
	case OsWindow:
		break
	case Tab:
		renameCmd("set-tab-title", kc.commandExecutor, entry, newName)
		break
	case Window:
		renameCmd("set-window-title", kc.commandExecutor, entry, newName)
	}
}

func renameCmd(renameType string, commandExecutor CommandExecutor, entry WindowListEntry, newName string) {
	args := make([]string, 0)
	args = append(args, renameType, "-m", fmt.Sprintf("id:%d", entry.Id), newName)

	commandExecutor.ExecuteCommand(args)
}
