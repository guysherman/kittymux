package main

import (
	"testing"

	"github.com/charmbracelet/bubbles/list"
	"github.com/charmbracelet/bubbles/textinput"
	"github.com/guysherman/kittymux/kitty"
	"github.com/guysherman/kittymux/settings"
	. "github.com/smartystreets/goconvey/convey"
)

func TestMainUpdate(t *testing.T) {
	Convey("Main Update Function", t, func() {
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
					Id:        3,
				},
			},
			item{
				listEntry: kitty.WindowListEntry{
					Text:      "Win 2",
					EntryType: kitty.Window,
					Id:        4,
				},
			},
		}
		l := list.New(items, ItemDelegate{}, 0, 0)
		i := textinput.New()
		i.Prompt = ""
		m := model{list: l, input: i, mode: Navigate}

		Convey("QuickNavsUpdatedMessage assigns shortcut keys to model", func() {
			qndb := settings.QuickNavDatabase{
				QuickNavs: map[string][]settings.QuickNavHandle{
					"a": {
						{
							EntryId:   1,
							EntryType: kitty.Tab,
						},
						{
							EntryId:   4,
							EntryType: kitty.Window,
						},
					},
				},
			}

			msg := QuickNavsUpdatedMsg{
				qndb: qndb,
			}

			newm, cmd := m.Update(msg)
			So(cmd, ShouldBeNil)
			newModel := newm.(model)
			So(newModel.list.Items()[0].(item).shortcutKey, ShouldEqual, "a")
			So(newModel.list.Items()[1].(item).shortcutKey, ShouldEqual, "")
			So(newModel.list.Items()[3].(item).shortcutKey, ShouldEqual, "a")

		})
	})
}
