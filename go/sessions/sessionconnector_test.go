package sessions

import (
	"os"
	"testing"

	"github.com/guysherman/kittymux/kitty"
	"github.com/guysherman/kittymux/settings"
	. "github.com/smartystreets/goconvey/convey"
)

func TestLoadSession(t *testing.T) {
	Convey("Load saved session", t, func() {
		Convey("Load single new tab", func() {
			msd := &MockSessionDao{}
			mce := &kitty.MockCommandExecutor{}
			qnd := &settings.MockQuickNavDao{}
			kc := kitty.NewKittyConnector(mce)
			sc := NewSessionConnector(msd, kc, qnd)

			testSession := Session{
				Title:       "Test",
				ShortcutKey: "a",
				Windows: []Window{
					Window{
						Title:       "Test Window",
						ShortcutKey: "b",
						ForegroundProcess: ProcessHandle{
							Args: []string{"nvim", "."},
							Cwd:  "~/",
						},
						Cwd: "~/",
					},
				},
			}

			msd.SetReadReturnValue(MockSessionDaoReadReturn{session: testSession, err: nil})
			mce.SetReturnValueOnce("29")
			mce.SetReturnValueOnce(`
      [{
        "id": 1,
        "is_focused": true,
        "platform_window_id": 77594671,
        "tabs": [
        {
          "id": 1,
          "active_window_history": [1],
          "enabled_layouts": ["stack"],
          "layout": "stack",
          "title": "tab1",
          "windows": [
          {
            "cmdline": [
              "/usr/bin/zsh"
            ],
            "title": "win1",
            "id": 1
          }
          ]
        }, 
        {
          "id": 2,
          "active_window_history": [2],
          "enabled_layouts": ["stack"],
          "layout": "stack",
          "title": "Test",
          "windows": [{
            "cmdline": ["/usr/bin/sh"],
            "title": "Test Window",
            "id": 29 
          }]
        }
        ]
      }]
      `)

			mce.SetReturnValueOnce("")
			mce.SetReturnValueOnce("")

			qnd.SetReadReturnValue(settings.MockQuickNavDaoReadReturn{
				Db: settings.QuickNavDatabase{
					QuickNavs: map[string][]settings.QuickNavHandle{},
				},
				Err: nil,
			})

			expectedQuickNavs := settings.QuickNavDatabase{
				QuickNavs: map[string][]settings.QuickNavHandle{
					"a": {
						{
							EntryId:   2,
							EntryType: kitty.Tab,
						},
					},
					"b": {
						{
							EntryId:   29,
							EntryType: kitty.Window,
						},
					},
				},
			}

			sc.LoadSession("sessionName")

			msdCalls := msd.GetCalls()
			os.Setenv("KITTYMUX_STATE_DIR", "/foo/")
			So(msdCalls.Read[0].filepath, ShouldEndWith, "sessionName.json")

			qndCalls := qnd.GetCalls()
			So(qndCalls.Read.Filepath, ShouldEndWith, "quicknavs.json")

			cmds := mce.GetSavedArgs()
			So(cmds[0], ShouldResemble, []string{"new-window", "--title", "Test Window", "--new-tab", "--tab-title", "\"Test\"", "--cwd", "~/"})
			So(cmds[1], ShouldResemble, []string{"ls"})
			So(cmds[2], ShouldResemble, []string{"focus-tab", "-m", "id:2"})
			So(cmds[3], ShouldResemble, []string{"send-text", "-m", "id:29", "'cd ~/\\n'"})
			So(cmds[4], ShouldResemble, []string{"send-text", "-m", "id:29", "'nvim .\\n'"})

			So(qndCalls.Write.Filepath, ShouldEndWith, "quicknavs.json")
			So(qndCalls.Write.QuickNavs, ShouldResemble, expectedQuickNavs)

		})

		Convey("Load new tab with two windows", func() {
			msd := &MockSessionDao{}
			mce := &kitty.MockCommandExecutor{}
			qnd := &settings.MockQuickNavDao{}
			kc := kitty.NewKittyConnector(mce)
			sc := NewSessionConnector(msd, kc, qnd)

			testSession := Session{
				Title:       "Test",
				ShortcutKey: "a",
				Windows: []Window{
					{
						Title:       "Test Window",
						ShortcutKey: "b",
						ForegroundProcess: ProcessHandle{
							Args: []string{"nvim", "."},
							Cwd:  "~/",
						},
						Cwd: "~/",
					},
					{
						Title:       "Test Window 2",
						ShortcutKey: "c",
						ForegroundProcess: ProcessHandle{
							Args: []string{"foo", "."},
							Cwd:  "~/",
						},
						Cwd: "~/bar",
					},
				},
			}

			msd.SetReadReturnValue(MockSessionDaoReadReturn{session: testSession, err: nil})

			// Create window
			mce.SetReturnValueOnce("29")

			// ls
			mce.SetReturnValueOnce(`
      [{
        "id": 1,
        "is_focused": true,
        "platform_window_id": 77594671,
        "tabs": [
        {
          "id": 1,
          "active_window_history": [1],
          "enabled_layouts": ["stack"],
          "layout": "stack",
          "title": "tab1",
          "windows": [
          {
            "cmdline": [
              "/usr/bin/zsh"
            ],
            "title": "win1",
            "id": 1
          }
          ]
        }, 
        {
          "id": 2,
          "active_window_history": [2],
          "enabled_layouts": ["stack"],
          "layout": "stack",
          "title": "Test",
          "windows": [{
            "cmdline": ["/usr/bin/sh"],
            "title": "Test Window",
            "id": 29 
          }]
        }
        ]
      }]
      `)

			// focus-tab
			mce.SetReturnValueOnce("")
			// send command
			mce.SetReturnValueOnce("")
			// send command
			mce.SetReturnValueOnce("")

			// Create second window and set it up
			mce.SetReturnValueOnce("30")
			// send command
			mce.SetReturnValueOnce("")
			// send command
			mce.SetReturnValueOnce("")

			qnd.SetReadReturnValue(settings.MockQuickNavDaoReadReturn{
				Db: settings.QuickNavDatabase{
					QuickNavs: map[string][]settings.QuickNavHandle{},
				},
				Err: nil,
			})

			expectedQuickNavs := settings.QuickNavDatabase{
				QuickNavs: map[string][]settings.QuickNavHandle{
					"a": {
						{
							EntryId:   2,
							EntryType: kitty.Tab,
						},
					},
					"b": {
						{
							EntryId:   29,
							EntryType: kitty.Window,
						},
					},
					"c": {
						{
							EntryId:   30,
							EntryType: kitty.Window,
						},
					},
				},
			}

			sc.LoadSession("sessionName")

			msdCalls := msd.GetCalls()
			os.Setenv("KITTYMUX_STATE_DIR", "/foo/")
			So(msdCalls.Read[0].filepath, ShouldEndWith, "sessionName.json")

			qndCalls := qnd.GetCalls()
			So(qndCalls.Read.Filepath, ShouldEndWith, "quicknavs.json")

			cmds := mce.GetSavedArgs()
			So(len(cmds), ShouldEqual, 8)
			So(cmds[0], ShouldResemble, []string{"new-window", "--title", "Test Window", "--new-tab", "--tab-title", "\"Test\"", "--cwd", "~/"})
			So(cmds[1], ShouldResemble, []string{"ls"})
			So(cmds[2], ShouldResemble, []string{"focus-tab", "-m", "id:2"})
			So(cmds[3], ShouldResemble, []string{"send-text", "-m", "id:29", "'cd ~/\\n'"})
			So(cmds[4], ShouldResemble, []string{"send-text", "-m", "id:29", "'nvim .\\n'"})

			So(cmds[5], ShouldResemble, []string{"new-window", "--title", "Test Window 2", "-m", "title:Test", "--cwd", "~/bar"})
			So(cmds[6], ShouldResemble, []string{"send-text", "-m", "id:30", "'cd ~/\\n'"})
			So(cmds[7], ShouldResemble, []string{"send-text", "-m", "id:30", "'foo .\\n'"})

			So(qndCalls.Write.Filepath, ShouldEndWith, "quicknavs.json")
			So(qndCalls.Write.QuickNavs, ShouldResemble, expectedQuickNavs)

		})
	})
}
