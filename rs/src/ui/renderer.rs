use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    terminal::CompletedFrame,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use unicode_width::UnicodeWidthStr;

use crate::kitty_model::window_list_entry::WindowListEntry;

use super::{
    mode::Mode::{Navigate, Rename},
    model::AppModel,
};
pub fn render<'b>(
    terminal: &'b mut Terminal<CrosstermBackend<Stdout>>,
    model: &mut AppModel,
) -> std::io::Result<CompletedFrame<'b>> {
    terminal.draw(|f| {
        let list: Vec<ListItem> = model
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

        f.render_stateful_widget(list, panes[0], &mut model.state());

        let input_text = model.text_input.as_ref();
        let input = Paragraph::new(input_text)
            .style(match model.mode() {
                Navigate => Style::default(),
                Rename => Style::default().fg(Color::Yellow),
                super::mode::Mode::SetQuickNav => Style::default(),
            })
        .block(Block::default().borders(Borders::ALL));
        f.render_widget(input, panes[1]);
        match model.mode() {
            Navigate =>
                // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
                {}

            Rename => {
                // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
                f.set_cursor(
                    // Put cursor past the end of the input text
                    panes[1].x + input_text.width() as u16 + 1,
                    // Move one line down, from the border to the input line
                    panes[1].y + 1,
                )
            }
            super::mode::Mode::SetQuickNav => {}
        }
    })
}
