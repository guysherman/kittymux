use crate::{kitty_model::KittyModel, quicknav::QuickNavDatabase};

use super::{command::Command, model::AppModel};

pub struct RenameEntryCommand {
    model: Option<AppModel>,
}

impl RenameEntryCommand {
    pub fn new(model: AppModel) -> Self {
        RenameEntryCommand { model: Some(model) }
    }
}

impl Command for RenameEntryCommand {
    fn execute(
        &mut self,
        kitty_model: &dyn KittyModel,
    ) -> Result<Option<super::model::AppModel>, Box<dyn std::error::Error>> {
        self.model.as_ref().map(|m| {
            m.selected().as_ref().map(|s| {
                kitty_model.rename_entry(s, &m.text_input.as_ref());
            });
        });

        let selected_index: Option<usize> =
            self.model.as_ref().map_or(None, |m| m.selected_index());
        Ok(Some(
            AppModel::new(
                kitty_model.load()?,
                QuickNavDatabase::load(),
                super::mode::Mode::Navigate,
            )
            .with_selected(selected_index),
        ))
    }
}
