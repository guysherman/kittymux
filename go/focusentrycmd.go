package main

import (
	tea "github.com/charmbracelet/bubbletea"
	"github.com/guysherman/kittymux/kitty"
)

type ExitMessage struct{}

func focusEntry(i item) tea.Cmd {
	return func() tea.Msg {
		ce := kitty.KittyCommandExecutor{}
		kc := kitty.NewKittyConnector(&ce)
		kc.FocusEntry(i.listEntry)

		m := ExitMessage{}
		return m
	}
}
