package main

import (
	"fmt"
	"io"

	"github.com/charmbracelet/bubbles/key"
	"github.com/charmbracelet/bubbles/list"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
)

type ItemDelegate struct{}

func (d ItemDelegate) Height() int  { return 1 }
func (d ItemDelegate) Spacing() int { return 0 }
func (d ItemDelegate) Update(msg tea.Msg, m *list.Model) tea.Cmd {
	return nil
}

func (d ItemDelegate) Render(w io.Writer, m list.Model, index int, listItem list.Item) {
	i, ok := listItem.(ListItemModel)
	if !ok {
		return
	}

	selected := index == m.Index()

	str := fmt.Sprintf("%s", i.listEntry.Text)
	if i.listMode == QuickNav || i.listMode == SetQuickNav {
		shortcutKey := " "
		if i.shortcutKey != "" {
			shortcutKey = i.shortcutKey
		}
		itemText := lipgloss.NewStyle().SetString(str)
		if selected {
			itemText = SelectedTextStyle.Copy().SetString(fmt.Sprintf("%s", str))
		}
		sc := ShortcutStyle.Copy().SetString(fmt.Sprintf("%s", shortcutKey))
		str = fmt.Sprintf("%s%s", sc, itemText)
	}

	fn := ItemStyle.Render
	if index == m.Index() {
		fn = func(s string) string {
			return SelectedItemStyle.Render(s)
		}
	}

	fmt.Fprintf(w, fn(str))
}

func (d ItemDelegate) ShortHelp() []key.Binding {
	return []key.Binding{
		DefaultItemActions.Rename,
		DefaultItemActions.Delete,
	}
}

func (d ItemDelegate) FullHelp() [][]key.Binding {
	return [][]key.Binding{
		{
			DefaultItemActions.Rename,
			DefaultItemActions.Delete,
		},
	}
}
