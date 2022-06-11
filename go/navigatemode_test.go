package main

import (
	"fmt"
	"testing"

	"github.com/charmbracelet/bubbles/list"
	"github.com/charmbracelet/bubbles/textinput"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/guysherman/kittymux/kitty"
	"github.com/guysherman/kittymux/sessions"
	"github.com/guysherman/kittymux/settings"
	. "github.com/smartystreets/goconvey/convey"
)

func TestNavigateMode(t *testing.T) {
	Convey("Navigate Mode", t, func() {
		tab := kitty.KittyTab{
			Id:     1,
			Layout: "stack",
			Title:  "Tab1",
			Windows: []kitty.KittyWindow{
				{
					Id:    1,
					Title: "Win1",
					Cwd:   "/foo",
					Foreground_processes: []kitty.KittyForegroundProcessHandle{
						{
							Cmdline: []string{"nvim", "."},
							Cwd:     "/foo",
						},
					},
				},
			},
		}

		items := []list.Item{
			ListItemModel{
				listEntry: kitty.WindowListEntry{
					Text:      "Tab1",
					EntryType: kitty.Tab,
					Id:        1,
					Tab:       &tab,
				},
				shortcutKey: "a",
			},
			ListItemModel{
				listEntry: kitty.WindowListEntry{
					Text:      "Win1",
					EntryType: kitty.Window,
					Id:        1,
					Tab:       &tab,
				},
				shortcutKey: "b",
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

		Convey("' enters quicknav mode", func() {
			msg := tea.KeyMsg{
				Type:  tea.KeyRunes,
				Runes: []rune{'\''},
				Alt:   false,
			}

			newM, cmd := NavigateModeUpdate(m, msg)
			So(newM.(model).mode, ShouldEqual, QuickNav)
			So(newM.(model).list.Items()[0].(ListItemModel).listMode, ShouldEqual, QuickNav)
			So(cmd, ShouldBeNil)
		})

		Convey("m enters SetQuickNav mode", func() {
			msg := tea.KeyMsg{
				Type:  tea.KeyRunes,
				Runes: []rune{'m'},
				Alt:   false,
			}

			newM, cmd := NavigateModeUpdate(m, msg)
			So(newM.(model).mode, ShouldEqual, SetQuickNav)
			So(newM.(model).list.Items()[0].(ListItemModel).listMode, ShouldEqual, SetQuickNav)
			So(cmd, ShouldBeNil)
		})

		Convey("when a tab is selected, s saves a session", func() {
			msg := tea.KeyMsg{
				Type:  tea.KeyRunes,
				Runes: []rune{'s'},
				Alt:   false,
			}

			qn := settings.MockQuickNavDao{}
			qn.SetReadReturnValue(settings.MockQuickNavDaoReadReturn{
				Db: settings.QuickNavDatabase{
					QuickNavs: map[string][]settings.QuickNavHandle{
						"a": {
							{
								EntryId:   1,
								EntryType: kitty.Tab,
							},
						},
						"b": {
							{
								EntryId:   1,
								EntryType: kitty.Window,
							},
						},
					},
				},
			})
			qnd := settings.NewQuickNavDatabase(&qn)

			ce := kitty.MockCommandExecutor{}
			kc := kitty.NewKittyConnector(&ce)

			sd := sessions.MockSessionDao{}
			sc := sessions.NewSessionConnector(&sd, kc, qnd)

			m.sc = sc

			_, cmd := NavigateModeUpdate(m, msg)
			cmd()

			So(sd.Calls.Write[0].Session, ShouldResemble, sessions.Session{
				Title:       "Tab1",
				ShortcutKey: "a",
				Windows: []sessions.Window{
					{
						Title:       "Win1",
						ShortcutKey: "b",
						ForegroundProcess: sessions.ProcessHandle{
							Args: []string{"nvim", "."},
							Cwd:  "/foo",
						},
						Cwd: "/foo",
					},
				},
				Layout: "stack",
			})
			So(sd.Calls.Write[0].Filepath, ShouldEndWith, "Tab1.json")

		})

		Convey("when a tab is not selected, s does nothing", func() {
			msg := tea.KeyMsg{
				Type:  tea.KeyRunes,
				Runes: []rune{'s'},
				Alt:   false,
			}

			qn := settings.MockQuickNavDao{}
			qnd := settings.NewQuickNavDatabase(&qn)

			ce := kitty.MockCommandExecutor{}
			kc := kitty.NewKittyConnector(&ce)

			sd := sessions.MockSessionDao{}
			sc := sessions.NewSessionConnector(&sd, kc, qnd)

			m.sc = sc
			m.list.Select(1)

			_, cmd := NavigateModeUpdate(m, msg)

			So(cmd, ShouldBeNil)
			So(len(sd.GetCalls().Write), ShouldEqual, 0)
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

		Convey("del deletes an entry", func() {
			msg := tea.KeyMsg{
				Type: tea.KeyDelete,
				Alt:  false,
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
