use crate::{kitty_model::KittyModel, quicknav::persistence::QuickNavPersistence, error::KittyMuxError};

use super::{command::Command, model::AppModel};

pub struct LoadCommand {
    selected: Option<usize>,
}

impl<'a> LoadCommand {
    pub fn new() -> LoadCommand {
        LoadCommand { selected: None }
    }
}

impl Command for LoadCommand {
    fn execute(
        &mut self,
        kitty_model: &dyn KittyModel,
        _quick_nav_persistence: &dyn QuickNavPersistence,
    ) -> Result<Option<AppModel>, KittyMuxError> {
        let selected = self.selected;
        let mut new_model = AppModel::new(
            kitty_model.load()?,
            _quick_nav_persistence.load()?,
            super::mode::Mode::Navigate,
        );
        new_model.select(selected);
        Ok(Some(new_model))
    }
}
