package kitty

import "testing"

func TestCreatesOsWindowCorrectly(t *testing.T) {
	ce := &MockCommandExecutorKittyLs{}
	kc := KittyConnector{}

	windows := kc.WindowList(ce)
	if len(windows) < 1 {
		t.Error("Not enough os windows returned")
	}

	window := windows[0]

	if window.Id != 2 {
		t.Error("Incorrect window id", window.Id)
	}

	if window.Platform_window_id != 77594671 {
		t.Error("Incorrect platform_window_id")
	}

	if len(window.Tabs) != 1 {
		t.Error("Did not process tabs")
	}
}

// I was going to add more tests, but Unmarshal basically workslike json.parse in JS,
// so it's fine, I'm not going to test it any further, the only issue I ran into was
// struct member naming, and now I know, I don't need tests for that

type MockCommandExecutorKittyLs struct{}

func (c *MockCommandExecutorKittyLs) ExecuteCommand(args []string) string {
	return `
  [ {
    "id": 2,
    "is_focused": true,
    "platform_window_id": 77594671,
    "tabs": [
    {
      "active_window_history": [
      14
      ],
      "enabled_layouts": [
      "stack",
      "tall:bias=75;full_size=1;mirrored=false"
      ],
      "id": 3,
      "is_focused": true,
      "layout": "stack",
      "layout_opts": {},
      "layout_state": {},
      "title": "kitty @ ls",
      "windows": [
      {
        "cmdline": [
        "/usr/bin/zsh"
        ],
        "columns": 116,
        "cwd": "/home/guy",
        "env": {
          "KITTY_WINDOW_ID": "14",
          "PWD": "/home/guy",
          "WINDOWID": "77594671"
        },
        "foreground_processes": [
        {
          "cmdline": [
          "kitty",
          "@",
          "ls"
          ],
          "cwd": "/home/guy",
          "pid": 163835
        }
        ],
        "id": 14,
        "is_focused": true,
        "is_self": false,
        "lines": 48,
        "pid": 163131,
        "title": "kitty @ ls"
      }
      ]
    }
    ],
    "wm_class": "kitty",
    "wm_name": "kitty"
  } ]
  `
}
