package main

import (
	"fmt"
	"os"

	"github.com/charmbracelet/bubbles/list"
	"github.com/charmbracelet/bubbles/textinput"
	tea "github.com/charmbracelet/bubbletea"
)

const listHeight = 14

type uiMode int64

const (
	None uiMode = iota
	Command
	Navigate
	Rename
	QuickNav
	SetQuickNav
)

type model struct {
	list      list.Model
	input     textinput.Model
	items     []item
	choice    string
	quitting  bool
	width     int
	height    int
	inputText string
	mode      uiMode
}

func (m model) Init() tea.Cmd {
	return listWindows
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.WindowSizeMsg:
		return handleWindowResized(m, msg)
	case ListUpdatedMsg:
		return m, listWindows
	case ListWindowsMsg:
		return handleWindowList(m, msg)
	case ExitMessage:
		return m, tea.Quit
	default:
		switch m.mode {
		case Navigate:
			mdl, c := NavigateModeUpdate(m, msg)
			return mdl, c
		case Rename:
			mdl, c := RenameModeUpdate(m, msg)
			return mdl, c
		default:
			return m, nil
		}
	}
}

func handleWindowResized(m model, msg tea.WindowSizeMsg) (tea.Model, tea.Cmd) {
	m.list.SetWidth(msg.Width)
	m.width = msg.Width
	m.height = msg.Height
	m.list.SetHeight(msg.Height - 6)

	UpdateStylesWithWidth(msg.Width)
	return m, nil
}

func handleWindowList(m model, msg ListWindowsMsg) (tea.Model, tea.Cmd) {
	items := []list.Item{}
	for _, item := range msg.ListItems {
		items = append(items, item)
	}
	m.list.SetItems(items)
	return m, nil
}

func (m model) View() string {
	if m.choice != "" || m.quitting {
		return ""
	}

	box := OuterStyle(m.width, m.height-5).Render("\n" + m.list.View())
	smallerBox := FooterStyle(m.width).Render(m.input.View())
	return fmt.Sprintf("%s\n%s", box, smallerBox)
}

func main() {
	items := []list.Item{}

	const defaultWidth = 20

	l := list.New(items, ItemDelegate{}, defaultWidth, listHeight)
	l.SetShowStatusBar(false)
	l.SetShowTitle(false)
	l.SetShowHelp(true)
	l.SetFilteringEnabled(false)
	l.Styles.Title = TitleStyle
	l.Styles.PaginationStyle = PaginationStyle
	l.Styles.HelpStyle = HelpStyle

	i := textinput.New()
	i.Prompt = ""

	m := model{
		list:  l,
		input: i,
		mode:  Navigate,
	}

	if err := tea.NewProgram(m, tea.WithAltScreen()).Start(); err != nil {
		fmt.Println("Error running program:", err)
		os.Exit(1)
	}
}
