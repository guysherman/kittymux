mod command;
mod load_command;
mod mode;
mod model;
mod navigatemode;
mod renamemode;
mod setquicknavmode;
mod quicknavmode;
mod renderer;
mod noop_command;
mod quit_command;
mod enter_rename_command;
mod enter_navigate_command;
mod enter_setquicknav_command;
mod enter_quicknav_command;
mod rename_entry_command;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, time::Duration};
use tui::{backend::CrosstermBackend, Terminal};

use crate::{kitty_model::KittyModel, quicknav::{persistence::QuickNavPersistence}, error::KittyMuxError};

use self::{model::AppModel, navigatemode::NavigateMode, mode::Mode::{Navigate, Rename, SetQuickNav, QuickNav}, renderer::render, renamemode::RenameMode, setquicknavmode::SetQuickNavMode, quicknavmode::QuickNavMode};

pub fn run(kitty_model: &dyn KittyModel, quick_nav_persistence: &dyn QuickNavPersistence) -> Result<(), KittyMuxError> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app_model =
        AppModel::new(kitty_model.load()?, quick_nav_persistence.load()?, Navigate);

    loop {
        render(&mut terminal, &mut app_model)?;
        if crossterm::event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                let mut cmd = match app_model.mode() {
                    Navigate => NavigateMode::handle_input(&key, app_model, kitty_model, quick_nav_persistence)?,
                    Rename => RenameMode::handle_input(&key, app_model, kitty_model, quick_nav_persistence)?,
                    SetQuickNav => SetQuickNavMode::handle_input(&key, app_model, kitty_model, quick_nav_persistence)?,
                    QuickNav => QuickNavMode::handle_input(&key, app_model, kitty_model, quick_nav_persistence)?
                };

                app_model = cmd.execute(kitty_model, quick_nav_persistence)?.unwrap();
                if app_model.should_quit() {
                    break;
                }
            }
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
