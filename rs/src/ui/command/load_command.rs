use crate::{
    error::KittyMuxError,
    kitty_model::KittyModel,
    quicknav::persistence::QuickNavPersistence,
    ui::{mode::Mode::Navigate, model::AppModel},
};

use super::Command;

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
        &self,
        kitty_model: &dyn KittyModel,
        _quick_nav_persistence: &dyn QuickNavPersistence,
        _model: AppModel,
    ) -> Result<AppModel, KittyMuxError> {
        let selected = self.selected;
        let mut new_model = AppModel::new(
            kitty_model.load()?,
            _quick_nav_persistence.load()?,
            Navigate,
        );
        new_model.select(selected);
        Ok(new_model)
    }
}
