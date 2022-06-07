package main

import (
	tea "github.com/charmbracelet/bubbletea"
)

type ExitMessage struct{}

func focusEntry(m model, i item) tea.Cmd {
	return func() tea.Msg {
		m.kc.FocusEntry(i.listEntry)

		m := ExitMessage{}
		return m
	}
}
