use crate::{
    error::KittyMuxError,
    kitty_model::KittyModel,
    quicknav::persistence::QuickNavPersistence,
};

use super::{command::Command, model::AppModel};

pub struct RenameEntryCommand { }

impl RenameEntryCommand {
    pub fn new() -> Self {
        RenameEntryCommand {}
    }
}

impl Command for RenameEntryCommand {
    fn execute(
        &self,
        kitty_model: &dyn KittyModel,
        quick_nav_persistence: &dyn QuickNavPersistence,
        model: AppModel,
    ) -> Result<super::model::AppModel, KittyMuxError> {
        let mut quicknavs = quick_nav_persistence.load()?;

        if let Some(selected) = model.selected() {
            kitty_model.rename_entry(selected, model.text_input.as_str());
            quicknavs.rename_entry(selected.id, model.text_input.to_owned());
            quick_nav_persistence.save(&quicknavs)?;
        }

        let selected_index: Option<usize> = model.selected_index();
        Ok(AppModel::new(
                kitty_model.load()?,
                quick_nav_persistence.load()?,
                super::mode::Mode::Navigate,
                )
            .with_selected(selected_index),
            )
    }
}
