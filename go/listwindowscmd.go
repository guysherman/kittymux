package main

import (
	tea "github.com/charmbracelet/bubbletea"
	"github.com/guysherman/kittymux/kitty"
)

type ListWindowsMsg struct {
	ListItems []item
}

func listWindows(m model) tea.Cmd {
	return func() tea.Msg {
		wl := kitty.EntryListerBase{}

		entries := wl.EntryList(m.kc)
		items := []item{}
		for _, entry := range entries {
			items = append(items, item{listEntry: entry})
		}

		return ListWindowsMsg{ListItems: items}
	}
}
