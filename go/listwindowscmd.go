package main

import (
	tea "github.com/charmbracelet/bubbletea"
	"github.com/guysherman/kittymux/kitty"
)

func listWindows(m UiModel) tea.Cmd {
	return func() tea.Msg {
		wl := kitty.EntryListerBase{}

		entries := wl.EntryList(m.kc)
		items := []ListItemModel{}
		for _, entry := range entries {
			items = append(items, ListItemModel{listEntry: entry, listMode: m.mode})
		}

		return ListWindowsMsg{ListItems: items}
	}
}
