package main

import "github.com/guysherman/kittymux/settings"

type ExitMessage struct{}

type ListWindowsMsg struct {
	ListItems []ListItemModel
}

type NoopMsg struct{}

type ListUpdatedMsg struct{}

type QuickNavsUpdatedMsg struct {
	qndb settings.QuickNavDatabase
}
