use crate::{
    kitty_model::{entry_type::EntryType, window_list_entry::WindowListEntry},
    quicknav::QuickNavDatabase,
};
use tui::widgets::ListState;

use super::mode::Mode;

pub struct AppModel {
    list_state: ListState,
    items: Vec<WindowListEntry>,
    should_quit: bool,
    mode: Mode,
    quicknavs: QuickNavDatabase,
    pub(in crate::ui) text_input: String,
}

impl AppModel {
    pub fn new(
        items: Vec<WindowListEntry>,
        mut quicknavs: QuickNavDatabase,
        mode: Mode,
    ) -> AppModel {
        let selected: Option<usize>;
        if items.len() > 0 {
            selected = Some(0);
        } else {
            selected = None;
        }

        let entries = items
            .iter()
            .map(|e| (e.title.clone(), e.id))
            .collect::<Vec<(String, u32)>>();
        quicknavs.clean_up(entries);

        let mut state = ListState::default();
        state.select(selected);

        AppModel {
            list_state: state,
            items,
            should_quit: false,
            mode,
            text_input: "".to_string(),
            quicknavs,
        }
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn select(&mut self, selected: Option<usize>) {
        if selected < Some(self.items.len()) {
            self.list_state.select(selected);
        }
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.list_state.selected()
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

    pub fn with_selected(mut self, selected_index: Option<usize>) -> AppModel {
        self.select(selected_index);
        self
    }

    #[cfg(test)]
    pub fn with_text_input(mut self, text_input: String) -> AppModel {
        self.text_input = text_input;
        self
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn quicknavs(&self) -> &QuickNavDatabase {
        &self.quicknavs
    }

    pub fn quicknavs_mut(&mut self) -> &mut QuickNavDatabase {
        &mut self.quicknavs
    }
}

#[cfg(test)]
mod tests {
    use crate::{kitty_model::entry_type, quicknav::QuickNavEntry};

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
        let quicknavs = QuickNavDatabase::new();

        let mut list = AppModel::new(items, quicknavs, Mode::Navigate);
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
        let quicknavs = QuickNavDatabase::new();
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
        let mut list = AppModel::new(items, quicknavs, Mode::Navigate);
        list.list_state.select(Some(1));

        list.select_prev();

        assert_eq!(*list.selected().unwrap(), expected);
    }

    #[test]
    fn given_selected_0_when_select_next_selected_1() {
        let items = basic_windows();
        let quicknavs = QuickNavDatabase::new();

        let mut list = AppModel::new(items, quicknavs, Mode::Navigate);
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
        let quicknavs = QuickNavDatabase::new();

        let mut list = AppModel::new(items, quicknavs, Mode::Navigate);
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
        let quicknavs = QuickNavDatabase::new();

        let mut list = AppModel::new(items, quicknavs, Mode::Navigate);
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
        let quicknavs = QuickNavDatabase::new();
        let mut app_model = AppModel::new(windows_and_tabs(), quicknavs, Mode::Navigate);
        app_model.list_state.select(Some(1));
        app_model.select_next_tab();

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

        assert_eq!(*app_model.selected().unwrap(), expected);
    }

    #[test]
    fn given_3_selected_when_select_prev_tab_1_selected() {
        let quicknavs = QuickNavDatabase::new();
        let mut app_model = AppModel::new(windows_and_tabs(), quicknavs, Mode::Navigate);
        app_model.list_state.select(Some(3));
        app_model.select_prev_tab();

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

        assert_eq!(*app_model.selected().unwrap(), expected);
    }

    #[test]
    fn cleans_up_quicknavs() {
        let mut quicknavs = QuickNavDatabase::new();
        quicknavs.add_entry(QuickNavEntry::new("Fake".to_string(), 'z', 7));
        let mut app_model = AppModel::new(windows_and_tabs(), quicknavs, Mode::Navigate);
        app_model.list_state.select(Some(3));
        app_model.select_prev_tab();

        let cleaned_entry = app_model.quicknavs().find_entry_by_id(7);
        assert!(cleaned_entry.is_none());
    }
}
