use std::io::{Result, Stdout};

use crossterm::event::{KeyCode, KeyEvent};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    terminal::CompletedFrame,
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};

use crate::{
    entry_list::{window_list_entry::WindowListEntry, KittyEntryList}, kitty_connector::KittyConnector, ui::mode::Mode,
};

use super::entry_list::EntryList;

pub struct NavigateMode<'a> {
    kitty_entry_list: &'a KittyEntryList<'a>
}

impl<'a> NavigateMode<'a> {
    pub fn new(kitty_entry_list: &'a KittyEntryList) -> NavigateMode<'a> {
        NavigateMode { kitty_entry_list }
    }
}

impl<'a> Mode for NavigateMode<'a> {
    fn draw<'b>(
        terminal: &'b mut Terminal<CrosstermBackend<Stdout>>,
        entry_list: &mut EntryList,
    ) -> Result<CompletedFrame<'b>> {
        terminal.draw(|f| {
            let list: Vec<ListItem> = entry_list
                .items()
                .iter()
                .map(|x: &WindowListEntry| {
                    ListItem::new(x.text.clone())
                        .style(Style::default().fg(Color::White).bg(Color::Black))
                })
                .collect();

            let list = List::new(list)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(tui::widgets::BorderType::Rounded),
                )
                .highlight_style(
                    Style::default()
                        .bg(Color::Blue)
                        .add_modifier(Modifier::BOLD),
                );

            let panes = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(f.size());
            let block2 = Block::default()
                .title("Block 2")
                .borders(Borders::ALL)
                .border_type(tui::widgets::BorderType::Rounded);

            f.render_stateful_widget(list, panes[0], &mut entry_list.state());
            f.render_widget(block2, panes[1]);
        })
    }

    fn handle_input(&self, event: &KeyEvent, entry_list: &mut EntryList) -> Result<bool> {
        match event.code {
            KeyCode::Char('q') => Ok(true),
            KeyCode::Char('j') => {
                entry_list.select_next();
                Ok(false)
            }
            KeyCode::Char('k') => {
                entry_list.select_prev();
                Ok(false)
            }
            KeyCode::Enter => {
                entry_list.selected().map(|selected_item| {
                    self.kitty_entry_list.focus_entry(selected_item);
                });
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}
