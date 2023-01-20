use std::error::Error;

use kitty_model::{BaseKittyModel, KittyModel};
use kitty_connector::{
    command_executor::KittyCommandExecutor,
    KittyConnector,
};

mod kitty_model;
mod kitty_connector;
mod ui;


fn main() -> Result<(), Box<dyn Error>> {
    let kc = KittyConnector { executor: &KittyCommandExecutor {} };
    let km = BaseKittyModel::new(kc);
    ui::run(&km)
}
