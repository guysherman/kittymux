package kitty

import "encoding/json"
import "fmt"

type WindowLister interface {
	WindowList(commandExecutor CommandExecutor) []KittyOsWindow
}

type WindowListerBase struct{}

func (wl *WindowListerBase) WindowList(commandExecutor CommandExecutor) []KittyOsWindow {
	var r []KittyOsWindow

	commandOutput := commandExecutor.ExecuteCommand([]string{"ls"})
	err := json.Unmarshal([]byte(commandOutput), &r)
	if err != nil {
		fmt.Println("JSON Error:", err.Error())
	}

	return r
}
