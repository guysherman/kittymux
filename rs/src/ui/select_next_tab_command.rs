use crate::{kitty_model::KittyModel, quicknav::persistence::QuickNavPersistence, error::KittyMuxError};

use super::{model::AppModel, command::Command};

pub struct SelectNextTabCommand {
}

impl SelectNextTabCommand {
    pub fn new() -> SelectNextTabCommand {
        SelectNextTabCommand {}
    }
}

impl Command for SelectNextTabCommand {
    fn execute(
        &self,
        _kitty_model: &dyn KittyModel,
        _quick_nav_persistence: &dyn QuickNavPersistence,
        mut model: AppModel,
    ) -> Result<AppModel, KittyMuxError> {
        model.select_next_tab();
        Ok(model)
    }
}


