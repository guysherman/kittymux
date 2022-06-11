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

func getTabIdForListItem(i ListItemModel) int {
	tabId := 0
	switch i.listEntry.EntryType {
	case kitty.OsWindow:
		break
	case kitty.Tab:
		tabId = i.listEntry.Id
	case kitty.Window:
		tabId = i.listEntry.Tab.Id
	}

	return tabId
}

func collectItemsForTabId(l list.Model, tabId int) []ListItemModel {
	items := []ListItemModel{}
	listItems := l.Items()
	for _, i := range listItems {
		listItem := i.(ListItemModel)
		switch listItem.listEntry.EntryType {
		case kitty.OsWindow:
			break
		case kitty.Tab:
			if listItem.listEntry.Id == tabId {
				items = append(items, listItem)
			}
			break
		case kitty.Window:
			if listItem.listEntry.Tab.Id == tabId {
				items = append(items, listItem)
			}
			break
		}
	}

	return items
}
