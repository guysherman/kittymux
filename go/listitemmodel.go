package main

import (
	"github.com/guysherman/kittymux/kitty"
)

type ListItemModel struct {
	listEntry   kitty.WindowListEntry
	listMode    uiMode
	shortcutKey string
}

func (i ListItemModel) FilterValue() string { return i.listEntry.Text }
