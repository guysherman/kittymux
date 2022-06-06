package main

import (
	tea "github.com/charmbracelet/bubbletea"
	"github.com/guysherman/kittymux/kitty"
)

type ListUpdatedMsg struct{}

func renameEntry(i item, newName string) tea.Cmd {
	return func() tea.Msg {
		ce := kitty.KittyCommandExecutor{}
		kc := kitty.NewKittyConnector(&ce)
		kc.RenameEntry(i.listEntry, newName)

		msg := ListUpdatedMsg{}
		return msg
	}
}
