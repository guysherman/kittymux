use crate::{
    error::KittyMuxError,
    kitty_model::KittyModel,
    quicknav::persistence::QuickNavPersistence,
    ui::{mode::Mode::Rename, model::AppModel},
};

use super::Command;

pub struct EnterRenameCommand {}

impl EnterRenameCommand {
    pub fn new() -> Self {
        EnterRenameCommand {}
    }
}

impl Command for EnterRenameCommand {
    fn execute(
        &self,
        _kitty_model: &dyn KittyModel,
        _quick_nav_persistence: &dyn QuickNavPersistence,
        mut model: AppModel,
    ) -> Result<AppModel, KittyMuxError> {
        model.set_mode(Rename);
        let selected_text = model
            .selected()
            .map(|md| md.title.clone())
            .unwrap_or_default();
        model.text_input = selected_text;
        Ok(model)
    }
}

#[cfg(test)]
mod tests {
    use super::EnterRenameCommand;
    use crate::{
        kitty_model::{entry_type, window_list_entry::WindowListEntry, MockKittyModel},
        quicknav::{persistence::MockQuickNavPersistence, QuickNavDatabase},
        ui::{mode, model},
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
    fn returns_a_model_with_rename_mode() {
        let initial_model = model::AppModel::new(
            basic_windows(),
            QuickNavDatabase::new(),
            mode::Mode::Navigate,
        );
        let mock_window_list = MockKittyModel::new();
        let qnp = MockQuickNavPersistence::default();

        let cmd = EnterRenameCommand::new();
        let result = cmd
            .execute(&mock_window_list, &qnp, initial_model)
            .expect("Result had no AppModel");

        assert_eq!(result.mode(), mode::Mode::Rename);
    }
}
