package main

import (
	tea "github.com/charmbracelet/bubbletea"
	"github.com/guysherman/kittymux/kitty"
)

type ListWindowsMsg struct {
	ListItems []item
}

func listWindows() tea.Msg {
	ce := kitty.KittyCommandExecutor{}
	kc := kitty.NewKittyConnector(&ce)
	wl := kitty.EntryListerBase{}

	entries := wl.EntryList(kc)
	items := []item{}
	for _, entry := range entries {
		items = append(items, item{listEntry: entry})
	}

	return ListWindowsMsg{ListItems: items}
}
