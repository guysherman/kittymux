package main

import (
	tea "github.com/charmbracelet/bubbletea"
)

func renameEntry(m UiModel, i ListItemModel, newName string) tea.Cmd {
	return func() tea.Msg {
		m.kc.RenameEntry(i.listEntry, newName)

		msg := ListUpdatedMsg{}
		return msg
	}
}
