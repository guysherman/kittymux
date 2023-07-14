use crate::{
    error::KittyMuxError, kitty_model::KittyModel, quicknav::persistence::QuickNavPersistence,
    ui::model::AppModel,
};

use super::Command;

pub struct SelectPrevTabCommand {}

impl SelectPrevTabCommand {
    pub fn new() -> SelectPrevTabCommand {
        SelectPrevTabCommand {}
    }
}

impl Command for SelectPrevTabCommand {
    fn execute(
        &self,
        _kitty_model: &dyn KittyModel,
        _quick_nav_persistence: &dyn QuickNavPersistence,
        mut model: AppModel,
    ) -> Result<AppModel, KittyMuxError> {
        model.select_prev_tab();
        Ok(model)
    }
}
