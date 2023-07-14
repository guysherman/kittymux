use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    error::KittyMuxError,
    ui::enter_navigate_command::EnterNavigateCommand,
};

use super::{command::Command, set_quicknav_command::SetQuickNavCommand};

pub struct SetQuickNavMode {}

impl SetQuickNavMode {
    pub fn handle_input(
        event: &KeyEvent,
    ) -> Result<Vec<Box<dyn Command>>, KittyMuxError> {
        match event.code {
            KeyCode::Char(c) => match c {
                '0'..='9' | 'a'..='z' => Ok(vec![
                    Box::new(SetQuickNavCommand::new(c)),
                    Box::new(EnterNavigateCommand::new()),
                ]),
                _ => Ok(vec![]),
            },
            KeyCode::Esc => Ok(vec![Box::new(EnterNavigateCommand::new())]),
            _ => Ok(vec![]),
        }
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEvent, KeyEventState, KeyModifiers};

    use crate::{
        kitty_model::{entry_type, window_list_entry::WindowListEntry, MockKittyModel},
        quicknav::{persistence::MockQuickNavPersistence, QuickNavDatabase},
        ui::{mode, model::AppModel},
    };

    use super::SetQuickNavMode;

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
    fn when_a_pressed_quicknav_set_under_tabid() {
        let mut quicknav_persistence = MockQuickNavPersistence::default();
        quicknav_persistence
            .expect_save()
            .times(1)
            .returning(|_| Ok(()));
        let mock_window_list = MockKittyModel::new();
        let kitty_model = mock_window_list;

        let mut model = AppModel::new(basic_windows(), QuickNavDatabase::new(), mode::Mode::Rename);
        model.state().select(Some(0));

        let event = KeyEvent::new_with_kind_and_state(
            KeyCode::Char('a'),
            KeyModifiers::empty(),
            crossterm::event::KeyEventKind::Press,
            KeyEventState::NONE,
        );

        let cmds = SetQuickNavMode::handle_input(&event)
            .expect("handle_input failed");
        let mut it = cmds.iter();
        while let Some(cmd) = it.next() {
            model = cmd.execute(&kitty_model, &quicknav_persistence, model).expect("command failed");
        }

        assert_eq!(model.mode(), mode::Mode::Navigate);
        assert_eq!(
            model
                .quicknavs()
                .find_entry_by_id(1)
                .expect("quicknav entry not created")
                .key,
            'a'
        );
    }
}
