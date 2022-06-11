package main

import (
	"fmt"
	"testing"

	"github.com/charmbracelet/bubbles/list"
	"github.com/charmbracelet/bubbles/textinput"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/guysherman/kittymux/kitty"
	"github.com/guysherman/kittymux/settings"
	. "github.com/smartystreets/goconvey/convey"
)

func TestSetQuickNavMode(t *testing.T) {
	Convey("SetQuickNav Mode", t, func() {
		items := []list.Item{
			ListItemModel{
				listEntry: kitty.WindowListEntry{
					Text:      "Tab 1",
					EntryType: kitty.Tab,
					Id:        1,
				},
			},
			ListItemModel{
				listEntry: kitty.WindowListEntry{
					Text:      "Win 1",
					EntryType: kitty.Window,
					Id:        2,
					Tab: &kitty.KittyTab{
						Id: 1,
					},
				},
			},
			ListItemModel{
				listEntry: kitty.WindowListEntry{
					Text:      "Tab 2",
					EntryType: kitty.Tab,
					Id:        3,
				},
			},
			ListItemModel{
				listEntry: kitty.WindowListEntry{
					Text:      "Win 2",
					EntryType: kitty.Window,
					Id:        4,
					Tab: &kitty.KittyTab{
						Id: 3,
					},
				},
			},
		}
		l := list.New(items, ItemDelegate{}, 0, 0)
		i := textinput.New()
		i.Prompt = ""
		m := UiModel{list: l, input: i, mode: SetQuickNav}

		Convey("Selecting a letter updates the quick nav database", func() {
			msg := tea.KeyMsg{
				Type:  tea.KeyRunes,
				Runes: []rune{'a'},
				Alt:   false,
			}

			cw := kitty.MockCommandExecutor{}
			kc := kitty.NewKittyConnector(&cw)
			qndao := settings.MockQuickNavDao{}
			qndao.SetReadReturnValue(settings.MockQuickNavDaoReadReturn{
				Db: settings.QuickNavDatabase{
					QuickNavs: map[string][]settings.QuickNavHandle{},
				},
				Err: nil,
			})
			qndb := settings.NewQuickNavDatabase(&qndao)
			m.qndb = qndb
			m.kc = kc

			newModel, cmd := SetQuickNavModeUpdate(m, msg)
			newMsg := cmd()
			So(fmt.Sprintf("%T", newMsg), ShouldEqual, fmt.Sprintf("%T", QuickNavsUpdatedMsg{}))
			So(newMsg.(QuickNavsUpdatedMsg).qndb.QuickNavs["a"][0], ShouldResemble, settings.QuickNavHandle{EntryId: 1, EntryType: kitty.Tab})
			So(newModel.(UiModel).mode, ShouldEqual, Navigate)
			So(newModel.(UiModel).list.Items()[0].(ListItemModel).listMode, ShouldEqual, Navigate)
		})

		Convey("Selecting a number updates the quick nav database", func() {
			msg := tea.KeyMsg{
				Type:  tea.KeyRunes,
				Runes: []rune{'1'},
				Alt:   false,
			}

			cw := kitty.MockCommandExecutor{}
			kc := kitty.NewKittyConnector(&cw)
			qndao := settings.MockQuickNavDao{}
			qndao.SetReadReturnValue(settings.MockQuickNavDaoReadReturn{
				Db: settings.QuickNavDatabase{
					QuickNavs: map[string][]settings.QuickNavHandle{},
				},
				Err: nil,
			})
			qndb := settings.NewQuickNavDatabase(&qndao)
			m.qndb = qndb
			m.kc = kc
			m.list.Select(3)

			newModel, cmd := SetQuickNavModeUpdate(m, msg)
			newMsg := cmd()
			So(fmt.Sprintf("%T", newMsg), ShouldEqual, fmt.Sprintf("%T", QuickNavsUpdatedMsg{}))
			So(newMsg.(QuickNavsUpdatedMsg).qndb.QuickNavs["1"][0], ShouldResemble, settings.QuickNavHandle{EntryId: 4, EntryType: kitty.Window})
			So(newModel.(UiModel).mode, ShouldEqual, Navigate)
			So(newModel.(UiModel).list.Items()[0].(ListItemModel).listMode, ShouldEqual, Navigate)
		})

		Convey("assigning the same shortcut to a second window in the same tab removes it from the first", func() {
			cw := kitty.MockCommandExecutor{}
			kc := kitty.NewKittyConnector(&cw)
			qndao := settings.MockQuickNavDao{}
			qndao.SetReadReturnValue(settings.MockQuickNavDaoReadReturn{
				Db: settings.QuickNavDatabase{
					QuickNavs: map[string][]settings.QuickNavHandle{
						"a": {
							{
								EntryId:   1,
								EntryType: kitty.Tab,
							},
						},
					},
				},
				Err: nil,
			})
			qndb := settings.NewQuickNavDatabase(&qndao)
			m.qndb = qndb
			m.kc = kc
			item1 := m.list.Items()[0].(ListItemModel)
			item1.shortcutKey = "a"
			m.list.SetItem(0, item1)
			m.list.Select(1)

			msg := tea.KeyMsg{
				Type:  tea.KeyRunes,
				Runes: []rune{'a'},
				Alt:   false,
			}

			_, cmd := SetQuickNavModeUpdate(m, msg)
			qnum := cmd()
			qndb = qnum.(QuickNavsUpdatedMsg).qndb
			byEntryId := qndb.ShortcutsByEntryId()
			So(byEntryId["w:2"], ShouldEqual, "a")
			So(byEntryId["t:1"], ShouldEqual, "")

		})

		Convey("assigning the same shortcut to a second window in a different tab adds it as normal", func() {
			cw := kitty.MockCommandExecutor{}
			kc := kitty.NewKittyConnector(&cw)
			qndao := settings.MockQuickNavDao{}
			qndao.SetReadReturnValue(settings.MockQuickNavDaoReadReturn{
				Db: settings.QuickNavDatabase{
					QuickNavs: map[string][]settings.QuickNavHandle{
						"a": {
							{
								EntryId:   1,
								EntryType: kitty.Tab,
							},
						},
					},
				},
				Err: nil,
			})
			qndb := settings.NewQuickNavDatabase(&qndao)
			m.qndb = qndb
			m.kc = kc
			item1 := m.list.Items()[0].(ListItemModel)
			item1.shortcutKey = "a"
			m.list.SetItem(0, item1)
			m.list.Select(3)

			msg := tea.KeyMsg{
				Type:  tea.KeyRunes,
				Runes: []rune{'a'},
				Alt:   false,
			}

			_, cmd := SetQuickNavModeUpdate(m, msg)
			qnum := cmd()
			qndb = qnum.(QuickNavsUpdatedMsg).qndb
			byEntryId := qndb.ShortcutsByEntryId()
			So(byEntryId["w:4"], ShouldEqual, "a")
			So(byEntryId["t:1"], ShouldEqual, "a")

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
			So(newModel.(UiModel).mode, ShouldEqual, Navigate)
			So(newModel.(UiModel).list.Items()[0].(ListItemModel).listMode, ShouldEqual, Navigate)
		})

		Reset(func() {
			l = list.New(items, ItemDelegate{}, 0, 0)
			i = textinput.New()
			i.Prompt = ""
			m = UiModel{list: l, input: i}
		})

	})
}
