use entry_list::FlatWindowList;
use kitty_connector::{
    command_executor::{CommandExecutor, KittyCommandExecutor},
    KittyConnector,
};

mod entry_list;
mod kitty_connector;
mod ui;

fn main() -> Result<(), std::io::Error> {
    let ce = KittyCommandExecutor {};
    let kc = KittyConnector { executor: &ce };
    let el = FlatWindowList::new(&kc);
    ui::run(&el)
}
