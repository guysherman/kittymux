use crate::{
    error::KittyMuxError,
    kitty_model::{entry_type::EntryType::Window, KittyModel},
    quicknav::persistence::QuickNavPersistence,
};

use super::{command::Command, mode::Mode::SetQuickNav, model::AppModel};

pub struct EnterSetQuickNavCommand {}

impl EnterSetQuickNavCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for EnterSetQuickNavCommand {
    fn execute(
        &self,
        _kitty_model: &dyn KittyModel,
        _quick_nav_persistence: &dyn QuickNavPersistence,
        mut model: AppModel,
    ) -> Result<AppModel, KittyMuxError> {
        if let Some(selected) = model.selected() {
            if let Window = selected.entry_type {
                model.set_mode(SetQuickNav);
            };
        }
        Ok(model)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        kitty_model::{entry_type, window_list_entry::WindowListEntry, MockKittyModel},
        quicknav::{persistence::MockQuickNavPersistence, QuickNavDatabase},
    };

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
    fn test_execute() {
        let kitty_model = MockKittyModel::new();
        let qnp = MockQuickNavPersistence::default();
        let model = AppModel::new(
            basic_windows(),
            QuickNavDatabase::new(),
            crate::ui::mode::Mode::Navigate,
        );
        let command = EnterSetQuickNavCommand::new();
        let result = command.execute(&kitty_model, &qnp, model);
        assert!(result.is_ok());
        let model = result.expect("Command returned an error");
        assert_eq!(model.mode(), SetQuickNav);
    }
}
