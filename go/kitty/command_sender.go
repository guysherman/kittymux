package kitty

import "strings"
import "fmt"

type CommandSender interface {
	SendCommand(commandExecutor *CommandExecutor, argsToSend []string, windowId int)
}

type CommandSenderBase struct{}

func (cs *CommandSenderBase) SendCommand(commandExecutor CommandExecutor, argsToSend []string, windowId int) {
	for i := 0; i < len(argsToSend); i++ {
		if strings.Contains(argsToSend[i], " ") {
			argsToSend[i] = fmt.Sprintf("\"%s\"", argsToSend[i])
		}
	}

	commandText := fmt.Sprintf("'%s\\n'", strings.Join(argsToSend, " "))
	args := []string{"send-text", "-m", fmt.Sprintf("id:%d", windowId), commandText}
	commandExecutor.ExecuteCommand(args)
}
