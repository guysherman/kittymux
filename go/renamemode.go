package main

import (
	tea "github.com/charmbracelet/bubbletea"
)

func RenameModeUpdate(m model, msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch keypress := msg.String(); keypress {
		case "ctrl+c":
			m.quitting = true
			return m, tea.Quit
		case "enter":
			return renameModeEnterPressed(m)
		default:
			im, cmd := m.input.Update(msg)
			m.input = im
			return m, cmd
		}
	}

	return m, nil
}

func renameModeEnterPressed(m model) (tea.Model, tea.Cmd) {
	m.mode = Navigate
	m.inputText = m.input.Value()
	m.input.SetValue("")
	m.input.Blur()
	i, _ := m.list.SelectedItem().(item)

	return m, renameEntry(i, m.inputText)
}
