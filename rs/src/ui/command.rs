use crate::{kitty_model::KittyModel, quicknav::persistence::QuickNavPersistence, error::KittyMuxError};

use super::model::AppModel;

pub trait Command {
    fn execute(
        &mut self,
        kitty_model: &dyn KittyModel,
        quick_nav_persistence: &dyn QuickNavPersistence,
    ) -> Result<Option<AppModel>, KittyMuxError>;
}
