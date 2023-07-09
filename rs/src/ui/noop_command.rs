use std::error::Error;

use crate::{kitty_model::KittyModel, quicknav::persistence::QuickNavPersistence, error::KittyMuxError};

use super::{command::Command, model::AppModel};

pub struct NoopCommand {
    model: Option<AppModel>,
}

impl NoopCommand {
    pub fn new(model: AppModel) -> NoopCommand {
        NoopCommand { model: Some(model) }
    }
}

impl Command for NoopCommand {
    fn execute(
        &mut self,
        _kitty_model: &dyn KittyModel,
        _quick_nav_persistence: &dyn QuickNavPersistence,
    ) -> Result<Option<AppModel>, KittyMuxError> {
        Ok(self.model.take())
    }
}
