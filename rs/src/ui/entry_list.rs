use crate::entry_list::window_list_entry::WindowListEntry;
use tui::widgets::ListState;

pub struct EntryList {
    state: ListState,
    items: Vec<WindowListEntry>,
}

impl EntryList {
    pub fn with_items(items: Vec<WindowListEntry>) -> EntryList {
        let selected: Option<usize>;
        if items.len() > 0 {
            selected = Some(0);
        } else {
            selected = None;
        }

        let mut state = ListState::default();
        state.select(selected);

        EntryList { state, items }
    }

    pub fn select_next(&mut self) {
        let len = self.items.len();
        let i = match self.state.selected() {
            Some(i) => (i + 1).min(len - 1),
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn select_prev(&mut self) {
        let i = match self.state.selected() {
            Some(i) => (i as i128 - 1).max(0) as usize,
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn items<'a>(&'a self) -> &'a Vec<WindowListEntry> {
        &self.items
    }

    pub fn state<'a>(&'a mut self) -> &'a mut ListState {
        &mut self.state
    }

    pub fn selected(&self) -> Option<&WindowListEntry> {
        match self.state.selected() {
            Some(i) => self.items.get(i),
            None => None
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::entry_list::{entry_type};

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
            }
        ]
    }


    #[test]
    fn given_selected_0_when_select_prev_selected_0() {
        let items = basic_windows(); 

        let mut list = EntryList::with_items(items);
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
        let mut list = EntryList::with_items(items);
        list.state.select(Some(1));

        list.select_prev();

        assert_eq!(*list.selected().unwrap(), expected);
    }

    #[test]
    fn given_selected_0_when_select_next_selected_1() {
        let items = basic_windows(); 

        let mut list = EntryList::with_items(items);
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

        let mut list = EntryList::with_items(items);
        list.state.select(Some(2));
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

        let mut list = EntryList::with_items(items);
        list.state.select(Some(2));
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
}
