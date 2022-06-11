package main

import (
	"github.com/charmbracelet/bubbles/key"
	tea "github.com/charmbracelet/bubbletea"
)

type additionalListActions struct {
	PrevTab     key.Binding
	NextTab     key.Binding
	QuickNav    key.Binding
	SetQuickNav key.Binding
	Save        key.Binding
}

var AdditionalActions = additionalListActions{
	PrevTab: key.NewBinding(
		key.WithKeys("K"),
		key.WithHelp("K", "previous tab"),
	),
	NextTab: key.NewBinding(
		key.WithKeys("J"),
		key.WithHelp("J", "next tab"),
	),
	QuickNav: key.NewBinding(
		key.WithKeys("'"),
		key.WithHelp("'", "followed by <letter> to jump to a quick nav"),
	),
	SetQuickNav: key.NewBinding(
		key.WithKeys("m"),
		key.WithHelp("m", "followed by <letter> to set a quick nav"),
	),
	Save: key.NewBinding(
		key.WithKeys("s"),
		key.WithHelp("s", "(tabs only) save tab as session"),
	),
}

type itemActions struct {
	Rename key.Binding
	Delete key.Binding
}

var DefaultItemActions = itemActions{
	Rename: key.NewBinding(
		key.WithKeys("a"),
		key.WithHelp("a", "rename"),
	),
	Delete: key.NewBinding(
		key.WithKeys("x", tea.KeyDelete.String()),
		key.WithHelp("x/del", "close"),
	),
}

var lowercaseLetters = key.NewBinding(
	key.WithKeys("a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"),
)

var numbers = key.NewBinding(
	key.WithKeys("1", "2", "3", "4", "5", "6", "7", "8", "9", "0"),
)

var cancel = key.NewBinding(
	key.WithKeys("escape", "esc", "'"),
)
