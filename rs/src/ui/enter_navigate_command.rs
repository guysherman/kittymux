use crate::kitty_model::KittyModel;
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
    ) -> Result<Option<super::model::AppModel>, Box<dyn std::error::Error>> {
        let model = self.model.as_mut().expect("Command did not have a model");
        model.set_mode(Navigate);
        Ok(self.model.take())
    }
}

