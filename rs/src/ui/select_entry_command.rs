use crate::{
    error::KittyMuxError, kitty_model::KittyModel, quicknav::persistence::QuickNavPersistence,
};

use super::{command::Command, model::AppModel};

pub struct SelectEntryCommand;

impl SelectEntryCommand {
    pub fn new() -> SelectEntryCommand {
        SelectEntryCommand {}
    }
}

impl Command for SelectEntryCommand {
    fn execute(
        &self,
        kitty_model: &dyn KittyModel,
        _quick_nav_persistence: &dyn QuickNavPersistence,
        mut model: AppModel,
    ) -> Result<AppModel, KittyMuxError> {
        let selected = model.selected();
        selected.map(|selected_item| {
            kitty_model.focus_entry(selected_item);
        });

        if selected.is_some() {
            model.quit();
        }
        Ok(model)
    }
}
