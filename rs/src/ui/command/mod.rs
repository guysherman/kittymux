pub mod close_entry_command;
pub mod enter_navigate_command;
pub mod enter_quicknav_command;
pub mod enter_rename_command;
pub mod enter_setquicknav_command;
pub mod load_command;
pub mod quicknav_command;
pub mod quit_command;
pub mod rename_entry_command;
pub mod select_entry_command;
pub mod select_next_command;
pub mod select_next_tab_command;
pub mod select_prev_command;
pub mod select_prev_tab_command;
pub mod set_quicknav_command;
pub mod text_command;

use crate::{kitty_model::KittyModel, quicknav::persistence::QuickNavPersistence, error::KittyMuxError};

use super::model::AppModel;

pub trait Command {
    fn execute(
        &self,
        kitty_model: &dyn KittyModel,
        quick_nav_persistence: &dyn QuickNavPersistence,
        model: AppModel,
    ) -> Result<AppModel, KittyMuxError>;
}
