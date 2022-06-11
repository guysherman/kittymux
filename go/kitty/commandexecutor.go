package kitty

import (
	"log"
	"os"
	"os/exec"
	"strings"
)

type CommandExecutor interface {
	ExecuteCommand(args []string) string
}

type KittyCommandExecutor struct{}

func (c *KittyCommandExecutor) ExecuteCommand(args []string) string {
	args = append([]string{"@"}, args...)
	cmd := exec.Command("kitty", args...)
	output, err := cmd.Output()
	if err != nil {
		log.Fatal(err)
		os.Exit(1)
	}

	return strings.TrimSpace(string(output))
}
