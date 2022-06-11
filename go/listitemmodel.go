package main

import (
	"github.com/charmbracelet/bubbles/list"
	"github.com/guysherman/kittymux/kitty"
)

type ListItemModel struct {
	listEntry   kitty.WindowListEntry
	listMode    uiMode
	shortcutKey string
}

func (i ListItemModel) FilterValue() string { return i.listEntry.Text }

func findEntry(l list.Model, shortcutKey string) (ListItemModel, bool) {
	listItems := l.Items()
	for _, i := range listItems {
		listItem := i.(ListItemModel)
		if listItem.shortcutKey == shortcutKey && listItem.listEntry.TabIsFocused {
			return listItem, true
		}
	}

	return ListItemModel{}, false
}
