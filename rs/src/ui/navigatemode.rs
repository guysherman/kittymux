use crossterm::event::{KeyCode, KeyEvent};

use crate::error::KittyMuxError;

use super::{
    close_entry_command::CloseEntryCommand, command::Command,
    enter_quicknav_command::EnterQuickNavCommand, enter_rename_command::EnterRenameCommand,
    enter_setquicknav_command::EnterSetQuickNavCommand, load_command::LoadCommand,
    quit_command::QuitCommand, select_entry_command::SelectEntryCommand,
    select_next_command::SelectNextCommand, select_next_tab_command::SelectNextTabCommand,
    select_prev_command::SelectPrevCommand, select_prev_tab_command::SelectPrevTabCommand,
};

pub struct NavigateMode {}

impl NavigateMode {
    pub fn handle_input(
        event: &KeyEvent,
    ) -> Result<Vec<Box<dyn Command>>, KittyMuxError> {
        match event.code {
            KeyCode::Char('q') => Ok(vec![Box::new(QuitCommand::new())]),
            KeyCode::Esc => Ok(vec![Box::new(QuitCommand::new())]),
            KeyCode::Char('j') => Ok(vec![Box::new(SelectNextCommand::new())]),
            KeyCode::Char('k') => Ok(vec![Box::new(SelectPrevCommand::new())]),
            KeyCode::Char('J') => Ok(vec![Box::new(SelectNextTabCommand::new())]),
            KeyCode::Char('K') => Ok(vec![Box::new(SelectPrevTabCommand::new())]),
            KeyCode::Char('x') => Ok(vec![
                Box::new(CloseEntryCommand::new()),
                Box::new(LoadCommand::new()),
            ]),
            KeyCode::Char('a') => Ok(vec![Box::new(EnterRenameCommand::new())]),
            KeyCode::Enter => Ok(vec![Box::new(SelectEntryCommand::new())]),
            KeyCode::Char('m') => Ok(vec![Box::new(EnterSetQuickNavCommand::new())]),
            KeyCode::Char('\'') => Ok(vec![Box::new(EnterQuickNavCommand::new())]),
            _ => Ok(vec![]),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        kitty_model::window_list_entry::WindowListEntry,
        quicknav::{persistence::MockQuickNavPersistence, QuickNavDatabase},
        ui::{mode, model::AppModel},
    };
    use crossterm::event::{KeyEventState, KeyModifiers};

    use crate::{
        kitty_model::{entry_type::EntryType, MockKittyModel},
        ui::mode::Mode::Navigate,
    };

    use super::*;

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

    #[test]
    fn given_0_selected_when_j_pressed_1_selected() {
        let quicknav_persistence = MockQuickNavPersistence::default();
        let kitty_model = MockKittyModel::new();
        let mut model = AppModel::new(basic_windows(), QuickNavDatabase::new(), Navigate);
        let event = KeyEvent::new_with_kind_and_state(
            KeyCode::Char('j'),
            KeyModifiers::empty(),
            crossterm::event::KeyEventKind::Press,
            KeyEventState::NONE,
        );
        let cmds =
            NavigateMode::handle_input(&event).unwrap();
        let cmd = cmds.get(0).expect("No command returned");
        model = Result::expect(
            cmd.execute(&kitty_model, &quicknav_persistence, model),
            "Command returned an error when executed",
        );
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

        assert_eq!(*model.selected().unwrap(), expected);
    }

    #[test]
    fn given_0_selected_when_shift_j_pressed_1_selected() {
        let quicknav_persistence = MockQuickNavPersistence::default();
        let kitty_model = MockKittyModel::new();
        let mut model = AppModel::new(basic_windows(), QuickNavDatabase::new(), Navigate);
        let event = KeyEvent::new_with_kind_and_state(
            KeyCode::Char('J'),
            KeyModifiers::SHIFT,
            crossterm::event::KeyEventKind::Press,
            KeyEventState::NONE,
        );
        let cmds =
            NavigateMode::handle_input(&event).unwrap();
        let cmd = cmds.get(0).expect("No command returned");
        model = Result::expect(
            cmd.execute(&kitty_model, &quicknav_persistence, model),
            "Command returned an error",
        );
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

        assert_eq!(*model.selected().unwrap(), expected);
    }

    #[test]
    fn given_1_selected_when_shift_j_pressed_3_selected() {
        let quicknav_persistence = MockQuickNavPersistence::default();
        let kitty_model = MockKittyModel::new();
        let mut model = AppModel::new(basic_windows(), QuickNavDatabase::new(), Navigate);
        model.state().select(Some(1));

        let event = KeyEvent::new_with_kind_and_state(
            KeyCode::Char('J'),
            KeyModifiers::SHIFT,
            crossterm::event::KeyEventKind::Press,
            KeyEventState::NONE,
        );
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

        let cmds =
            NavigateMode::handle_input(&event).unwrap();
        let cmd = cmds.get(0).expect("No command returned");
        model = Result::expect(
            cmd.execute(&kitty_model, &quicknav_persistence, model),
            "Command returned an error",
        );
        assert_eq!(*model.selected().unwrap(), expected);
    }

    #[test]
    fn given_3_selected_when_shift_k_pressed_1_selected() {
        let quicknav_persistence = MockQuickNavPersistence::default();
        let kitty_model = MockKittyModel::new();

        let mut model = AppModel::new(basic_windows(), QuickNavDatabase::new(), Navigate);
        model.state().select(Some(3));

        let event = KeyEvent::new_with_kind_and_state(
            KeyCode::Char('K'),
            KeyModifiers::SHIFT,
            crossterm::event::KeyEventKind::Press,
            KeyEventState::NONE,
        );
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

        let cmds =
            NavigateMode::handle_input(&event).unwrap();
        let cmd = cmds.get(0).expect("No command returned");
        model = Result::expect(
            cmd.execute(&kitty_model, &quicknav_persistence, model),
            "Command returned an error",
        );
        assert_eq!(*model.selected().unwrap(), expected);
    }

    #[test]
    fn given_1_selected_when_x_pressed_then_close_entry_called() {
        let quicknav_persistence = MockQuickNavPersistence::default();
        let mut mock_window_list = MockKittyModel::new();
        mock_window_list
            .expect_close_entry()
            .withf(|_entry: &WindowListEntry| true)
            .times(1)
            .returning(|_| ());

        let kitty_model = mock_window_list;

        let mut model = AppModel::new(basic_windows(), QuickNavDatabase::new(), Navigate);
        model.state().select(Some(1));

        let event = KeyEvent::new_with_kind_and_state(
            KeyCode::Char('x'),
            KeyModifiers::empty(),
            crossterm::event::KeyEventKind::Press,
            KeyEventState::NONE,
        );
        let cmds =
            NavigateMode::handle_input(&event).unwrap();
        let cmd = cmds.get(0).expect("No command returned");
        Result::expect(
            cmd.execute(&kitty_model, &quicknav_persistence, model),
            "Command returned an error",
        );
    }

    #[test]
    fn given_1_selected_when_a_pressed_then_rename_entry_mode_entered() {
        let quicknav_persistence = MockQuickNavPersistence::default();
        let mock_window_list = MockKittyModel::new();

        let kitty_model = mock_window_list;

        let mut model = AppModel::new(basic_windows(), QuickNavDatabase::new(), Navigate);
        model.state().select(Some(1));

        let event = KeyEvent::new_with_kind_and_state(
            KeyCode::Char('a'),
            KeyModifiers::empty(),
            crossterm::event::KeyEventKind::Press,
            KeyEventState::NONE,
        );
        let cmds =
            NavigateMode::handle_input(&event).unwrap();
        let cmd = cmds.get(0).expect("No command returned");
        model = Result::expect(
            cmd.execute(&kitty_model, &quicknav_persistence, model),
            "Command returned an error",
        );

        assert_eq!(model.mode(), mode::Mode::Rename);
    }

    #[test]
    fn given_1_selected_when_m_pressed_then_setquicknav_mode_entered() {
        let quicknav_persistence = MockQuickNavPersistence::default();
        let mock_window_list = MockKittyModel::new();

        let kitty_model = mock_window_list;

        let mut model = AppModel::new(basic_windows(), QuickNavDatabase::new(), Navigate);
        model.state().select(Some(2));

        let event = KeyEvent::new_with_kind_and_state(
            KeyCode::Char('m'),
            KeyModifiers::empty(),
            crossterm::event::KeyEventKind::Press,
            KeyEventState::NONE,
        );
        let cmds =
            NavigateMode::handle_input(&event).unwrap();
        let cmd = cmds.get(0).expect("No command returned");
        model = Result::expect(
            cmd.execute(&kitty_model, &quicknav_persistence, model),
            "Command returned an error",
        );

        assert_eq!(model.mode(), mode::Mode::SetQuickNav);
    }

    #[test]
    fn given_1_selected_when_apostraphe_pressed_then_quicknav_mode_entered() {
        let quicknav_persistence = MockQuickNavPersistence::default();
        let mock_window_list = MockKittyModel::new();

        let kitty_model = mock_window_list;

        let mut model = AppModel::new(basic_windows(), QuickNavDatabase::new(), Navigate);
        model.state().select(Some(1));

        let event = KeyEvent::new_with_kind_and_state(
            KeyCode::Char('\''),
            KeyModifiers::empty(),
            crossterm::event::KeyEventKind::Press,
            KeyEventState::NONE,
        );
        let cmds =
            NavigateMode::handle_input(&event).unwrap();
        let cmd = cmds.get(0).expect("No command returned");
        model = Result::expect(
            cmd.execute(&kitty_model, &quicknav_persistence, model),
            "Command returned an error",
        );

        assert_eq!(model.mode(), mode::Mode::QuickNav);
    }
}
