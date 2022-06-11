package main

import (
	"fmt"
	"testing"

	"github.com/charmbracelet/bubbles/list"
	"github.com/charmbracelet/bubbles/textinput"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/guysherman/kittymux/kitty"
	. "github.com/smartystreets/goconvey/convey"
)

func TestRenameMode(t *testing.T) {
	Convey("Rename Mode", t, func() {
		items := []list.Item{
			ListItemModel{
				listEntry: kitty.WindowListEntry{
					Text:      "Tab 1",
					EntryType: kitty.Tab,
					Id:        1,
				},
				shortcutKey: "a",
			},
			ListItemModel{
				listEntry: kitty.WindowListEntry{
					Text:      "Win 1",
					EntryType: kitty.Window,
					Id:        2,
				},
				shortcutKey: "1",
			},
			ListItemModel{
				listEntry: kitty.WindowListEntry{
					Text:      "Tab 2",
					EntryType: kitty.Tab,
				},
			},
			ListItemModel{
				listEntry: kitty.WindowListEntry{
					Text:      "Win 2",
					EntryType: kitty.Window,
				},
			},
		}
		l := list.New(items, ItemDelegate{}, 0, 0)
		i := textinput.New()
		i.SetValue("Foozle")
		i.Prompt = ""
		m := model{list: l, input: i, mode: Rename}

		Convey("Renames entry when enter is pressed", func() {
			msg := tea.KeyMsg{
				Type: tea.KeyEnter,
				Alt:  false,
			}

			cw := kitty.MockCommandExecutor{}
			kc := kitty.NewKittyConnector(&cw)
			m.kc = kc

			newModel, cmd := RenameModeUpdate(m, msg)
			newMsg := cmd()
			So(newModel.(model).inputText, ShouldEqual, "Foozle")
			So(fmt.Sprintf("%T", newMsg), ShouldEqual, fmt.Sprintf("%T", ListUpdatedMsg{}))
		})

		Convey("Returns to navigate mode when Esc is pressed", func() {
			msg := tea.KeyMsg{
				Type: tea.KeyEscape,
				Alt:  false,
			}

			cw := kitty.MockCommandExecutor{}
			kc := kitty.NewKittyConnector(&cw)
			m.kc = kc

			newModel, cmd := RenameModeUpdate(m, msg)
			So(cmd, ShouldBeNil)
			So(newModel.(model).mode, ShouldEqual, Navigate)
		})

	})
}
