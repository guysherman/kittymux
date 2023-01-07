use std::io::{Stdout, Result};

use crossterm::event::KeyEvent;
use tui::{Terminal, backend::CrosstermBackend, terminal::CompletedFrame};

use super::entry_list::EntryList;

pub trait Mode {
    fn draw<'a>(terminal: &'a mut Terminal<CrosstermBackend<Stdout>>, entry_list: &mut EntryList) -> Result<CompletedFrame<'a>>;
    fn handle_input(&self, event: &KeyEvent, entry_list: &mut EntryList) -> Result<bool>;
}
