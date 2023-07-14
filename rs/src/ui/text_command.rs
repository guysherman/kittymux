use crate::{kitty_model::KittyModel, quicknav::persistence::QuickNavPersistence, error::KittyMuxError};

use super::{command::Command, model::AppModel};

pub struct TextCommand {
    character: Option<char>,
}

impl TextCommand {
    pub fn new(character: Option<char>) -> Self {
        TextCommand {
            character,
        }
    }
}

impl Command for TextCommand {
    fn execute(
        &self,
        _kitty_model: &dyn KittyModel,
        _quick_nav_persistence: &dyn QuickNavPersistence,
        mut model: AppModel,
    ) -> Result<AppModel, KittyMuxError> {
        if let Some(c) = self.character {
            model.text_input.push(c);
        } else {
            model.text_input.pop();
        }
        Ok(model)
    }
}
