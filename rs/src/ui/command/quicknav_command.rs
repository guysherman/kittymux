use crate::{
    error::KittyMuxError,
    kitty_model::{KittyModel, entry_type::EntryType},
    quicknav::persistence::QuickNavPersistence, ui::model::AppModel,
};

use super::Command;

pub struct QuickNavCommand {
    pub quick_nav: char,
}

impl QuickNavCommand {
    pub fn new(quick_nav: char) -> Self {
        QuickNavCommand { quick_nav }
    }
}

impl Command for QuickNavCommand {
    fn execute(
        &self,
        kitty_model: &dyn KittyModel,
        _quick_nav_persistence: &dyn QuickNavPersistence,
        mut model: AppModel,
    ) -> Result<AppModel, KittyMuxError> {
        let candidate_ids = model
            .quicknavs()
            .find_entries_by_key(self.quick_nav)
            .iter()
            .map(|e| e.id)
            .collect::<Vec<u32>>();

        let window = model.items().iter().find(|w| {
            candidate_ids.contains(&w.id) && w.tab_is_focused && w.entry_type == EntryType::Window
        });

        if let Some(window) = window {
            kitty_model.focus_entry(window);
            model.quit();
        } 
        Ok(model)
    }
}
