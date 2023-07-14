use crate::{
    error::KittyMuxError, kitty_model::KittyModel, quicknav::persistence::QuickNavPersistence,
};

use super::{command::Command, model::AppModel};

pub struct QuitCommand { }

impl QuitCommand {
    pub fn new() -> QuitCommand {
        QuitCommand {}
    }
}

impl Command for QuitCommand {
    fn execute(
        &self,
        _kitty_model: &dyn KittyModel,
        _quick_nav_persistence: &dyn QuickNavPersistence,
        mut model: AppModel,
    ) -> Result<AppModel, KittyMuxError> {
        model.quit();
        Ok(model)
    }
}
