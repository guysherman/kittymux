mod command;
mod mode;
mod model;
mod renderer;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, time::Duration};
use tui::{backend::CrosstermBackend, Terminal};

use crate::{
    error::KittyMuxError, kitty_model::KittyModel, quicknav::persistence::QuickNavPersistence,
};

use self::mode::{
    navigatemode::NavigateMode,
    quicknavmode::QuickNavMode,
    renamemode::RenameMode,
    setquicknavmode::SetQuickNavMode,
    Mode::{Navigate, QuickNav, Rename, SetQuickNav},
};
use self::{model::AppModel, renderer::render};

pub fn run(
    kitty_model: &dyn KittyModel,
    quick_nav_persistence: &dyn QuickNavPersistence,
) -> Result<(), KittyMuxError> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app_model = AppModel::new(kitty_model.load()?, quick_nav_persistence.load()?, Navigate);

    loop {
        render(&mut terminal, &mut app_model)?;
        if crossterm::event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                let cmds = match app_model.mode() {
                    Navigate => NavigateMode::handle_input(&key)?,
                    Rename => RenameMode::handle_input(&key)?,
                    SetQuickNav => SetQuickNavMode::handle_input(&key)?,
                    QuickNav => QuickNavMode::handle_input(&key)?,
                };

                let mut it = cmds.iter();
                while let Some(cmd) = it.next() {
                    app_model = cmd.execute(kitty_model, quick_nav_persistence, app_model)?;
                }

                if app_model.should_quit() {
                    break;
                }
            }
        }
    }

    // restore terminal
    quick_nav_persistence.save(app_model.quicknavs())?;
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
