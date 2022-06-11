package main

import (
	tea "github.com/charmbracelet/bubbletea"
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
		sameTabItems := collectItemsForTabId(m.list, tabId)

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
