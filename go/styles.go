package main

import (
	"github.com/charmbracelet/bubbles/list"
	"github.com/charmbracelet/lipgloss"
)

var (
	ItemStyle         = makeItemStyle(false, 50)
	SelectedItemStyle = makeItemStyle(true, 50)
	SelectedTextStyle = lipgloss.NewStyle().Background(lipgloss.Color("4"))
	ShortcutStyle     = lipgloss.NewStyle().Background(lipgloss.Color("7")).Foreground(lipgloss.Color("0"))
	HelpStyle         = list.DefaultStyles().HelpStyle.PaddingLeft(4)
	OuterStyle        = func(width int, height int) lipgloss.Style {
		return lipgloss.NewStyle().Width(width - 2).Height(height).Border(lipgloss.RoundedBorder())
	}
	FooterStyle = func(width int) lipgloss.Style {
		return lipgloss.NewStyle().Width(width - 2).Height(1).Border(lipgloss.RoundedBorder())
	}
)

func makeItemStyle(selected bool, width int) lipgloss.Style {
	if selected {
		return lipgloss.NewStyle().Width(width - 2).Background(lipgloss.Color("4"))
	} else {
		return lipgloss.NewStyle().Width(width - 2)
	}
}

func UpdateStylesWithWidth(width int) {
	ItemStyle = makeItemStyle(false, width)
	SelectedItemStyle = makeItemStyle(true, width)
}
