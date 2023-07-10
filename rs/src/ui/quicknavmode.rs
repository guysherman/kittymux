use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    error::KittyMuxError,
    kitty_model::{entry_type::EntryType, KittyModel}, quicknav::persistence::QuickNavPersistence,
};

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
        _quick_nav_persistence: &dyn QuickNavPersistence,
    ) -> Result<Box<dyn Command>, KittyMuxError> {
        match event.code {
            KeyCode::Char(c) => match c {
                '0'..='9' | 'a'..='z' => {
                    let candidate_ids = model
                        .quicknavs()
                        .find_entries_by_key(c)
                        .iter()
                        .map(|e| e.id)
                        .collect::<Vec<u32>>();

                    let window = model.items().iter().find(|w| {
                        candidate_ids.contains(&w.id)
                            && w.tab_is_focused
                            && w.entry_type == EntryType::Window
                    });

                    if let Some(window) = window {
                        kitty_model.focus_entry(window);
                        Ok(Box::new(QuitCommand::new(model)))
                    } else {
                        Ok(Box::new(NoopCommand::new(model)))
                    }
                }
                _ => Ok(Box::new(NoopCommand::new(model))),
            },
            KeyCode::Esc => Ok(Box::new(EnterNavigateCommand::new(model))),
            _ => Ok(Box::new(NoopCommand::new(model))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEvent, KeyEventState, KeyModifiers};

    use crate::{
        kitty_model::{entry_type::EntryType, window_list_entry::WindowListEntry, MockKittyModel},
        quicknav::{persistence::MockQuickNavPersistence, QuickNavDatabase, QuickNavEntry},
        ui::{mode::Mode::QuickNav, model::AppModel, quicknavmode::QuickNavMode},
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
        db.add_entry(QuickNavEntry::new("1".to_string(), 'a', 1));
        db.add_entry(QuickNavEntry::new("2".to_string(), 'a', 2));
        db
    }

    #[test]
    fn when_a_pressed_then_1_entered() {
        let mut mock_quicknav_persistence = MockQuickNavPersistence::default();
        mock_quicknav_persistence
            .expect_save()
            .times(1)
            .returning(|_| Ok(()));
        let mut kitty_model = MockKittyModel::default();
        kitty_model
            .expect_focus_entry()
            .withf(|e| e.id == 1 && e.entry_type == EntryType::Window)
            .times(1)
            .returning(|_| ());

        let mut model = AppModel::new(basic_windows(), qndb(), QuickNav);
        model.state().select(Some(4));

        let event = KeyEvent::new_with_kind_and_state(
            KeyCode::Char('a'),
            KeyModifiers::empty(),
            crossterm::event::KeyEventKind::Press,
            KeyEventState::NONE,
        );

        let mut cmd = QuickNavMode::handle_input(&event, model, &kitty_model, &mock_quicknav_persistence).unwrap();
        let result = cmd
            .execute(&kitty_model, &mock_quicknav_persistence)
            .unwrap()
            .expect("Command had no AppModel");

        assert_eq!(result.should_quit(), true);
    }
}
