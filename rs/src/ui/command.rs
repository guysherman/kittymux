use std::error::Error;

use super::model::AppModel;

pub trait Command {
    fn execute() -> Result<AppModel, Box<dyn Error>>;
}
