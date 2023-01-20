use std::error::Error;

use crate::kitty_model::KittyModel;

use super::{model::AppModel, command::Command};

pub struct QuitCommand {
    model: Option<AppModel>
}

impl QuitCommand {
    pub fn new(mut model: AppModel) -> QuitCommand {
        model.quit();
        QuitCommand {
            model: Some(model)
        }
    }
}

impl Command for QuitCommand {
    fn execute(&mut self, _kitty_model: &dyn KittyModel) -> Result<Option<AppModel>, Box<dyn Error>> {
        Ok(self.model.take())
    }
}

