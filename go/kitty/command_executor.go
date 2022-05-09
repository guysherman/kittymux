package kitty

type CommandExecutor interface {
	ExecuteCommand(args []string) string
}

type KittyCommandExecutor struct{}

func (c *KittyCommandExecutor) ExecuteCommand(args []string) string {
	return ""
}
