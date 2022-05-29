package kitty

import "testing"

type MockCommandExecutor struct {
	returnValue string
	savedArgs   []string
}

func (c *MockCommandExecutor) SetReturnValue(returnValue string) {
	c.returnValue = returnValue
}

func (c *MockCommandExecutor) GetSavedArgs() []string {
	return c.savedArgs
}

func (c *MockCommandExecutor) ExecuteCommand(args []string) string {
	c.savedArgs = args
	return c.returnValue
}

func TestFlattenOsWindow(t *testing.T) {
	ce := &MockCommandExecutor{}
	wl := &KittyConnector{}

	ce.SetReturnValue(`
      [
        {
          "id": 2,
          "is_focused": true,
          "platform_window_id": 77594671,
          "tabs": [],
          "wm_class": "kitty",
          "wm_name": "kitty"
        },
        {
          "id": 3,
          "is_focused": false,
          "platform_window_id": 77594672,
          "tabs": [],
          "wm_class": "kitty",
          "wm_name": "kitty"
        }
      ]
  `)

	el := &EntryListerBase{}

	entries := el.EntryList(wl, ce)

	if len(entries) != 2 {
		t.Error("Incorrect number of entries", len(entries))
	}

	if entries[0].Id != 2 {
		t.Error("Incorrect entry Id", entries[0].Id)
	}

	if entries[0].EntryType != OsWindow {
		t.Error("Incorrect entry type", entries[0].EntryType)
	}

	if entries[0].OsWindowIsFocused != true {
		t.Error("Incorrect focus detection", entries[0].OsWindowIsFocused)
	}

	if entries[1].OsWindowIsFocused != false {
		t.Error("Incorrect focus detection", entries[1].OsWindowIsFocused)
	}

	if entries[0].Text != "kitty:2" {
		t.Error("Incorrect entry text", entries[0].Text)
	}
}

func TestFlattenTabs(t *testing.T) {
	ce := &MockCommandExecutor{}
	wl := &KittyConnector{}

	ce.SetReturnValue(`
    [
      {
        "id": 2,
        "is_focused": true,
        "platform_window_id": 77594671,
        "tabs": [
          {
            "id": 123,
            "is_focused": true,
            "layout": "stack",
            "title": "kitty @ ls",
            "windows": [],
            "active_window_history": [1, 2, 3, 4]
          },
          {
            "id": 456,
            "is_focused": false,
            "layout": "stack",
            "title": "zsh",
            "windows": [],
            "active_window_history": [1, 2, 3, 4]
          }
        ],
        "wm_class": "kitty",
        "wm_name": "kitty"
      }
    ]
  `)

	el := &EntryListerBase{}

	entries := el.EntryList(wl, ce)

	if len(entries) != 3 {
		t.Error("Incorrect number of entries", len(entries))
	}

	if entries[1].EntryType != Tab {
		t.Error("Incorrect entry type", entries[1].EntryType)
	}

	if entries[1].Id != 123 {
		t.Error("Incorrect id", entries[1].Id)
	}

	if entries[1].Title != "kitty @ ls" {
		t.Error("Incorrect title", entries[1].Title)
	}

	if entries[1].IsFocused != true {
		t.Error("Incorrect Focus", entries[1].IsFocused)
	}

	if entries[2].IsFocused != false {
		t.Error("Incorrect Focus", entries[2].IsFocused)
	}

	if entries[2].OsWindowIsFocused != true {
		t.Error("Incorrect OsWindow Focus", entries[2].OsWindowIsFocused)
	}

	if entries[1].Text != " ├─ kitty @ ls (tab:123) *" {
		t.Error("Incorrect Text", entries[1].Text)
	}

	if entries[2].Text != " └─ zsh (tab:456) " {
		t.Error("Incorrect Last Tab Text", entries[2].Text)
	}

}

func TestFlattenWindows(t *testing.T) {
	ce := &MockCommandExecutor{}
	wl := &KittyConnector{}

	ce.SetReturnValue(`
    [
      {
        "id": 2,
        "is_focused": true,
        "platform_window_id": 77594671,
        "tabs": [
          {
            "id": 123,
            "is_focused": true,
            "layout": "stack",
            "title": "kitty @ ls",
            "windows": [
            {
              "cmdline": [
                "/usr/bin/zsh"
              ],
              "columns": 236,
              "cwd": "/home/guy/source/kittymux/go",
              "env": {
                "KITTY_WINDOW_ID": "10",
                "PWD": "/home/guy"
              },
              "foreground_processes": [
                {
                  "cmdline": [
                    "/usr/bin/zsh"
                  ],
                  "cwd": "/home/guy/source/kittymux/go",
                  "pid": 299098
                }
              ],
              "id": 10,
              "is_focused": true,
              "is_self": false,
              "lines": 48,
              "pid": 299098,
              "title": "vim-test-output"
            },
            {
              "cmdline": [
                "/usr/bin/zsh"
              ],
              "columns": 236,
              "cwd": "/home/guy/source/tview",
              "env": {
                "KITTY_WINDOW_ID": "21",
                "PWD": "/home/guy/source/kittymux/ts"
              },
              "foreground_processes": [
                {
                  "cmdline": [
                    "/usr/bin/zsh"
                  ],
                  "cwd": "/home/guy/source/tview",
                  "pid": 329681
                }
              ],
              "id": 21,
              "is_focused": false,
              "is_self": false,
              "lines": 48,
              "pid": 329681,
              "title": "~/source/tview"
            }
            ],
            "active_window_history": [1, 2, 3, 4]
          }
        ],
        "wm_class": "kitty",
        "wm_name": "kitty"
      }
    ]
  `)

	el := &EntryListerBase{}

	entries := el.EntryList(wl, ce)

	if len(entries) != 4 {
		t.Error("Incorrect number of entries", len(entries))
	}

	if entries[2].Cwd != "/home/guy/source/kittymux/go" {
		t.Error("Incorrect Cwd", entries[2].Cwd)
	}

	if entries[2].Pid != 299098 {
		t.Error("Incorrect Pid", entries[2].Pid)
	}

	if entries[2].Title != "vim-test-output" {
		t.Error("Incorrect Title", entries[2].Title)
	}

	if entries[2].EntryType != Window {
		t.Error("Incorrect entry type", entries[2].EntryType)
	}

	if entries[2].Id != 10 {
		t.Error("Incorrect Id", entries[2].Id)
	}

	if entries[2].IsFocused != true {
		t.Error("First window Focus incorrect", entries[2].IsFocused)
	}

	if entries[3].IsFocused != false {
		t.Error("Second window Focus incorrect", entries[3].IsFocused)
	}

	if entries[2].TabIsFocused != true {
		t.Error("First window Tab Focus incorrect", entries[2].TabIsFocused)
	}

	if entries[2].OsWindowIsFocused != true {
		t.Error("First window Os Window Focus incorrect", entries[2].OsWindowIsFocused)
	}

	if entries[2].Tab.Id != 123 {
		t.Error("Tab is incorrect", entries[2].Tab)
	}

	if entries[2].Text != "    ├─ vim-test-output (id:10; pid:299098) *" {
		t.Error("Incorrect Text", entries[2].Text)
	}

	if entries[3].Text != "    └─ ~/source/tview (id:21; pid:329681) " {
		t.Error("Incorrect Window Text", entries[3].Text)
	}
}
