use entry_list::KittyEntryList;
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
    let el = KittyEntryList::new(&kc);
    ui::run(&el)
}
