package main

import tea "github.com/charmbracelet/bubbletea"

type NoopMessage struct{}

func saveSession(m model) tea.Cmd {
	return func() tea.Msg {
		selected := m.list.SelectedItem().(item)
		tab := selected.listEntry.Tab
		m.sc.SaveSession(*tab)
		return NoopMessage{}
	}
}
