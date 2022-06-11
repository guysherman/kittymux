package main

import (
	"github.com/charmbracelet/bubbles/key"
	tea "github.com/charmbracelet/bubbletea"
)

func QuickNavModeUpdate(m UiModel, msg tea.Msg) (tea.Model, tea.Cmd) {
	var listEntry ListItemModel
	var found bool
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch {
		case key.Matches(msg, lowercaseLetters):
			keypress := msg.String()
			listEntry, found = findEntry(m.list, keypress)
			break
		case key.Matches(msg, numbers):
			keypress := msg.String()
			listEntry, found = findEntry(m.list, keypress)
			break
		case key.Matches(msg, cancel):
			return setNavigateMode(m)
		}
	}

	if found {
		return m, focusEntry(m, listEntry)
	}
	return m, nil
}
