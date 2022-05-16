package kitty

import "strconv"
import "fmt"

type WindowCreator interface {
	CreateWindow(commandExecutor CommandExecutor) int
}

type WindowCreatorBase struct{}

func (c *WindowCreatorBase) CreateWindow(commandExecutor CommandExecutor, title string, tabId int, tabTitle string, newTab bool, cwd string) int {
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
