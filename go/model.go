package main

import (
	"github.com/charmbracelet/bubbles/list"
	"github.com/charmbracelet/bubbles/textinput"
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

