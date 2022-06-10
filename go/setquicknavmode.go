package main

import (
	"github.com/charmbracelet/bubbles/key"
	tea "github.com/charmbracelet/bubbletea"
)

func SetQuickNavModeUpdate(m model, msg tea.Msg) (tea.Model, tea.Cmd) {
	shortcut := false
	var keypress string
	switch msg := msg.(type) {
	case tea.KeyMsg:
		keypress = msg.String()
		switch {
		case key.Matches(msg, lowercaseLetters):
			shortcut = true
			break
		case key.Matches(msg, numbers):
			shortcut = true
			break
		case key.Matches(msg, cancel):
			return setNavigateMode(m)
		}
	}

	if shortcut {
		index := m.list.Index()
		listEntry := m.list.SelectedItem().(item)
		listEntry.shortcutKey = keypress
		m.list.SetItem(index, listEntry)
		return setNavigateMode(m)
	}
	return m, nil
}
