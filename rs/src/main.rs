use clap::Parser;
use error::KittyMuxError;
use kitty_connector::{command_executor::KittyCommandExecutor, KittyConnector};
use kitty_model::{BaseKittyModel, KittyModel};
use quicknav::persistence::{
    get_quicknav_file_path, ConfigFileQuickNavPersistence, QuickNavPersistence,
};

use crate::kitty_model::entry_type::EntryType;

mod error;
mod kitty_connector;
mod kitty_model;
mod quicknav;
mod ui;

/// A program to manage kitty windows and tabs
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Sets a quicknav key. Requires <window-id> to be supplied.
    #[arg(short = 'k', long = "key", value_name = "key")]
    key: Option<char>,
    /// The window id to set a quicknav for, if supplied without <key> then this does nothing
    #[arg(short = 'w', long = "window-id", value_name = "window-id")]
    window_id: Option<u32>,
}

fn main() -> Result<(), KittyMuxError> {
    let kc = KittyConnector {
        executor: &KittyCommandExecutor {},
    };
    let km = BaseKittyModel::new(kc);
    let qnp = ConfigFileQuickNavPersistence::new(get_quicknav_file_path());

    let args = Args::parse();
    if let Some(key) = args.key {
        if let Some(window_id) = args.window_id {
            let windows = km.load()?;
            let title = windows
                .iter()
                .find(|w| w.id == window_id && w.entry_type == EntryType::Window)
                .map(|w| w.title.clone())
                .ok_or(error::InvalidWindowIdError { window_id })?;
            let mut quicknavs = qnp.load()?;
            quicknavs.add_entry(quicknav::QuickNavEntry {
                key,
                id: window_id,
                title,
            });
            qnp.save(&quicknavs)?;
            return Ok(());
        } else {
            return Err(KittyMuxError::MissingArgumentError(
                error::MissingArgumentError {
                    arugment: "window-id".to_string(),
                },
            ));
        }
    }

    ui::run(&km, &qnp)
}
