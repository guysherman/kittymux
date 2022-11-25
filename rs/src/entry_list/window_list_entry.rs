use json::JsonValue;

use super::entry_type::EntryType;

const TAB_INDENT: &str = " ├─ ";
const LAST_TAB_INDENT: &str = " └─ ";
const WINDOW_INDENT: &str = " │  ├─ ";
const LAST_WINDOW_INDENT: &str = " │  └─ ";
const LAST_TAB_WINDOW_INDENT: &str = "    ├─ ";
const LAST_TAB_LAST_WINDOW_INDENT: &str = "    └─ ";

#[derive(Debug, Eq, PartialEq)]
pub struct WindowListEntry {
    id: u32,
    text: String,
    entry_type: EntryType,
    pid: u32,
    cwd: String,
    title: String,
    is_focused: bool,
    tab_is_focused: bool,
    os_window_is_focused: bool,
    tab_id: u32,
}

impl WindowListEntry {
    pub fn new_from_os_window(os_window_json: &JsonValue) -> WindowListEntry {
        let id = os_window_json["id"].as_u32().unwrap_or_default();
        let is_focused = os_window_json["is_focused"].as_bool().unwrap_or_default();
        let title = format!("kitty: {}", id);
        WindowListEntry {
            id,
            text: title.clone(),
            entry_type: EntryType::OsWindow,
            pid: 0,
            cwd: "".to_string(),
            title,
            is_focused,
            tab_is_focused: is_focused,
            os_window_is_focused: is_focused,
            tab_id: 0,
        }
    }

    pub fn new_from_tab(
        tab: &JsonValue,
        is_last: bool,
        os_window_is_focused: bool,
    ) -> WindowListEntry {
        let id = tab["id"].as_u32().unwrap_or_default();
        let title = tab["title"].as_str().unwrap_or_default().to_string();
        let is_focused = tab["is_focused"].as_bool().unwrap_or_default();
        let indent: &str;
        if is_last {
            indent = LAST_TAB_INDENT;
        } else {
            indent = TAB_INDENT;
        }

        let star;
        if is_focused {
            star = "*";
        } else {
            star = "";
        }

        let text = format!("{}{} (tab:{}) {}", indent, &title, id, star);

        WindowListEntry {
            id,
            text,
            title,
            entry_type: EntryType::Tab,
            pid: 0,
            cwd: "".to_string(),
            is_focused,
            tab_is_focused: is_focused,
            os_window_is_focused,
            tab_id: id,
        }
    }

    pub fn new_from_window(
        window: &JsonValue,
        is_last: bool,
        parent_is_last: bool,
        os_window_is_focused: bool,
        tab_is_focused: bool,
        tab_id: u32,
    ) -> WindowListEntry {
        let id = window["id"].as_u32().unwrap_or_default();
        let pid = window["pid"].as_u32().unwrap_or_default();
        let title = window["title"].as_str().unwrap_or_default().to_string();
        let is_focused = window["is_focused"].as_bool().unwrap_or_default();
        let cwd = window["cwd"].as_str().unwrap_or_default().to_string();

        let indent;
        if parent_is_last && is_last {
            indent = LAST_TAB_LAST_WINDOW_INDENT;
        } else if parent_is_last {
            indent = LAST_TAB_WINDOW_INDENT;
        } else if is_last {
            indent = LAST_WINDOW_INDENT;
        } else {
            indent = WINDOW_INDENT;
        }

        let star;
        if is_focused {
            star = "*";
        } else {
            star = "";
        }

        let text = format!("{}{} (id:{}; pid:{}) {}", indent, &title, id, pid, star);

        WindowListEntry {
            id,
            text,
            entry_type: EntryType::Window,
            pid,
            cwd,
            title,
            is_focused,
            tab_is_focused,
            os_window_is_focused,
            tab_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::entry_list::entry_type::EntryType;

    use super::WindowListEntry;

    #[test]
    fn given_json_value_when_new_from_os_window_called_then_windowlistentry_returned() {
        let os_window_json = r###"
            {
                "id": 2,
                "is_focused": true,
                "platform_window_id": 77594671
            }
            "###;

        let os_window = json::parse(os_window_json).unwrap();
        let os_window_entry = WindowListEntry::new_from_os_window(&os_window);
        let expected = WindowListEntry {
            id: 2,
            text: "kitty: 2".to_string(),
            entry_type: EntryType::OsWindow,
            pid: 0,
            cwd: "".to_string(),
            title: "kitty: 2".to_string(),
            is_focused: true,
            tab_is_focused: true,
            os_window_is_focused: true,
            tab_id: 0,
        };

        assert_eq!(expected, os_window_entry);
    }

    #[test]
    fn given_json_value_when_new_from_tab_called_then_windowlistentry_returned() {
        let tab_json = r###"
        {
           "id": 2,
           "title": "code",
           "is_focused": true
        }
        "###;

        let tab = json::parse(tab_json).unwrap();
        let tab_entry = WindowListEntry::new_from_tab(&tab, true, true);
        let expected = WindowListEntry {
            id: 2,
            text: " └─ code (tab:2) *".to_string(),
            entry_type: EntryType::Tab,
            pid: 0,
            cwd: "".to_string(),
            title: "code".to_string(),
            is_focused: true,
            tab_is_focused: true,
            os_window_is_focused: true,
            tab_id: 2,
        };

        assert_eq!(expected, tab_entry);
    }

    #[test]
    fn given_penultimate_tab_when_new_from_tab_called_then_windowlistentry_returned() {
        let tab_json = r###"
        {
           "id": 2,
           "title": "code",
           "is_focused": true
        }
        "###;

        let tab = json::parse(tab_json).unwrap();
        let tab_entry = WindowListEntry::new_from_tab(&tab, false, true);
        let expected = WindowListEntry {
            id: 2,
            text: " ├─ code (tab:2) *".to_string(),
            entry_type: EntryType::Tab,
            pid: 0,
            cwd: "".to_string(),
            title: "code".to_string(),
            is_focused: true,
            tab_is_focused: true,
            os_window_is_focused: true,
            tab_id: 2,
        };

        assert_eq!(expected, tab_entry);
    }

    #[test]
    fn given_unfocused_tab_when_new_from_tab_called_then_windowlistentry_returned() {
        let tab_json = r###"
        {
           "id": 2,
           "title": "code",
           "is_focused": false
        }
        "###;

        let tab = json::parse(tab_json).unwrap();
        let tab_entry = WindowListEntry::new_from_tab(&tab, false, false);
        let expected = WindowListEntry {
            id: 2,
            text: " ├─ code (tab:2) ".to_string(),
            entry_type: EntryType::Tab,
            pid: 0,
            cwd: "".to_string(),
            title: "code".to_string(),
            is_focused: false,
            tab_is_focused: false,
            os_window_is_focused: false,
            tab_id: 2,
        };

        assert_eq!(expected, tab_entry);
    }

    #[test]
    fn given_last_window_last_tab_when_new_from_window_called_then_windowlistentry_returned() {
        let window_json = r###"
        {
            "id": 5,
            "pid": 12345,
            "cwd": "/foo",
            "is_focused": true,
            "title": "nvim"
        }
        "###;

        let window = json::parse(window_json).unwrap();
        let window_entry = WindowListEntry::new_from_window(&window, true, true, true, true, 2);
        let expected = WindowListEntry {
            id: 5,
            text: "    └─ nvim (id:5; pid:12345) *".to_string(),
            entry_type: EntryType::Window,
            pid: 12345,
            cwd: "/foo".to_string(),
            title: "nvim".to_string(),
            is_focused: true,
            tab_is_focused: true,
            os_window_is_focused: true,
            tab_id: 2,
        };

        assert_eq!(expected, window_entry);
    }

    #[test]
    fn given_notlast_window_last_tab_when_new_from_window_called_then_windowlistentry_returned() {
        let window_json = r###"
        {
            "id": 5,
            "pid": 12345,
            "cwd": "/foo",
            "is_focused": true,
            "title": "nvim"
        }
        "###;

        let window = json::parse(window_json).unwrap();
        let window_entry = WindowListEntry::new_from_window(&window, false, true, true, true, 2);
        let expected = WindowListEntry {
            id: 5,
            text: "    ├─ nvim (id:5; pid:12345) *".to_string(),
            entry_type: EntryType::Window,
            pid: 12345,
            cwd: "/foo".to_string(),
            title: "nvim".to_string(),
            is_focused: true,
            tab_is_focused: true,
            os_window_is_focused: true,
            tab_id: 2,
        };

        assert_eq!(expected, window_entry);
    }

    #[test]
    fn given_notlast_window_notlast_tab_when_new_from_window_called_then_windowlistentry_returned()
    {
        let window_json = r###"
        {
            "id": 5,
            "pid": 12345,
            "cwd": "/foo",
            "is_focused": true,
            "title": "nvim"
        }
        "###;

        let window = json::parse(window_json).unwrap();
        let window_entry = WindowListEntry::new_from_window(&window, false, false, true, true, 2);
        let expected = WindowListEntry {
            id: 5,
            text: " │  ├─ nvim (id:5; pid:12345) *".to_string(),
            entry_type: EntryType::Window,
            pid: 12345,
            cwd: "/foo".to_string(),
            title: "nvim".to_string(),
            is_focused: true,
            tab_is_focused: true,
            os_window_is_focused: true,
            tab_id: 2,
        };

        assert_eq!(expected, window_entry);
    }

    #[test]
    fn given_notfocused_notlast_window_notlast_tab_when_new_from_window_called_then_windowlistentry_returned(
    ) {
        let window_json = r###"
        {
            "id": 5,
            "pid": 12345,
            "cwd": "/foo",
            "is_focused": false,
            "title": "nvim"
        }
        "###;

        let window = json::parse(window_json).unwrap();
        let window_entry = WindowListEntry::new_from_window(&window, false, false, false, false, 2);
        let expected = WindowListEntry {
            id: 5,
            text: " │  ├─ nvim (id:5; pid:12345) ".to_string(),
            entry_type: EntryType::Window,
            pid: 12345,
            cwd: "/foo".to_string(),
            title: "nvim".to_string(),
            is_focused: false,
            tab_is_focused: false,
            os_window_is_focused: false,
            tab_id: 2,
        };

        assert_eq!(expected, window_entry);
    }
}
