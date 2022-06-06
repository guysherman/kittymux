package main

import (
	tea "github.com/charmbracelet/bubbletea"
	"github.com/guysherman/kittymux/kitty"
)

func NavigateModeUpdate(m model, msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch keypress := msg.String(); keypress {
		case "J":
			return nextTabPressed(m)
		case "K":
			return prevTabPressed(m)
		case "a":
			m.mode = Rename
			m.input.Focus()
			return m, nil
		case "ctrl+c":
			m.quitting = true
			return m, tea.Quit
		case "enter":
			return navigateModeEnterPressed(m)
		default:
			lm, cmd := m.list.Update(msg)
			m.list = lm
			return m, cmd
		}
	}
	return m, nil
}

func nextTabPressed(m model) (tea.Model, tea.Cmd) {
	items := m.list.Items()
	for i := m.list.Index() + 1; i < len(items); i++ {
		item := items[i].(item)
		if item.listEntry.EntryType == kitty.Tab {
			m.list.Select(i)
			break
		}
	}

	return m, nil
}

func prevTabPressed(m model) (tea.Model, tea.Cmd) {
	items := m.list.Items()
	for i := m.list.Index() - 1; i >= 0; i-- {
		item := items[i].(item)
		if item.listEntry.EntryType == kitty.Tab {
			m.list.Select(i)
			break
		}
	}

	return m, nil
}

func navigateModeEnterPressed(m model) (tea.Model, tea.Cmd) {
	i, _ := m.list.SelectedItem().(item)
	return m, focusEntry(i)
}
