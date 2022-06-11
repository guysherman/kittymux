package main

import (
	"github.com/charmbracelet/bubbles/key"
	tea "github.com/charmbracelet/bubbletea"
)

func QuickNavModeUpdate(m model, msg tea.Msg) (tea.Model, tea.Cmd) {
	var listEntry item
	var found bool
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch {
		case key.Matches(msg, lowercaseLetters):
			keypress := msg.String()
			listEntry, found = findEntry(m, keypress)
			break
		case key.Matches(msg, numbers):
			keypress := msg.String()
			listEntry, found = findEntry(m, keypress)
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

func findEntry(m model, shortcutKey string) (item, bool) {
	listItems := m.list.Items()
	for _, i := range listItems {
		listItem := i.(item)
		if listItem.shortcutKey == shortcutKey && listItem.listEntry.TabIsFocused {
			return listItem, true
		}
	}

	return item{}, false
}
