use std::error::Error;

use crate::kitty_model::KittyModel;

use super::model::AppModel;

pub trait Command {
    fn execute(&mut self, kitty_model: &Box<dyn KittyModel>) -> Result<Option<AppModel>, Box<dyn Error>>;
}
