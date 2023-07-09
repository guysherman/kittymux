use crate::{kitty_model::KittyModel, quicknav::persistence::QuickNavPersistence, error::KittyMuxError};
use super::{command::Command, mode::Mode::Navigate, model::AppModel};

pub struct EnterNavigateCommand {
    model: Option<AppModel>,
}

impl EnterNavigateCommand {
    pub fn new(model: AppModel) -> Self {
        EnterNavigateCommand { model: Some(model) }
    }
}

impl Command for EnterNavigateCommand {
    fn execute(
        &mut self,
        _kitty_model: &dyn KittyModel,
        _quick_nav_persistence: &dyn QuickNavPersistence,
    ) -> Result<Option<super::model::AppModel>, KittyMuxError> {
        let model = self.model.as_mut().expect("Command did not have a model");
        model.set_mode(Navigate);
        Ok(self.model.take())
    }
}

