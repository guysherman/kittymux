use crate::{
    error::KittyMuxError, kitty_model::KittyModel, quicknav::persistence::QuickNavPersistence,
};

use super::{command::Command, model::AppModel};

pub struct QuitCommand {
    model: Option<AppModel>,
}

impl QuitCommand {
    pub fn new(mut model: AppModel) -> QuitCommand {
        model.quit();
        QuitCommand { model: Some(model) }
    }
}

impl Command for QuitCommand {
    fn execute(
        &mut self,
        _kitty_model: &dyn KittyModel,
        quick_nav_persistence: &dyn QuickNavPersistence,
    ) -> Result<Option<AppModel>, KittyMuxError> {
        if let Some(quicknavs) = self.model.as_ref().map(|model| model.quicknavs()) {
            quick_nav_persistence.save(quicknavs)?;
        }
        Ok(self.model.take())
    }
}
