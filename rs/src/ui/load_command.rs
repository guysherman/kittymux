use std::error::Error;

use crate::kitty_model::KittyModel;

use super::{command::Command, model::AppModel};

pub struct LoadCommand {}

impl<'a> LoadCommand {
    pub fn new() -> LoadCommand {
        LoadCommand {}
    }
}

impl Command for LoadCommand {
    fn execute(&mut self, kitty_model: &dyn KittyModel) -> Result<Option<AppModel>, Box<dyn Error>> {
        Ok(Some(AppModel::with_items(kitty_model.load()?)))
    }
}
