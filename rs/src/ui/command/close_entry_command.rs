use crate::{kitty_model::KittyModel, quicknav::persistence::QuickNavPersistence, error::KittyMuxError, ui::model::AppModel};

use super::Command;

pub struct CloseEntryCommand {
}

impl CloseEntryCommand {
    pub fn new() -> CloseEntryCommand {
        CloseEntryCommand {}
    }
}


impl Command for CloseEntryCommand {
    fn execute(
        &self,
        kitty_model: &dyn KittyModel,
        _quick_nav_persistence: &dyn QuickNavPersistence,
        model: AppModel,
    ) -> Result<AppModel, KittyMuxError> {
        model.selected().map(|entry| kitty_model.close_entry(entry));
        Ok(model)
    }
}
