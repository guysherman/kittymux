package main

import tea "github.com/charmbracelet/bubbletea"

func NavigateModeUpdate(m model, msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch keypress := msg.String(); keypress {
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

func navigateModeEnterPressed(m model) (tea.Model, tea.Cmd) {
	i, _ := m.list.SelectedItem().(item)
	return m, focusEntry(i)
}
