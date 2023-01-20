use std::error::Error;

use crate::kitty_model::KittyModel;

use super::{model::AppModel, command::Command};

pub struct NoopCommand {
    model: Option<AppModel>
}

impl NoopCommand {
    pub fn new(model: AppModel) -> NoopCommand {
        NoopCommand {
            model: Some(model)
        }
    }
}

impl Command for NoopCommand {
    fn execute(&mut self, _kitty_model: &dyn KittyModel) -> Result<Option<AppModel>, Box<dyn Error>> {
        Ok(self.model.take())
    }
}
