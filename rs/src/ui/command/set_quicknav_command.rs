use crate::{
    error::KittyMuxError,
    kitty_model::KittyModel,
    quicknav::{persistence::QuickNavPersistence, QuickNavEntry},
    ui::model::AppModel,
};

use super::Command;

pub struct SetQuickNavCommand {
    pub quick_nav: char,
}

impl SetQuickNavCommand {
    pub fn new(quick_nav: char) -> Self {
        SetQuickNavCommand { quick_nav }
    }
}

impl Command for SetQuickNavCommand {
    fn execute(
        &self,
        _kitty_model: &dyn KittyModel,
        quick_nav_persistence: &dyn QuickNavPersistence,
        mut model: AppModel,
    ) -> Result<AppModel, KittyMuxError> {
        if let Some(selected) = model.selected() {
            let title = selected.title.clone();
            let id = selected.id;
            model
                .quicknavs_mut()
                .add_entry(QuickNavEntry::new(title, self.quick_nav, id));

            quick_nav_persistence.save(model.quicknavs())?;
        }
        Ok(model)
    }
}
