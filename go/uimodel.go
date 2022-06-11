package main

import (
	"fmt"

	"github.com/charmbracelet/bubbles/list"
	"github.com/charmbracelet/bubbles/textinput"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/guysherman/kittymux/kitty"
	"github.com/guysherman/kittymux/sessions"
	"github.com/guysherman/kittymux/settings"
)

type uiMode int64

const (
	None uiMode = iota
	Command
	Navigate
	Rename
	QuickNav
	SetQuickNav
)

type UiModel struct {
	list      list.Model
	input     textinput.Model
	items     []ListItemModel
	choice    string
	quitting  bool
	width     int
	height    int
	inputText string
	mode      uiMode
	kc        kitty.IKittyConnector
	qndb      settings.QuickNavDatabase
	sc        sessions.ISessionConnector
}

func (m UiModel) Init() tea.Cmd {
	return listWindows(m)
}

func (m UiModel) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.WindowSizeMsg:
		return handleWindowResized(m, msg)
	case ListUpdatedMsg:
		return m, listWindows(m)
	case ListWindowsMsg:
		return handleWindowList(m, msg)
	case ExitMessage:
		return m, tea.Quit
	case QuickNavsUpdatedMsg:
		return handleQuickNavDatabase(m, msg)
	default:
		switch m.mode {
		case Navigate:
			mdl, c := NavigateModeUpdate(m, msg)
			return mdl, c
		case Rename:
			mdl, c := RenameModeUpdate(m, msg)
			return mdl, c
		case QuickNav:
			mdl, c := QuickNavModeUpdate(m, msg)
			return mdl, c
		case SetQuickNav:
			mdl, c := SetQuickNavModeUpdate(m, msg)
			return mdl, c
		default:
			return m, nil
		}
	}
}

func handleWindowResized(m UiModel, msg tea.WindowSizeMsg) (tea.Model, tea.Cmd) {
	m.list.SetWidth(msg.Width)
	m.width = msg.Width
	m.height = msg.Height
	m.list.SetHeight(msg.Height - 6)

	UpdateStylesWithWidth(msg.Width)
	return m, nil
}

func handleWindowList(m UiModel, msg ListWindowsMsg) (tea.Model, tea.Cmd) {
	items := []list.Item{}
	for _, item := range msg.ListItems {
		items = append(items, item)
	}

	shortcuts := m.qndb.ShortcutsByEntryId()
	items = assignShortcutKeys(items, shortcuts)
	m.list.SetItems(items)
	return m, nil
}

func assignShortcutKeys(items []list.Item, shortcuts map[string]string) []list.Item {
	newItems := []list.Item{}
	for _, i := range items {
		listItem := i.(ListItemModel)
		entryId := settings.EntryIdForEntry(listItem.listEntry)
		listItem.shortcutKey = shortcuts[entryId]
		newItems = append(newItems, listItem)
	}

	return newItems
}

func handleQuickNavDatabase(m UiModel, msg QuickNavsUpdatedMsg) (tea.Model, tea.Cmd) {
	items := m.list.Items()
	shortcuts := msg.qndb.ShortcutsByEntryId()

	items = assignShortcutKeys(items, shortcuts)
	m.list.SetItems(items)

	return m, nil
}

func (m UiModel) View() string {
	if m.choice != "" || m.quitting {
		return ""
	}

	box := OuterStyle(m.width, m.height-5).Render("\n" + m.list.View())
	smallerBox := FooterStyle(m.width).Render(m.input.View())
	return fmt.Sprintf("%s\n%s", box, smallerBox)
}
