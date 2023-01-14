mod entry_list;
mod mode;
mod navigatemode;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, time::Duration};
use tui::{backend::CrosstermBackend, Terminal};

use crate::entry_list::KittyWindowList;

use self::{entry_list::EntryList, mode::Mode, navigatemode::NavigateMode};

pub fn run(kitty_entry_list: & dyn KittyWindowList) -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut entry_list = EntryList::with_items(kitty_entry_list.load());
    let mode = NavigateMode::new(kitty_entry_list);

    loop {
        NavigateMode::draw(&mut terminal, &mut entry_list)?;

        if crossterm::event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                let should_quit = mode.handle_input(&key, &mut entry_list)?;
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
