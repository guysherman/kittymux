use crate::kitty_model::KittyModel;

use super::{command::Command, mode::Mode::Rename, model::AppModel};

pub struct EnterRenameCommand {
    model: Option<AppModel>,
}

impl EnterRenameCommand {
    pub fn new(model: AppModel) -> Self {
        EnterRenameCommand { model: Some(model) }
    }
}

impl Command for EnterRenameCommand {
    fn execute(
        &mut self,
        _kitty_model: &dyn KittyModel,
    ) -> Result<Option<super::model::AppModel>, Box<dyn std::error::Error>> {
        let model = self.model.as_mut().expect("Command did not have a model");
        model.set_mode(Rename);
        let selected_text = model
            .selected()
            .map(|md| md.title.clone())
            .unwrap_or_default();
        model.text_input = selected_text;
        Ok(self.model.take())
    }
}

#[cfg(test)]
mod tests {
    use crate::{kitty_model::{entry_type, window_list_entry::WindowListEntry, MockKittyModel}, ui::{model, mode, enter_rename_command::EnterRenameCommand}, quicknav::QuickNavDatabase};

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
        let initial_model = model::AppModel::new(basic_windows(), QuickNavDatabase::new(), mode::Mode::Navigate);
        let mock_window_list = MockKittyModel::new();

        let mut cmd = EnterRenameCommand::new(initial_model);
        let result = cmd.execute(&mock_window_list).unwrap().expect("Result had no AppModel");

        assert_eq!(result.mode(), mode::Mode::Rename);
    }
}
