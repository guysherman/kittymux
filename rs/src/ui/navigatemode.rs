use std::error::Error;

use crossterm::event::{KeyCode, KeyEvent};

use crate::kitty_model::KittyModel;

use super::{
    command::Command, enter_rename_command::EnterRenameCommand,
    enter_setquicknav_command::EnterSetQuickNavCommand, load_command::LoadCommand, model::AppModel,
    noop_command::NoopCommand, quit_command::QuitCommand,
};

pub struct NavigateMode {}

impl NavigateMode {
    pub fn handle_input(
        event: &KeyEvent,
        mut model: AppModel,
        kitty_model: &dyn KittyModel,
    ) -> Result<Box<dyn Command>, Box<dyn Error>> {
        match event.code {
            KeyCode::Char('q') => Ok(Box::new(QuitCommand::new(model))),
            KeyCode::Esc => Ok(Box::new(QuitCommand::new(model))),
            KeyCode::Char('j') => {
                model.select_next();
                Ok(Box::new(NoopCommand::new(model)))
            }
            KeyCode::Char('k') => {
                model.select_prev();
                Ok(Box::new(NoopCommand::new(model)))
            }
            KeyCode::Char('J') => {
                model.select_next_tab();
                Ok(Box::new(NoopCommand::new(model)))
            }
            KeyCode::Char('K') => {
                model.select_prev_tab();
                Ok(Box::new(NoopCommand::new(model)))
            }
            KeyCode::Char('x') => {
                model.selected().map(|entry| kitty_model.close_entry(entry));
                Ok(Box::new(LoadCommand::new()))
            }
            KeyCode::Char('a') => Ok(Box::new(EnterRenameCommand::new(model))),
            KeyCode::Enter => {
                model.selected().map(|selected_item| {
                    kitty_model.focus_entry(selected_item);
                });
                Ok(Box::new(QuitCommand::new(model)))
            }
            KeyCode::Char('m') => Ok(Box::new(EnterSetQuickNavCommand::new(model))),
            _ => Ok(Box::new(NoopCommand::new(model))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        kitty_model::window_list_entry::WindowListEntry, quicknav::QuickNavDatabase, ui::mode,
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
        let kitty_model = MockKittyModel::new();
        let mut model = AppModel::new(basic_windows(), QuickNavDatabase::new(), Navigate);
        let event = KeyEvent::new_with_kind_and_state(
            KeyCode::Char('j'),
            KeyModifiers::empty(),
            crossterm::event::KeyEventKind::Press,
            KeyEventState::NONE,
        );
        let mut cmd = NavigateMode::handle_input(&event, model, &kitty_model).unwrap();
        model = Result::expect(
            cmd.execute(&kitty_model),
            "Command returned an error when executed",
        )
        .unwrap();
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
        let kitty_model = MockKittyModel::new();
        let mut model = AppModel::new(basic_windows(), QuickNavDatabase::new(), Navigate);
        let event = KeyEvent::new_with_kind_and_state(
            KeyCode::Char('J'),
            KeyModifiers::SHIFT,
            crossterm::event::KeyEventKind::Press,
            KeyEventState::NONE,
        );
        let mut cmd = NavigateMode::handle_input(&event, model, &kitty_model).unwrap();
        model = Result::expect(cmd.execute(&kitty_model), "Command returned an error").unwrap();
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

        let mut cmd = NavigateMode::handle_input(&event, model, &kitty_model).unwrap();
        model = Result::expect(cmd.execute(&kitty_model), "Command returned error").unwrap();
        assert_eq!(*model.selected().unwrap(), expected);
    }

    #[test]
    fn given_3_selected_when_shift_k_pressed_1_selected() {
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

        let mut cmd = NavigateMode::handle_input(&event, model, &kitty_model).unwrap();
        model = Result::expect(cmd.execute(&kitty_model), "Command returned an error").unwrap();
        assert_eq!(*model.selected().unwrap(), expected);
    }

    #[test]
    fn given_1_selected_when_x_pressed_then_close_entry_called() {
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
        Result::expect(
            NavigateMode::handle_input(&event, model, &kitty_model),
            "handle_input threw an error",
        );
    }

    #[test]
    fn given_1_selected_when_a_pressed_then_rename_entry_mode_entered() {
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
        let mut cmd = NavigateMode::handle_input(&event, model, &kitty_model).unwrap();
        let result = cmd
            .execute(&kitty_model)
            .unwrap()
            .expect("Command had no AppModel");

        assert_eq!(result.mode(), mode::Mode::Rename);
    }

    #[test]
    fn given_1_selected_when_m_pressed_then_setquicknav_mode_entered() {
        let mock_window_list = MockKittyModel::new();

        let kitty_model = mock_window_list;

        let mut model = AppModel::new(basic_windows(), QuickNavDatabase::new(), Navigate);
        model.state().select(Some(1));

        let event = KeyEvent::new_with_kind_and_state(
            KeyCode::Char('m'),
            KeyModifiers::empty(),
            crossterm::event::KeyEventKind::Press,
            KeyEventState::NONE,
        );
        let mut cmd = NavigateMode::handle_input(&event, model, &kitty_model).unwrap();
        let result = cmd
            .execute(&kitty_model)
            .unwrap()
            .expect("Command had no AppModel");

        assert_eq!(result.mode(), mode::Mode::SetQuickNav);
    }
}
