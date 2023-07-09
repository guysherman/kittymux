use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    terminal::CompletedFrame,
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use unicode_width::UnicodeWidthStr;

use crate::kitty_model::window_list_entry::WindowListEntry;

use super::{
    mode::Mode::{Navigate, QuickNav, Rename, SetQuickNav},
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
                let key = model
                    .quicknavs()
                    .find_entry_by_id(x.id)
                    .map_or(" ".to_string(), |x| x.key.to_owned().to_string());
                let gutter = Span::styled(gutter_text(&key, model.mode()), gutter_style(model.mode()));
                let text = Span::styled(x.text.clone(), default_style());
                
                ListItem::new(Text::from(Spans::from(vec![gutter, text])))
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
                SetQuickNav => Style::default().fg(Color::Red),
                QuickNav => Style::default().fg(Color::Blue),
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
            SetQuickNav => {}
            QuickNav => {}
        }
    })
}

fn gutter_style(mode: super::mode::Mode) -> Style {
    // each arm calls a function that returns a style
    match mode {
        Navigate => default_style(),
        Rename => default_style(),
        SetQuickNav => set_quicknav_style(),
        QuickNav => quicknav_style(),
    }
}

fn default_style() -> Style {
    Style::default().fg(Color::White).bg(Color::Black)
}

fn selected_style() -> Style {
    Style::default()
        .bg(Color::Blue)
        .add_modifier(Modifier::BOLD)
}

fn set_quicknav_style() -> Style {
    Style::default().bg(Color::Yellow).fg(Color::Black)
}

fn quicknav_style() -> Style {
    Style::default().bg(Color::Green).fg(Color::Black)
}

fn gutter_text(key: &String, mode: super::mode::Mode) -> String {
    match mode {
        Navigate => "   ".to_string(),
        Rename => "   ".to_string(), 
        SetQuickNav => format!(" {} ", key),
        QuickNav => format!(" {} ", key),
    }
}
