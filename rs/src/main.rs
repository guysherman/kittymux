use error::KittyMuxError;
use kitty_model::BaseKittyModel;
use kitty_connector::{
    command_executor::KittyCommandExecutor,
    KittyConnector,
};
use quicknav::persistence::{ConfigFileQuickNavPersistence, get_quicknav_file_path};

mod kitty_model;
mod kitty_connector;
mod ui;
mod quicknav;
mod error;


fn main() -> Result<(), KittyMuxError> {
    let kc = KittyConnector { executor: &KittyCommandExecutor {} };
    let km = BaseKittyModel::new(kc);
    let qnp = ConfigFileQuickNavPersistence::new(get_quicknav_file_path());
    ui::run(&km, &qnp)
}
