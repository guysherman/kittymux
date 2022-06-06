package main

import (
	tea "github.com/charmbracelet/bubbletea"
	"github.com/guysherman/kittymux/kitty"
)

func closeEntry(i item) tea.Cmd {
	return func() tea.Msg {
		ce := kitty.KittyCommandExecutor{}
		kc := kitty.NewKittyConnector(&ce)
		kc.CloseEntry(i.listEntry)

		msg := ListUpdatedMsg{}
		return msg
	}
}
