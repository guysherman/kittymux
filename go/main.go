package main

import (
	"flag"
	"fmt"
	"os"

	"github.com/charmbracelet/bubbles/key"
	"github.com/charmbracelet/bubbles/list"
	"github.com/charmbracelet/bubbles/textinput"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/guysherman/kittymux/kitty"
	"github.com/guysherman/kittymux/sessions"
	"github.com/guysherman/kittymux/settings"
)

func main() {
	session := flag.String("session", "", "the name of the session to restore")
	flag.Parse()

	if *session != "" {
		cliMode(session)
	}

	interactiveMode()
}

func cliMode(session *string) {
	ce := kitty.KittyCommandExecutor{}
	kc := kitty.NewKittyConnector(&ce)
	qnd := settings.QuickNavDao{}
	qndb := settings.NewQuickNavDatabase(&qnd)

	sd := sessions.SessionDao{}
	sc := sessions.NewSessionConnector(&sd, kc, qndb)

	sc.LoadSession(*session)
	os.Exit(0)
}

func interactiveMode() {
	items := []list.Item{}

	const defaultWidth = 20
	const listHeight = 14

	l := list.New(items, ItemDelegate{}, defaultWidth, listHeight)
	l.SetShowStatusBar(false)
	l.SetShowTitle(false)
	l.SetShowHelp(true)
	l.SetFilteringEnabled(false)
	l.Styles.HelpStyle = HelpStyle
	l.AdditionalFullHelpKeys = func() []key.Binding {
		return []key.Binding{
			AdditionalActions.PrevTab,
			AdditionalActions.NextTab,
			AdditionalActions.QuickNav,
			AdditionalActions.SetQuickNav,
			AdditionalActions.Save,
		}
	}

	i := textinput.New()
	i.Prompt = ""

	ce := kitty.KittyCommandExecutor{}
	kc := kitty.NewKittyConnector(&ce)
	qnd := settings.QuickNavDao{}
	qndb := settings.NewQuickNavDatabase(&qnd)

	sd := sessions.SessionDao{}
	sc := sessions.NewSessionConnector(&sd, kc, qndb)

	m := UiModel{
		list:  l,
		input: i,
		mode:  Navigate,
		kc:    kc,
		qndb:  qndb,
		sc:    sc,
	}

	if err := tea.NewProgram(m, tea.WithAltScreen()).Start(); err != nil {
		fmt.Println("Error running program:", err)
		os.Exit(1)
	}
}
