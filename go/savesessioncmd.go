package main

import tea "github.com/charmbracelet/bubbletea"

type NoopMessage struct{}

func saveSession(m UiModel) tea.Cmd {
	return func() tea.Msg {
		selected := m.list.SelectedItem().(ListItemModel)
		tab := selected.listEntry.Tab
		m.sc.SaveSession(*tab)
		return NoopMessage{}
	}
}
