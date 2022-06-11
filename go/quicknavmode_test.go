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

func TestQuickNavMode(t *testing.T) {
	Convey("QuickNav Mode", t, func() {
		items := []list.Item{
			item{
				listEntry: kitty.WindowListEntry{
					Text:              "Tab 1",
					EntryType:         kitty.Tab,
					Id:                1,
					OsWindowIsFocused: true,
					TabIsFocused:      false,
					IsFocused:         false,
				},
				shortcutKey: "a",
			},
			item{
				listEntry: kitty.WindowListEntry{
					Text:              "Win 1",
					EntryType:         kitty.Window,
					Id:                2,
					OsWindowIsFocused: true,
					TabIsFocused:      false,
					IsFocused:         false,
				},
				shortcutKey: "1",
			},
			item{
				listEntry: kitty.WindowListEntry{
					Text:              "Tab 2",
					EntryType:         kitty.Tab,
					Id:                3,
					OsWindowIsFocused: true,
					TabIsFocused:      true,
					IsFocused:         false,
				},
				shortcutKey: "a",
			},
			item{
				listEntry: kitty.WindowListEntry{
					Text:              "Win 2",
					EntryType:         kitty.Window,
					Id:                4,
					OsWindowIsFocused: true,
					TabIsFocused:      true,
					IsFocused:         false,
				},
				shortcutKey: "1",
			},
		}
		l := list.New(items, ItemDelegate{}, 0, 0)
		i := textinput.New()
		i.Prompt = ""
		m := model{list: l, input: i, mode: QuickNav}

		Convey("Selecting a letter focuses an entry, within the focused tab", func() {
			msg := tea.KeyMsg{
				Type:  tea.KeyRunes,
				Runes: []rune{'a'},
				Alt:   false,
			}

			cw := kitty.MockCommandExecutor{}
			kc := kitty.NewKittyConnector(&cw)
			m.kc = kc

			_, cmd := QuickNavModeUpdate(m, msg)
			newMsg := cmd()
			So(fmt.Sprintf("%T", newMsg), ShouldResemble, fmt.Sprintf("%T", ExitMessage{}))
			So(cw.GetSavedArgs()[0], ShouldResemble, []string{"focus-tab", "-m", "id:3"})
		})

		Convey("Selecting a number focuses an entry, within the focused tab", func() {
			msg := tea.KeyMsg{
				Type:  tea.KeyRunes,
				Runes: []rune{'1'},
				Alt:   false,
			}

			cw := kitty.MockCommandExecutor{}
			kc := kitty.NewKittyConnector(&cw)
			m.kc = kc

			_, cmd := QuickNavModeUpdate(m, msg)
			newMsg := cmd()
			So(fmt.Sprintf("%T", newMsg), ShouldResemble, fmt.Sprintf("%T", ExitMessage{}))
			So(cw.GetSavedArgs()[0], ShouldResemble, []string{"focus-window", "-m", "id:4"})
		})

		Convey("pressing esc returns to normal", func() {
			msg := tea.KeyMsg{
				Type: tea.KeyEscape,
				Alt:  false,
			}

			cw := kitty.MockCommandExecutor{}
			kc := kitty.NewKittyConnector(&cw)
			m.kc = kc

			newModel, cmd := QuickNavModeUpdate(m, msg)
			So(cmd, ShouldBeNil)
			So(newModel.(model).mode, ShouldEqual, Navigate)
			So(newModel.(model).list.Items()[0].(item).listMode, ShouldEqual, Navigate)
		})

		Reset(func() {
			l = list.New(items, ItemDelegate{}, 0, 0)
			i = textinput.New()
			i.Prompt = ""
			m = model{list: l, input: i}
		})

	})
}
