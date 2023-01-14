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
    kitty_model::{window_list_entry::WindowListEntry, KittyModel}, ui::mode::Mode,
};

use super::model::AppModel;

pub struct NavigateMode<'a> {
    kitty_model: &'a dyn KittyModel
}

impl<'a> NavigateMode<'a> {
    pub fn new(kitty_window_list: &'a dyn KittyModel) -> NavigateMode<'a> {
        NavigateMode { kitty_model: kitty_window_list }
    }
}

impl<'a> Mode for NavigateMode<'a> {
    fn draw<'b>(
        terminal: &'b mut Terminal<CrosstermBackend<Stdout>>,
        model: &mut AppModel,
    ) -> Result<CompletedFrame<'b>> {
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
            let block2 = Block::default()
                .title("Block 2")
                .borders(Borders::ALL)
                .border_type(tui::widgets::BorderType::Rounded);

            f.render_stateful_widget(list, panes[0], &mut model.state());
            f.render_widget(block2, panes[1]);
        })
    }

    fn handle_input(&self, event: &KeyEvent, model: &mut AppModel) -> Result<bool> {
        match event.code {
            KeyCode::Char('q') => Ok(true),
            KeyCode::Char('j') => {
                model.select_next();
                Ok(false)
            }
            KeyCode::Char('k') => {
                model.select_prev();
                Ok(false)
            }
            KeyCode::Char('J') => {
                model.select_next_tab();
                Ok(false)
            }
            KeyCode::Char('K') => {
                model.select_prev_tab();
                Ok(false)
            }
            KeyCode::Char('x') => {
                model.selected().map(|entry| self.kitty_model.close_entry(entry));
                Ok(false)
            }
            KeyCode::Enter => {
                model.selected().map(|selected_item| {
                    self.kitty_model.focus_entry(selected_item);
                });
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyModifiers, KeyEventState};

    use crate::kitty_model::{entry_type::EntryType, KittyModel};

    use super::*;

    fn basic_windows() -> Vec<WindowListEntry> {
        vec![
            WindowListEntry {
                id: 1,
                text: "kitty: 1".to_string(),
                entry_type: EntryType::OsWindow,
                pid: 0,
                cwd: "".to_string(),
                title: "kitty: 1".to_string(),
                is_focused: true,
                tab_is_focused: true,
                os_window_is_focused: true,
                tab_id: 0,
            },
            WindowListEntry {
                id: 1,
                text: "my tab".to_string(),
                title: "my tab".to_string(),
                entry_type: EntryType::Tab,
                pid: 0,
                cwd: "".to_string(),
                is_focused: true,
                tab_is_focused: true,
                os_window_is_focused: true,
                tab_id: 1,
            },
            WindowListEntry {
                id: 1,
                tab_id: 1,
                pid: 1,
                cwd: "/foo".to_string(),
                text: "1".to_string(),
                title: "1".to_string(),
                entry_type: EntryType::Window,
                is_focused: true,
                tab_is_focused: true,
                os_window_is_focused: true,
            },
            WindowListEntry {
                id: 2,
                text: "my tab 2".to_string(),
                title: "my tab 2".to_string(),
                entry_type: EntryType::Tab,
                pid: 0,
                cwd: "".to_string(),
                is_focused: false,
                tab_is_focused: false,
                os_window_is_focused: true,
                tab_id: 2,
            },
            WindowListEntry {
                id: 2,
                tab_id: 2,
                pid: 2,
                cwd: "/foo".to_string(),
                text: "2".to_string(),
                title: "2".to_string(),
                entry_type: EntryType::Window,
                is_focused: false,
                tab_is_focused: false,
                os_window_is_focused: true,
            },
            WindowListEntry {
                id: 3,
                text: "my tab 3".to_string(),
                title: "my tab 3".to_string(),
                entry_type: EntryType::Tab,
                pid: 0,
                cwd: "".to_string(),
                is_focused: false,
                tab_is_focused: false,
                os_window_is_focused: true,
                tab_id: 3,
            },
            WindowListEntry {
                id: 3,
                tab_id: 3,
                pid: 3,
                cwd: "/foo".to_string(),
                text: "3".to_string(),
                title: "3".to_string(),
                entry_type: EntryType::Window,
                is_focused: false,
                tab_is_focused: false,
                os_window_is_focused: true,
            }
        ]
    }

    #[test]
    fn given_0_selected_when_j_pressed_1_selected() {
        let window_list = KittyModel::new();
        let mode = NavigateMode::new(&window_list);
        let mut model = AppModel::with_items(basic_windows());
        let event = KeyEvent::new_with_kind_and_state(KeyCode::Char('j'), KeyModifiers::empty(), crossterm::event::KeyEventKind::Press, KeyEventState::NONE);
        mode.handle_input(&event, &mut model).unwrap();
        let expected = WindowListEntry {
            id: 1,
            text: "my tab".to_string(),
            title: "my tab".to_string(),
            entry_type: EntryType::Tab,
            pid: 0,
            cwd: "".to_string(),
            is_focused: true,
            tab_is_focused: true,
            os_window_is_focused: true,
            tab_id: 1,
        };

        assert_eq!(*model.selected().unwrap(), expected);
    }

    #[test]
    fn given_0_selected_when_shift_j_pressed_1_selected() {
        let window_list = KittyModel::new();
        let mode = NavigateMode::new(&window_list);
        let mut model = AppModel::with_items(basic_windows());
        let event = KeyEvent::new_with_kind_and_state(KeyCode::Char('J'), KeyModifiers::SHIFT, crossterm::event::KeyEventKind::Press, KeyEventState::NONE);
        mode.handle_input(&event, &mut model).unwrap();
        let expected = WindowListEntry {
            id: 1,
            text: "my tab".to_string(),
            title: "my tab".to_string(),
            entry_type: EntryType::Tab,
            pid: 0,
            cwd: "".to_string(),
            is_focused: true,
            tab_is_focused: true,
            os_window_is_focused: true,
            tab_id: 1,
        };

        assert_eq!(*model.selected().unwrap(), expected);

    }

    #[test]
    fn given_1_selected_when_shift_j_pressed_3_selected() {
        let window_list = KittyModel::new();
        let mode = NavigateMode::new(&window_list);

        let mut model = AppModel::with_items(basic_windows());
        model.state().select(Some(1));

        let event = KeyEvent::new_with_kind_and_state(KeyCode::Char('J'), KeyModifiers::SHIFT, crossterm::event::KeyEventKind::Press, KeyEventState::NONE);
        let expected = WindowListEntry {
            id: 2,
            text: "my tab 2".to_string(),
            title: "my tab 2".to_string(),
            entry_type: EntryType::Tab,
            pid: 0,
            cwd: "".to_string(),
            is_focused: false,
            tab_is_focused: false,
            os_window_is_focused: true,
            tab_id: 2,
        };

        mode.handle_input(&event, &mut model).unwrap();
        assert_eq!(*model.selected().unwrap(), expected);
    }

    #[test]
    fn given_3_selected_when_shift_k_pressed_1_selected() {
        let window_list = KittyModel::new();
        let mode = NavigateMode::new(&window_list);

        let mut model = AppModel::with_items(basic_windows());
        model.state().select(Some(3));

        let event = KeyEvent::new_with_kind_and_state(KeyCode::Char('K'), KeyModifiers::SHIFT, crossterm::event::KeyEventKind::Press, KeyEventState::NONE);
        let expected = WindowListEntry {
            id: 1,
            text: "my tab".to_string(),
            title: "my tab".to_string(),
            entry_type: EntryType::Tab,
            pid: 0,
            cwd: "".to_string(),
            is_focused: true,
            tab_is_focused: true,
            os_window_is_focused: true,
            tab_id: 1,
        };

        mode.handle_input(&event, &mut model).unwrap();
        assert_eq!(*model.selected().unwrap(), expected);
    }

    #[test]
    fn given_1_selected_when_x_pressed_then_close_entry_called() {
        let mut mock_window_list = KittyModel::new();
        mock_window_list.expect_close_entry()
            .withf(|_entry: &WindowListEntry| true)
            .times(1)
            .returning(|_| ());

        let mode = NavigateMode::new(&mock_window_list);

        let mut model = AppModel::with_items(basic_windows());
        model.state().select(Some(1));

        let event = KeyEvent::new_with_kind_and_state(KeyCode::Char('x'), KeyModifiers::empty(), crossterm::event::KeyEventKind::Press, KeyEventState::NONE);
        mode.handle_input(&event, &mut model);
    }

}
