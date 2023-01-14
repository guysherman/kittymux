use std::io::{Stdout, Result};

use crossterm::event::KeyEvent;
use tui::{Terminal, backend::CrosstermBackend, terminal::CompletedFrame};

use super::model::AppModel;

pub trait Mode {
    fn draw<'a>(terminal: &'a mut Terminal<CrosstermBackend<Stdout>>, entry_list: &mut AppModel) -> Result<CompletedFrame<'a>>;
    fn handle_input(&self, event: &KeyEvent, entry_list: &mut AppModel) -> Result<bool>;
}
