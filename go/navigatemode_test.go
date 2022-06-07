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

func TestNavigateMode(t *testing.T) {
	Convey("Navigate Mode", t, func() {
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
		m := model{list: l, input: i}

		Convey("Shift J selects tab 2", func() {
			msg := tea.KeyMsg{
				Type:  tea.KeyRunes,
				Runes: []rune{'J'},
				Alt:   false,
			}

			m, cmd := NavigateModeUpdate(m, msg)
			So(m.(model).list.Index(), ShouldEqual, 2)
			So(cmd, ShouldBeNil)
		})

		Convey("Shift K selects tab 2", func() {
			l.Select(2)
			msg := tea.KeyMsg{
				Type:  tea.KeyRunes,
				Runes: []rune{'K'},
				Alt:   false,
			}

			m, cmd := NavigateModeUpdate(m, msg)
			So(m.(model).list.Index(), ShouldEqual, 0)
			So(cmd, ShouldBeNil)
		})

		Convey("a enters rename mode", func() {
			msg := tea.KeyMsg{
				Type:  tea.KeyRunes,
				Runes: []rune{'a'},
				Alt:   false,
			}

			newM, cmd := NavigateModeUpdate(m, msg)
			So(newM.(model).mode, ShouldEqual, Rename)
			So(newM.(model).input.Focused(), ShouldBeTrue)
			So(cmd, ShouldBeNil)
		})

		Convey("ctrl+c quits", func() {
			msg := tea.KeyMsg{
				Type: tea.KeyCtrlC,
				Alt:  false,
			}

			newM, cmd := NavigateModeUpdate(m, msg)
			So(newM.(model).quitting, ShouldBeTrue)
			So(cmd, ShouldNotBeNil)
		})

		Convey("enter focuses entry", func() {
			msg := tea.KeyMsg{
				Type: tea.KeyEnter,
				Alt:  false,
			}

			cw := kitty.MockCommandExecutor{}
			kc := kitty.NewKittyConnector(&cw)
			m.kc = kc

			_, cmd := NavigateModeUpdate(m, msg)
			newMsg := cmd()
			So(fmt.Sprintf("%T", newMsg), ShouldResemble, fmt.Sprintf("%T", ExitMessage{}))
			So(cw.GetSavedArgs()[0], ShouldResemble, []string{"focus-tab", "-m", "id:1"})
		})

		Convey("x deletes an entry", func() {
			msg := tea.KeyMsg{
				Type:  tea.KeyRunes,
				Runes: []rune{'x'},
				Alt:   false,
			}

			cw := kitty.MockCommandExecutor{}
			kc := kitty.NewKittyConnector(&cw)
			m.kc = kc

			_, cmd := NavigateModeUpdate(m, msg)
			newMsg := cmd()
			So(fmt.Sprintf("%T", newMsg), ShouldResemble, fmt.Sprintf("%T", ListUpdatedMsg{}))
			So(cw.GetSavedArgs()[0], ShouldResemble, []string{"close-tab", "-m", "id:1"})
		})

		Reset(func() {
			l = list.New(items, ItemDelegate{}, 0, 0)
			i = textinput.New()
			i.Prompt = ""
			m = model{list: l, input: i}
		})
	})
}
