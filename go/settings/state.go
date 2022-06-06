package settings

import (
	"fmt"
	"os"
)

func GetStateDir() string {
	if os.Getenv("KITTYMUX_STATE_DIR") != "" {
		return os.Getenv("KITTYMUX_STATE_DIR")
	} else if os.Getenv("XDG_STATE_HOME") != "" {
		return fmt.Sprintf("%s/kittymux", os.Getenv("XDG_STATE_HOME"))
	} else {
		return "~/.local/state/kittymux"
	}
}
