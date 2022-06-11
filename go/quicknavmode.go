package main

import (
	"github.com/charmbracelet/bubbles/key"
	tea "github.com/charmbracelet/bubbletea"
)

var lowercaseLetters = key.NewBinding(
	key.WithKeys("a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"),
)

var numbers = key.NewBinding(
	key.WithKeys("1", "2", "3", "4", "5", "6", "7", "8", "9", "0"),
)

var cancel = key.NewBinding(
	key.WithKeys("escape", "esc", "'"),
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
