mod entry_type;
mod window_list_entry;

use std::num;

use json::JsonValue;
use mockall::automock;

use self::window_list_entry::WindowListEntry;
use crate::kitty_connector::KittyConnector;

#[automock]
pub trait EntryList {
    fn load(&self) -> Vec<WindowListEntry>;
}

pub struct KittyEntryList<'a> {
    connector: &'a KittyConnector<'a>,
}

impl EntryList for KittyEntryList<'_> {
    fn load(&self) -> Vec<WindowListEntry> {
        let ls_text = self.connector.ls();
        let ls_response = json::parse(&ls_text).unwrap();
        let num_entries = count_entries(&ls_response);
        let mut entries: Vec<WindowListEntry> = vec![];
        entries.reserve(num_entries);

        for os_window in ls_response.members() {
            flatten_os_window(&mut entries, &os_window);
        }

        entries
    }
}

fn count_entries(ls_response: &JsonValue) -> usize {
    let mut count: usize = 0;
    count += ls_response.len();

    for os_window in ls_response.members() {
        let tabs = &os_window["tabs"];
        count += tabs.len();
        for tab in tabs.members() {
            let windows = &tab["windows"];
            count += windows.len();
        }
    }

    count
}

fn flatten_os_window(entries: &mut Vec<WindowListEntry>, os_window: &JsonValue) {
    entries.push(WindowListEntry::new_from_os_window(&os_window));
    let num_tabs = os_window["tabs"].len();
    let os_window_focused = os_window["is_focused"].as_bool().unwrap_or_default();
    for (i, tab) in os_window["tabs"].members().enumerate() {
        let tab_is_last = i == num_tabs - 1;
        flatten_tab(entries, tab, tab_is_last, os_window_focused)
    }
}

fn flatten_tab(
    entries: &mut Vec<WindowListEntry>,
    tab: &JsonValue,
    tab_is_last: bool,
    os_window_focused: bool,
) {
    let tab_is_focused = tab["is_focused"].as_bool().unwrap_or_default();
    let tab_id = tab["id"].as_u32().unwrap_or_default();
    let num_windows = tab["windows"].len();

    entries.push(WindowListEntry::new_from_tab(
        &tab,
        tab_is_last,
        os_window_focused,
    ));

    for (j, window) in tab["windows"].members().enumerate() {
        let window_is_last = j == num_windows - 1;
        entries.push(WindowListEntry::new_from_window(
            &window,
            window_is_last,
            tab_is_last,
            os_window_focused,
            tab_is_focused,
            tab_id,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::{EntryList, KittyEntryList, WindowListEntry};
    use crate::kitty_connector::command_executor::MockCommandExecutor;
    use crate::kitty_connector::KittyConnector;

    #[test]
    fn given_ls_returns_empty_array_when_load_called_then_empty_vec_returned() {
        let mut mock = MockCommandExecutor::new();
        mock.expect_execute_command()
            .withf(|cmd: &str, _args: &[&str]| cmd == "ls")
            .times(1)
            .returning(|_cmd: &str, _args: &[&str]| "[]".to_string());

        let connector = KittyConnector { executor: &mock };
        let expected: Vec<WindowListEntry> = vec![];
        let el = KittyEntryList {
            connector: &connector,
        };
        let list = el.load();

        assert_eq!(expected.as_slice(), list.as_slice());
    }

    #[test]
    fn given_ls_returns_single_window_tree_when_load_called_then_correct_vec_returned() {
        let ls_return = r###"
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
            "###;
        let ls_json = json::parse(ls_return).unwrap();
        let os_window_json = &ls_json[0];
        let tab_json = &os_window_json["tabs"][0];
        let window_json = &tab_json["windows"][0];

        let mut mock = MockCommandExecutor::new();
        mock.expect_execute_command()
            .withf(|cmd: &str, _args: &[&str]| cmd == "ls")
            .times(1)
            .returning(|_cmd: &str, _args: &[&str]| ls_return.to_string());

        let connector = KittyConnector { executor: &mock };
        let expected: Vec<WindowListEntry> = vec![
            WindowListEntry::new_from_os_window(os_window_json),
            WindowListEntry::new_from_tab(tab_json, true, true),
            WindowListEntry::new_from_window(window_json, true, true, true, true, 3),
        ];
        let el = KittyEntryList {
            connector: &connector,
        };
        let list = el.load();

        assert_eq!(expected.as_slice(), list.as_slice());
    }
}
