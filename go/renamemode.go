package main

import (
	tea "github.com/charmbracelet/bubbletea"
)

func RenameModeUpdate(m UiModel, msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch keypress := msg.String(); keypress {
		case "enter":
			return renameModeEnterPressed(m)
		case "esc":
			m.mode = Navigate
			return m, nil
		default:
			im, cmd := m.input.Update(msg)
			m.input = im
			return m, cmd
		}
	}

	return m, nil
}

func renameModeEnterPressed(m UiModel) (tea.Model, tea.Cmd) {
	m.mode = Navigate
	m.inputText = m.input.Value()
	m.input.SetValue("")
	m.input.Blur()
	i, _ := m.list.SelectedItem().(ListItemModel)

	return m, renameEntry(m, i, m.inputText)
}
