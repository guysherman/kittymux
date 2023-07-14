use crate::{error::KittyMuxError, quicknav::persistence::QuickNavPersistence, kitty_model::KittyModel, ui::model::AppModel};

use super::Command;

pub struct SelectNextCommand {
}

impl SelectNextCommand {
    pub fn new() -> SelectNextCommand {
        SelectNextCommand {}
    }
}

impl Command for SelectNextCommand {
    fn execute(
        &self,
        _kitty_model: &dyn KittyModel,
        _quick_nav_persistence: &dyn QuickNavPersistence,
        mut model: AppModel,
    ) -> Result<AppModel, KittyMuxError> {
        model.select_next();
        Ok(model)
    }
}
