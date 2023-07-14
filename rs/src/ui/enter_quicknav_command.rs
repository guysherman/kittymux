use crate::{kitty_model::KittyModel, quicknav::persistence::QuickNavPersistence, error::KittyMuxError};

use super::{command::Command, mode, model::AppModel};

// has an optional AppModel model
pub struct EnterQuickNavCommand { }

impl EnterQuickNavCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for EnterQuickNavCommand {
    fn execute(
        &self,
        _kitty_model: &dyn KittyModel,
        _quick_nav_persistence: &dyn QuickNavPersistence,
        mut model: AppModel,
    ) -> Result<AppModel, KittyMuxError> {
        model.set_mode(mode::Mode::QuickNav);
        Ok(model)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        kitty_model::{entry_type, window_list_entry::WindowListEntry, MockKittyModel},
        quicknav::{QuickNavDatabase, persistence::MockQuickNavPersistence},
        ui::{
            mode::{self, Mode::Navigate},
            model::AppModel,
        },
    };

    use super::Command;

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
    fn test_enter_quicknav_command() {
        let kitty_model = MockKittyModel::default();
        let qnp = MockQuickNavPersistence::default();

        let mut model = AppModel::new(basic_windows(), QuickNavDatabase::new(), Navigate);
        model.select(Some(1));

        let command = super::EnterQuickNavCommand::new();
        let result = command
            .execute(&kitty_model, &qnp, model)
            .expect("Command should succeed");
        assert_eq!(result.mode(), mode::Mode::QuickNav);
    }
}
