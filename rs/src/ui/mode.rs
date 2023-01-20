use std::{io::{Stdout}, error::Error};

use crossterm::event::KeyEvent;
use tui::{Terminal, backend::CrosstermBackend, terminal::CompletedFrame};

use super::{model::AppModel, command::Command};

pub trait Mode {
    fn draw<'a>(terminal: &'a mut Terminal<CrosstermBackend<Stdout>>, entry_list: &mut AppModel) -> std::io::Result<CompletedFrame<'a>>;
    fn handle_input(&self, event: &KeyEvent, app_model: AppModel) -> Result<Box<dyn Command>, Box<dyn Error>>;
}
