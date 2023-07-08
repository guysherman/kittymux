use crate::kitty_model::KittyModel;

use super::{command::Command, mode::Mode::SetQuickNav, model::AppModel};

pub struct EnterSetQuickNavCommand {
    model: Option<AppModel>,
}

impl EnterSetQuickNavCommand {
    pub fn new(model: AppModel) -> Self {
        Self { model: Some(model) }
    }
}

impl Command for EnterSetQuickNavCommand {
    fn execute(
        &mut self,
        _kitty_model: &dyn KittyModel,
    ) -> Result<Option<AppModel>, Box<dyn std::error::Error>> {
        let model = self.model.as_mut().unwrap();
        model.set_mode(SetQuickNav);
        Ok(self.model.take())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{kitty_model::{entry_type, window_list_entry::WindowListEntry, MockKittyModel}, quicknav::QuickNavDatabase};

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
        let mut kitty_model = MockKittyModel::new();
        let mut command = EnterSetQuickNavCommand::new(AppModel::new(
            basic_windows(),
            QuickNavDatabase::new(),
            crate::ui::mode::Mode::Navigate,
        ));
        let result = command.execute(&kitty_model);
        assert!(result.is_ok());
        let model = result.unwrap().unwrap();
        assert_eq!(model.mode(), SetQuickNav);
    }
}
