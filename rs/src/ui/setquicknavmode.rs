use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    error::KittyMuxError, kitty_model::KittyModel, quicknav::{QuickNavEntry, persistence::QuickNavPersistence},
    ui::enter_navigate_command::EnterNavigateCommand,
};

use super::{command::Command, model::AppModel};

pub struct SetQuickNavMode {}

impl SetQuickNavMode {
    pub fn handle_input(
        event: &KeyEvent,
        mut model: AppModel,
        _kitty_model: &dyn KittyModel,
        quick_nav_persistence: &dyn QuickNavPersistence,
    ) -> Result<Box<dyn Command>, KittyMuxError> {
        match event.code {
            KeyCode::Char(c) => {
                if let Some(selected) = model.selected() {
                    let title = selected.title.clone();
                    let id = selected.id;
                    model
                        .quicknavs_mut()
                        .add_entry(QuickNavEntry::new(title, c, id));

                    quick_nav_persistence.save(model.quicknavs())?;
                }
            }
            _ => (),
        }
        Ok(Box::new(EnterNavigateCommand::new(model)))
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

        let mut command = SetQuickNavMode::handle_input(&event, model, &kitty_model, &quicknav_persistence)
            .expect("handle_input failed");
        let result = command
            .execute(&kitty_model, &quicknav_persistence)
            .expect("execute failed")
            .expect("did not contain a model");

        assert_eq!(result.mode(), mode::Mode::Navigate);
        assert_eq!(
            result
                .quicknavs()
                .find_entry_by_id(1)
                .expect("quicknav entry not created")
                .key,
            'a'
        );
    }
}
