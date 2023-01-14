mod model;
mod mode;
mod navigatemode;
mod command;
mod load_command;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, time::Duration};
use tui::{backend::CrosstermBackend, Terminal};

use crate::kitty_model::KittyModel;

use self::{model::AppModel, mode::Mode, navigatemode::NavigateMode};

pub fn run(kitty_model: & dyn KittyModel) -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app_model = AppModel::with_items(kitty_model.load());
    let mode = NavigateMode::new(kitty_model);

    loop {
        NavigateMode::draw(&mut terminal, &mut app_model)?;

        if crossterm::event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                let should_quit = mode.handle_input(&key, &mut app_model)?;
                if should_quit {
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
