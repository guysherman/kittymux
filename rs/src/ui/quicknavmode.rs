use std::error::Error;

use crossterm::event::{KeyCode, KeyEvent};

use crate::kitty_model::KittyModel;

use super::{
    command::Command, enter_navigate_command::EnterNavigateCommand, model::AppModel,
    noop_command::NoopCommand, quit_command::QuitCommand,
};

pub struct QuickNavMode {}

impl QuickNavMode {
    pub fn handle_input(
        event: &KeyEvent,
        model: AppModel,
        kitty_model: &dyn KittyModel,
    ) -> Result<Box<dyn Command>, Box<dyn Error>> {
        match event.code {
            KeyCode::Char(c) => {
                if let Some(selected) = model.selected() {
                    let candidate_titles = model
                        .quicknavs()
                        .find_entries_by_key(c)
                        .iter()
                        .map(|e| e.title.clone())
                        .collect::<Vec<String>>();

                    let window = model.items().iter().find(|w| {
                        w.tab_id == selected.tab_id && candidate_titles.contains(&w.title)
                    });

                    if let Some(window) = window {
                        kitty_model.focus_entry(window);
                        return Ok(Box::new(QuitCommand::new(model)));
                    }
                }

                return Ok(Box::new(NoopCommand::new(model)));
            }
            KeyCode::Esc => Ok(Box::new(EnterNavigateCommand::new(model))),
            _ => Ok(Box::new(EnterNavigateCommand::new(model))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEvent, KeyEventState, KeyModifiers};

    use crate::{
        kitty_model::{entry_type::EntryType, window_list_entry::WindowListEntry, MockKittyModel},
        quicknav::{QuickNavDatabase, QuickNavEntry},
        ui::{
            mode::Mode::Navigate,
            model::AppModel,
            navigatemode::NavigateMode, quicknavmode::QuickNavMode,
        },
    };

    fn basic_windows() -> Vec<WindowListEntry> {
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

    fn qndb() -> QuickNavDatabase {
        let mut db = QuickNavDatabase::new();
        db.add_entry(QuickNavEntry::new("2".to_string(), 'a'));
        db
    }

    #[test]
    fn when_a_pressed_then_2_entered() {
        let mut mock_window_list = MockKittyModel::new();
        mock_window_list
            .expect_focus_entry()
            .withf(|e| e.id == 2)
            .times(1)
            .returning(|_| ());
        let kitty_model = mock_window_list;

        let mut model = AppModel::new(basic_windows(), qndb(), Navigate);
        model.state().select(Some(4));

        let event = KeyEvent::new_with_kind_and_state(
            KeyCode::Char('a'),
            KeyModifiers::empty(),
            crossterm::event::KeyEventKind::Press,
            KeyEventState::NONE,
        );

        let mut cmd = QuickNavMode::handle_input(&event, model, &kitty_model).unwrap();
        let result = cmd
            .execute(&kitty_model)
            .unwrap()
            .expect("Command had no AppModel");

        assert_eq!(result.should_quit(), true);
    }
}
