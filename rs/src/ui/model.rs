use crate::kitty_model::{entry_type::EntryType, window_list_entry::WindowListEntry};
use tui::widgets::ListState;

pub struct AppModel {
    list_state: ListState,
    items: Vec<WindowListEntry>,
}

impl AppModel {
    pub fn with_items(items: Vec<WindowListEntry>) -> AppModel {
        let selected: Option<usize>;
        if items.len() > 0 {
            selected = Some(0);
        } else {
            selected = None;
        }

        let mut state = ListState::default();
        state.select(selected);

        AppModel { list_state: state, items }
    }

    pub fn select_next(&mut self) {
        let len = self.items.len();
        let i = match self.list_state.selected() {
            Some(i) => (i + 1).min(len - 1),
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn select_prev(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => (i as i128 - 1).max(0) as usize,
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn select_next_tab(&mut self) {
        let to_skip = match self.list_state.selected() {
            Some(i) => i + 1,
            None => 0,
        };
        self.items
            .iter()
            .enumerate()
            .skip(to_skip)
            .find(|(_i, entry)| entry.entry_type == EntryType::Tab)
            .map(|(i, _entry)| {
                self.list_state.select(Some(i));
            });
    }

    pub fn select_prev_tab(&mut self) {
        let to_take = self.list_state.selected().unwrap_or_default();

        self.items
            .iter()
            .enumerate()
            .take(to_take)
            .rev()
            .find(|(_i, entry)| entry.entry_type == EntryType::Tab)
            .map(|(i, _entry)| self.list_state.select(Some(i)));
    }

    pub fn items<'a>(&'a self) -> &'a Vec<WindowListEntry> {
        &self.items
    }

    pub fn state<'a>(&'a mut self) -> &'a mut ListState {
        &mut self.list_state
    }

    pub fn selected(&self) -> Option<&WindowListEntry> {
        match self.list_state.selected() {
            Some(i) => self.items.get(i),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::kitty_model::entry_type;

    use super::*;

    fn basic_windows() -> Vec<WindowListEntry> {
        vec![
            WindowListEntry {
                id: 1,
                tab_id: 1,
                pid: 1,
                cwd: "/foo".to_string(),
                text: "1".to_string(),
                title: "1".to_string(),
                entry_type: entry_type::EntryType::Window,
                is_focused: true,
                tab_is_focused: true,
                os_window_is_focused: true,
            },
            WindowListEntry {
                id: 2,
                tab_id: 2,
                pid: 2,
                cwd: "/foo".to_string(),
                text: "2".to_string(),
                title: "2".to_string(),
                entry_type: entry_type::EntryType::Window,
                is_focused: true,
                tab_is_focused: true,
                os_window_is_focused: true,
            },
            WindowListEntry {
                id: 3,
                tab_id: 3,
                pid: 3,
                cwd: "/foo".to_string(),
                text: "3".to_string(),
                title: "3".to_string(),
                entry_type: entry_type::EntryType::Window,
                is_focused: true,
                tab_is_focused: true,
                os_window_is_focused: true,
            },
        ]
    }

    #[test]
    fn given_selected_0_when_select_prev_selected_0() {
        let items = basic_windows();

        let mut list = AppModel::with_items(items);
        let expected = WindowListEntry {
            id: 1,
            tab_id: 1,
            pid: 1,
            cwd: "/foo".to_string(),
            text: "1".to_string(),
            title: "1".to_string(),
            entry_type: entry_type::EntryType::Window,
            is_focused: true,
            tab_is_focused: true,
            os_window_is_focused: true,
        };

        list.select_prev();

        assert_eq!(*list.selected().unwrap(), expected);
    }

    #[test]
    fn given_selected_1_when_select_prev_selected_0() {
        let items = basic_windows();
        let expected = WindowListEntry {
            id: 1,
            tab_id: 1,
            pid: 1,
            cwd: "/foo".to_string(),
            text: "1".to_string(),
            title: "1".to_string(),
            entry_type: entry_type::EntryType::Window,
            is_focused: true,
            tab_is_focused: true,
            os_window_is_focused: true,
        };
        let mut list = AppModel::with_items(items);
        list.list_state.select(Some(1));

        list.select_prev();

        assert_eq!(*list.selected().unwrap(), expected);
    }

    #[test]
    fn given_selected_0_when_select_next_selected_1() {
        let items = basic_windows();

        let mut list = AppModel::with_items(items);
        let expected = WindowListEntry {
            id: 2,
            tab_id: 2,
            pid: 2,
            cwd: "/foo".to_string(),
            text: "2".to_string(),
            title: "2".to_string(),
            entry_type: entry_type::EntryType::Window,
            is_focused: true,
            tab_is_focused: true,
            os_window_is_focused: true,
        };

        list.select_next();

        assert_eq!(*list.selected().unwrap(), expected);
    }

    #[test]
    fn given_selected_2_when_select_next_selected_2() {
        let items = basic_windows();

        let mut list = AppModel::with_items(items);
        list.list_state.select(Some(2));
        let expected = WindowListEntry {
            id: 3,
            tab_id: 3,
            pid: 3,
            cwd: "/foo".to_string(),
            text: "3".to_string(),
            title: "3".to_string(),
            entry_type: entry_type::EntryType::Window,
            is_focused: true,
            tab_is_focused: true,
            os_window_is_focused: true,
        };

        list.select_next();

        assert_eq!(*list.selected().unwrap(), expected);
    }

    #[test]
    fn given_selected_2_selected_returns_correct_item() {
        let items = basic_windows();

        let mut list = AppModel::with_items(items);
        list.list_state.select(Some(2));
        let expected = WindowListEntry {
            id: 3,
            tab_id: 3,
            pid: 3,
            cwd: "/foo".to_string(),
            text: "3".to_string(),
            title: "3".to_string(),
            entry_type: entry_type::EntryType::Window,
            is_focused: true,
            tab_is_focused: true,
            os_window_is_focused: true,
        };

        assert_eq!(*list.selected().unwrap(), expected);
    }

    fn windows_and_tabs() -> Vec<WindowListEntry> {
        vec![
            WindowListEntry {
                id: 1,
                text: "kitty: 1".to_string(),
                entry_type: EntryType::OsWindow,
                pid: 0,
                cwd: "".to_string(),
                title: "kitty: 1".to_string(),
                is_focused: true,
                tab_is_focused: true,
                os_window_is_focused: true,
                tab_id: 0,
            },
            WindowListEntry {
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
            },
            WindowListEntry {
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
            },
            WindowListEntry {
                id: 2,
                text: "my tab 2".to_string(),
                title: "my tab 2".to_string(),
                entry_type: EntryType::Tab,
                pid: 0,
                cwd: "".to_string(),
                is_focused: false,
                tab_is_focused: false,
                os_window_is_focused: true,
                tab_id: 2,
            },
            WindowListEntry {
                id: 2,
                tab_id: 2,
                pid: 2,
                cwd: "/foo".to_string(),
                text: "2".to_string(),
                title: "2".to_string(),
                entry_type: EntryType::Window,
                is_focused: false,
                tab_is_focused: false,
                os_window_is_focused: true,
            },
            WindowListEntry {
                id: 3,
                text: "my tab 3".to_string(),
                title: "my tab 3".to_string(),
                entry_type: EntryType::Tab,
                pid: 0,
                cwd: "".to_string(),
                is_focused: false,
                tab_is_focused: false,
                os_window_is_focused: true,
                tab_id: 3,
            },
            WindowListEntry {
                id: 3,
                tab_id: 3,
                pid: 3,
                cwd: "/foo".to_string(),
                text: "3".to_string(),
                title: "3".to_string(),
                entry_type: EntryType::Window,
                is_focused: false,
                tab_is_focused: false,
                os_window_is_focused: true,
            },
        ]
    }

    #[test]
    fn given_1_selected_when_select_next_tab_3_selected() {
        let mut entry_list = AppModel::with_items(windows_and_tabs());
        entry_list.list_state.select(Some(1));
        entry_list.select_next_tab();

        let expected = WindowListEntry {
            id: 2,
            text: "my tab 2".to_string(),
            title: "my tab 2".to_string(),
            entry_type: EntryType::Tab,
            pid: 0,
            cwd: "".to_string(),
            is_focused: false,
            tab_is_focused: false,
            os_window_is_focused: true,
            tab_id: 2,
        };

        assert_eq!(*entry_list.selected().unwrap(), expected);
    }

    #[test]
    fn given_3_selected_when_select_prev_tab_1_selected() {
        let mut entry_list = AppModel::with_items(windows_and_tabs());
        entry_list.list_state.select(Some(3));
        entry_list.select_prev_tab();

        let expected = WindowListEntry {
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

        assert_eq!(*entry_list.selected().unwrap(), expected);
    }
}
