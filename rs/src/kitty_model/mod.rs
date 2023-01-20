pub mod entry_type;
pub mod window_list_entry;

use std::error::Error;

use json::JsonValue;
use mockall::automock;

use self::{
    entry_type::EntryType::{Tab, Window},
    window_list_entry::WindowListEntry };
use crate::kitty_connector::KittyConnector;

pub struct BaseKittyModel<'a> {
    connector: KittyConnector<'a>,
}

#[automock]
pub trait KittyModel {
    fn load(&self) -> Result<Vec<WindowListEntry>, Box<dyn Error>>;
    fn focus_entry(&self, entry: &WindowListEntry);
    fn close_entry(&self, entry: &WindowListEntry);
}

impl BaseKittyModel<'_> {
    pub fn new(connector: KittyConnector) -> BaseKittyModel {
        BaseKittyModel { connector }
    }

    pub fn connector(&self) -> &KittyConnector {
        &self.connector
    }
}

impl KittyModel for BaseKittyModel<'_> {
    fn load(&self) -> Result<Vec<WindowListEntry>, Box<dyn Error>> {
        let ls_text = self.connector.ls();
        let ls_response = json::parse(&ls_text)?;
        let num_entries = count_entries(&ls_response);
        let mut entries: Vec<WindowListEntry> = vec![];
        entries.reserve(num_entries);

        for os_window in ls_response.members() {
            flatten_os_window(&mut entries, &os_window);
        }

        Ok(entries)
    }

    fn focus_entry(&self, entry: &WindowListEntry) {
        match &entry.entry_type {
            Window => self.connector.focus_window(entry.id),
            Tab => self.connector.focus_tab(entry.tab_id),
            _ => {}
        }
    }

    fn close_entry(&self, entry: &WindowListEntry) {
        match &entry.entry_type {
            Window => self.connector.close_window(entry.id),
            Tab => self.connector.close_tab(entry.tab_id),
            _ => {}
        }
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
    use super::entry_type::EntryType;
    use super::{BaseKittyModel, WindowListEntry, KittyModel};
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
        let el = BaseKittyModel {
            connector,
        };
        let list = Result::expect(el.load(), "KittyModel::load returned an error");

        assert_eq!(expected.as_slice(), list.as_slice());
    }

    #[test]
    fn given_ls_returns_singe_window_tree_when_load_called_then_correct_vec_returned() {
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
        let el = BaseKittyModel {
            connector,
        };
        let list = Result::expect(el.load(), "KittyModel::load returned an error");

        assert_eq!(expected.as_slice(), list.as_slice());
    }

    #[test]
    fn given_tab_when_close_entry_called_then_command_is_close_tab() {
        let mut mock = MockCommandExecutor::new();
        mock.expect_execute_command()
            .withf(|cmd: &str, _args: &[&str]| cmd == "close-tab")
            .times(1)
            .returning(|_cmd: &str, _args: &[&str]| "".to_string());

        let connector = KittyConnector { executor: &mock };
        let el = BaseKittyModel {
            connector,
        };

        let entry = WindowListEntry {
            id: 1,
            text: "my tab".to_string(),
            title: "my tab".to_string(),
            entry_type: EntryType::Tab,
            pid: 0,
            cwd: "".to_string(),
            is_focused: true,
            tab_is_focused: true,
            os_window_is_focused: true,
            tab_id: 1,
        };

        el.close_entry(&entry);
    }

    fn given_window_when_close_entry_called_then_command_is_close_window() {
        let mut mock = MockCommandExecutor::new();
        mock.expect_execute_command()
            .withf(|cmd: &str, _args: &[&str]| cmd == "close-window")
            .times(1)
            .returning(|_cmd: &str, _args: &[&str]| "".to_string());

        let connector = KittyConnector { executor: &mock };
        let el = BaseKittyModel {
            connector,
        };

        let entry = WindowListEntry {
            id: 1,
            tab_id: 1,
            pid: 1,
            cwd: "/foo".to_string(),
            text: "1".to_string(),
            title: "1".to_string(),
            entry_type: EntryType::Window,
            is_focused: true,
            tab_is_focused: true,
            os_window_is_focused: true,
        };

        el.close_entry(&entry);
    }

}
