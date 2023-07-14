use crate::{
    error::KittyMuxError, kitty_model::KittyModel, quicknav::persistence::QuickNavPersistence,
    ui::mode::Mode::Navigate, ui::model::AppModel,
};

use super::Command;

pub struct EnterNavigateCommand {}

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
    ) -> Result<AppModel, KittyMuxError> {
        model.set_mode(Navigate);
        Ok(model)
    }
}
