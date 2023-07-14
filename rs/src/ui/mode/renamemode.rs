use crossterm::event::{KeyCode, KeyEvent};

use crate::error::KittyMuxError;

use crate::ui::command::{
    enter_navigate_command::EnterNavigateCommand, rename_entry_command::RenameEntryCommand,
    text_command::TextCommand, Command,
};

pub struct RenameMode {}

impl RenameMode {
    pub fn handle_input(event: &KeyEvent) -> Result<Vec<Box<dyn Command>>, KittyMuxError> {
        match event.code {
            KeyCode::Esc => Ok(vec![Box::new(EnterNavigateCommand::new())]),
            KeyCode::Enter => Ok(vec![Box::new(RenameEntryCommand::new())]),
            KeyCode::Char(c) => Ok(vec![Box::new(TextCommand::new(Some(c)))]),
            KeyCode::Backspace => Ok(vec![Box::new(TextCommand::new(None))]),
            _ => Ok(vec![]),
        }
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEvent, KeyEventState, KeyModifiers};

    use crate::{
        kitty_model::{entry_type, window_list_entry::WindowListEntry, MockKittyModel},
        quicknav::{persistence::MockQuickNavPersistence, QuickNavDatabase, QuickNavEntry},
        ui::{mode, model::AppModel},
    };

    use super::RenameMode;

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
    fn when_1_selected_and_a_pressed_text_input_appends_a() {
        let mock_quicknav_persistence = MockQuickNavPersistence::new();
        let mock_window_list = MockKittyModel::new();
        let kitty_model = mock_window_list;

        let mut model = AppModel::new(basic_windows(), QuickNavDatabase::new(), mode::Mode::Rename)
            .with_text_input("1".to_string());
        model.state().select(Some(1));

        let event = KeyEvent::new_with_kind_and_state(
            KeyCode::Char('a'),
            KeyModifiers::empty(),
            crossterm::event::KeyEventKind::Press,
            KeyEventState::NONE,
        );
        let cmds = RenameMode::handle_input(&event).expect("Handle input had an error");
        let cmd = cmds.get(0).expect("Did not return a command");
        let result = cmd
            .execute(&kitty_model, &mock_quicknav_persistence, model)
            .expect("execute failed");

        assert_eq!(result.text_input, "1a".to_string());
    }

    fn original_quicknavs() -> QuickNavDatabase {
        QuickNavDatabase::from_entries(vec![QuickNavEntry::new("1".to_string(), 'c', 1)])
    }

    fn updated_quicknavs() -> QuickNavDatabase {
        QuickNavDatabase::from_entries(vec![QuickNavEntry::new("new name".to_string(), 'c', 1)])
    }

    #[test]
    fn when_1_selected_and_new_name_and_enter_pressed_rename_entry_called() {
        let mut mock_window_list = MockKittyModel::new();
        mock_window_list
            .expect_load()
            .times(1)
            .returning(|| Ok(vec![]));
        mock_window_list
            .expect_rename_entry()
            .withf(|_entry: &WindowListEntry, new_name: &str| new_name == "new name")
            .times(1)
            .returning(|_, _| ());

        let kitty_model = mock_window_list;

        let mut quicknav_persistence = MockQuickNavPersistence::default();
        quicknav_persistence
            .expect_load()
            .times(1)
            .returning(|| Ok(original_quicknavs()));

        quicknav_persistence
            .expect_load()
            .times(1)
            .returning(|| Ok(updated_quicknavs()));

        quicknav_persistence
            .expect_save()
            .times(1)
            .returning(|_| Ok(()));

        let mut model = AppModel::new(basic_windows(), QuickNavDatabase::new(), mode::Mode::Rename)
            .with_text_input("new name".to_string());
        model.state().select(Some(0));

        let event = KeyEvent::new_with_kind_and_state(
            KeyCode::Enter,
            KeyModifiers::empty(),
            crossterm::event::KeyEventKind::Press,
            KeyEventState::NONE,
        );
        let cmds = RenameMode::handle_input(&event).expect("Handle input had an error");

        let cmd = cmds.get(0).expect("Did not return a command");
        let result = cmd
            .execute(&kitty_model, &quicknav_persistence, model)
            .expect("execute failed");

        assert_eq!(result.mode(), mode::Mode::Navigate);
    }

    #[test]
    fn when_esc_pressed_enters_navigate_mode() {
        let quicknav_persistence = MockQuickNavPersistence::default();
        let mock_window_list = MockKittyModel::new();
        let kitty_model = mock_window_list;

        let mut model = AppModel::new(basic_windows(), QuickNavDatabase::new(), mode::Mode::Rename)
            .with_text_input("new-name".to_string());
        model.state().select(Some(1));

        let event = KeyEvent::new_with_kind_and_state(
            KeyCode::Esc,
            KeyModifiers::empty(),
            crossterm::event::KeyEventKind::Press,
            KeyEventState::NONE,
        );
        let cmds = RenameMode::handle_input(&event).expect("Handle input had an error");
        let cmd = cmds.get(0).expect("Did not return a command");
        let result = cmd
            .execute(&kitty_model, &quicknav_persistence, model)
            .expect("execute failed");

        assert_eq!(result.mode(), mode::Mode::Navigate);
        assert_eq!(result.should_quit(), false);
    }
}
