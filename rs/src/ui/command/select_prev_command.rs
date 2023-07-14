use crate::{error::KittyMuxError, quicknav::persistence::QuickNavPersistence, kitty_model::KittyModel, ui::model::AppModel};

use super::Command;

pub struct SelectPrevCommand {
}

impl SelectPrevCommand {
    pub fn new() -> SelectPrevCommand {
        SelectPrevCommand {}
    }
}

impl Command for SelectPrevCommand {
    fn execute(
        &self,
        _kitty_model: &dyn KittyModel,
        _quick_nav_persistence: &dyn QuickNavPersistence,
        mut model: AppModel,
    ) -> Result<AppModel, KittyMuxError> {
        model.select_prev();
        Ok(model)
    }
}

