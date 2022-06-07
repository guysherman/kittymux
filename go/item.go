package main

import (
	"fmt"
	"io"

	"github.com/charmbracelet/bubbles/key"
	"github.com/charmbracelet/bubbles/list"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/guysherman/kittymux/kitty"
)

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

type item struct {
	listEntry kitty.WindowListEntry
}

func (i item) FilterValue() string { return i.listEntry.Text }

type ItemDelegate struct{}

func (d ItemDelegate) Height() int  { return 1 }
func (d ItemDelegate) Spacing() int { return 0 }
func (d ItemDelegate) Update(msg tea.Msg, m *list.Model) tea.Cmd {
	return nil
}
func (d ItemDelegate) Render(w io.Writer, m list.Model, index int, listItem list.Item) {
	i, ok := listItem.(item)
	if !ok {
		return
	}

	str := fmt.Sprintf("%s", i.listEntry.Text)

	fn := ItemStyle.Render
	if index == m.Index() {
		fn = func(s string) string {
			return SelectedItemStyle.Render("> " + s)
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
