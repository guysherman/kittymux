use std::error::Error;

use crate::{kitty_model::KittyModel, quicknav::QuickNavDatabase};

use super::{command::Command, model::AppModel};

pub struct LoadCommand {
    selected: Option<usize>
}

impl<'a> LoadCommand {
    pub fn new() -> LoadCommand {
        LoadCommand { selected: None }
    }

    pub fn with_selected(mut self, selected: Option<usize>) -> LoadCommand {
        self.selected = selected;
        self
    }
}

impl Command for LoadCommand {
    fn execute(&mut self, kitty_model: &dyn KittyModel) -> Result<Option<AppModel>, Box<dyn Error>> {
        let selected = self.selected;
        let mut new_model = AppModel::new(kitty_model.load()?, QuickNavDatabase::load(), super::mode::Mode::Navigate);
        new_model.select(selected);
        Ok(Some(new_model))
    }
}
