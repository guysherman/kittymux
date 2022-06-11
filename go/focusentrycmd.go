package main

import (
	tea "github.com/charmbracelet/bubbletea"
)

type ExitMessage struct{}

func focusEntry(m UiModel, i ListItemModel) tea.Cmd {
	return func() tea.Msg {
		m.kc.FocusEntry(i.listEntry)

		m := ExitMessage{}
		return m
	}
}
