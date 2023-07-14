use crate::{kitty_model::KittyModel, quicknav::persistence::QuickNavPersistence, error::KittyMuxError};
use super::{command::Command, mode::Mode::Navigate, model::AppModel};

pub struct EnterNavigateCommand { }

impl EnterNavigateCommand {
    pub fn new() -> Self {
        EnterNavigateCommand {}
    }
}

impl Command for EnterNavigateCommand {
    fn execute(
        &self,
        _kitty_model: &dyn KittyModel,
        _quick_nav_persistence: &dyn QuickNavPersistence,
        mut model: AppModel,
    ) -> Result<super::model::AppModel, KittyMuxError> {
        model.set_mode(Navigate);
        Ok(model)
    }
}

