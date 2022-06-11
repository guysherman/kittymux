package main

import (
	tea "github.com/charmbracelet/bubbletea"
	"github.com/guysherman/kittymux/kitty"
	"github.com/guysherman/kittymux/settings"
)

func updateQuickNav(m UiModel, i ListItemModel, keypress string) tea.Cmd {
	return func() tea.Msg {
		handle := settings.QuickNavHandle{
			EntryId:   i.listEntry.Id,
			EntryType: i.listEntry.EntryType,
		}

		qndb := m.qndb
		tabId := getTabIdForListItem(i)
		sameTabItems := collectItemsForTabId(m, tabId)

		for _, i := range sameTabItems {
			if i.shortcutKey == keypress {
				qndb = qndb.RemoveHandle(settings.QuickNavHandle{
					EntryId:   i.listEntry.Id,
					EntryType: i.listEntry.EntryType,
				})
			}
		}

		qndb = qndb.SetShortcut(keypress, handle)

		return QuickNavsUpdatedMsg{
			qndb: qndb,
		}
	}
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

func collectItemsForTabId(m UiModel, tabId int) []ListItemModel {
	items := []ListItemModel{}
	listItems := m.list.Items()
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
