package main

import (
	"testing"

	"github.com/charmbracelet/bubbles/list"
	"github.com/charmbracelet/bubbles/textinput"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/guysherman/kittymux/kitty"
	. "github.com/smartystreets/goconvey/convey"
)

func TestSetQuickNavMode(t *testing.T) {
	Convey("SetQuickNav Mode", t, func() {
		items := []list.Item{
			item{
				listEntry: kitty.WindowListEntry{
					Text:      "Tab 1",
					EntryType: kitty.Tab,
					Id:        1,
				},
			},
			item{
				listEntry: kitty.WindowListEntry{
					Text:      "Win 1",
					EntryType: kitty.Window,
					Id:        2,
				},
			},
			item{
				listEntry: kitty.WindowListEntry{
					Text:      "Tab 2",
					EntryType: kitty.Tab,
				},
			},
			item{
				listEntry: kitty.WindowListEntry{
					Text:      "Win 2",
					EntryType: kitty.Window,
				},
			},
		}
		l := list.New(items, ItemDelegate{}, 0, 0)
		i := textinput.New()
		i.Prompt = ""
		m := model{list: l, input: i, mode: SetQuickNav}

		Convey("Selecting a letter sets the shortcut key on an entry", func() {
			msg := tea.KeyMsg{
				Type:  tea.KeyRunes,
				Runes: []rune{'a'},
				Alt:   false,
			}

			cw := kitty.MockCommandExecutor{}
			kc := kitty.NewKittyConnector(&cw)
			m.kc = kc

			newModel, cmd := SetQuickNavModeUpdate(m, msg)
			listItems := newModel.(model).list.Items()
			i := listItems[0].(item)
			So(cmd, ShouldBeNil)
			So(i.shortcutKey, ShouldEqual, "a")
			So(newModel.(model).mode, ShouldEqual, Navigate)
			So(newModel.(model).list.Items()[0].(item).listMode, ShouldEqual, Navigate)
		})

		Convey("Selecting a number sets the shortcut key on an entry", func() {
			msg := tea.KeyMsg{
				Type:  tea.KeyRunes,
				Runes: []rune{'1'},
				Alt:   false,
			}

			cw := kitty.MockCommandExecutor{}
			kc := kitty.NewKittyConnector(&cw)
			m.kc = kc
			m.list.Select(3)

			newModel, cmd := SetQuickNavModeUpdate(m, msg)
			listItems := newModel.(model).list.Items()
			i := listItems[3].(item)
			So(cmd, ShouldBeNil)
			So(i.shortcutKey, ShouldEqual, "1")
		})

		Convey("pressing esc returns to normal", func() {
			msg := tea.KeyMsg{
				Type: tea.KeyEscape,
				Alt:  false,
			}

			cw := kitty.MockCommandExecutor{}
			kc := kitty.NewKittyConnector(&cw)
			m.kc = kc

			newModel, cmd := SetQuickNavModeUpdate(m, msg)
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
