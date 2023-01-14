use kitty_model::BaseKittyModel;
use kitty_connector::{
    command_executor::{CommandExecutor, KittyCommandExecutor},
    KittyConnector,
};

mod kitty_model;
mod kitty_connector;
mod ui;

fn main() -> Result<(), std::io::Error> {
    let ce = KittyCommandExecutor {};
    let kc = KittyConnector { executor: &ce };
    let el = BaseKittyModel::new(&kc);
    ui::run(&el)
}
