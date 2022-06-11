package main

import (
	tea "github.com/charmbracelet/bubbletea"
)

func closeEntry(m UiModel, i ListItemModel) tea.Cmd {
	return func() tea.Msg {
		m.kc.CloseEntry(i.listEntry)

		msg := ListUpdatedMsg{}
		return msg
	}
}
