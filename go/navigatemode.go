package main

import (
	"github.com/charmbracelet/bubbles/list"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/guysherman/kittymux/kitty"
)

func NavigateModeUpdate(m model, msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch keypress := msg.String(); keypress {
		case "J":
			return nextTabPressed(m)
		case "K":
			return prevTabPressed(m)
		case "a":
			m.mode = Rename
			m.input.Focus()
			return m, nil
		case "delete":
			return closeEntryPressed(m)
		case "x":
			return closeEntryPressed(m)
		case "'":
			return quickNavModePressed(m)
		case "m":
			return setQuickNavModePressed(m)
		case "s":
			return saveSessionPressed(m)
		case "ctrl+c":
			m.quitting = true
			return m, tea.Quit
		case "enter":
			return navigateModeEnterPressed(m)
		default:
			lm, cmd := m.list.Update(msg)
			m.list = lm
			return m, cmd
		}
	}
	return m, nil
}

func nextTabPressed(m model) (tea.Model, tea.Cmd) {
	items := m.list.Items()
	for i := m.list.Index() + 1; i < len(items); i++ {
		item := items[i].(item)
		if item.listEntry.EntryType == kitty.Tab {
			m.list.Select(i)
			break
		}
	}

	return m, nil
}

func prevTabPressed(m model) (tea.Model, tea.Cmd) {
	items := m.list.Items()
	for i := m.list.Index() - 1; i >= 0; i-- {
		item := items[i].(item)
		if item.listEntry.EntryType == kitty.Tab {
			m.list.Select(i)
			break
		}
	}

	return m, nil
}

func closeEntryPressed(m model) (tea.Model, tea.Cmd) {
	i, _ := m.list.SelectedItem().(item)
	return m, closeEntry(m, i)
}

func quickNavModePressed(m model) (tea.Model, tea.Cmd) {
	m.mode = QuickNav
	listItems := []list.Item{}
	for _, listItem := range m.list.Items() {
		i := listItem.(item)
		i.listMode = QuickNav
		listItems = append(listItems, i)
	}

	m.list.SetItems(listItems)
	return m, nil
}

func setQuickNavModePressed(m model) (tea.Model, tea.Cmd) {
	m.mode = SetQuickNav
	listItems := []list.Item{}
	for _, listItem := range m.list.Items() {
		i := listItem.(item)
		i.listMode = SetQuickNav
		listItems = append(listItems, i)
	}

	m.list.SetItems(listItems)
	return m, nil
}

func saveSessionPressed(m model) (tea.Model, tea.Cmd) {
	selected := m.list.SelectedItem().(item)
	if selected.listEntry.EntryType != kitty.Tab {
		return m, nil
	}

	return m, saveSession(m)
}

func navigateModeEnterPressed(m model) (tea.Model, tea.Cmd) {
	i, _ := m.list.SelectedItem().(item)
	return m, focusEntry(m, i)
}

func setNavigateMode(m model) (tea.Model, tea.Cmd) {
	m.mode = Navigate
	listItems := []list.Item{}
	for _, listItem := range m.list.Items() {
		i := listItem.(item)
		i.listMode = Navigate
		listItems = append(listItems, i)
	}

	m.list.SetItems(listItems)
	return m, nil
}
