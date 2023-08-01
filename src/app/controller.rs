use crate::app::history::History;
use crate::app::state::State;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

pub fn poll_next_event<'a>(state: State<'a>, history: &'a History) -> Result<Option<State<'a>>> {
    match event::read()? {
        Event::Key(event) => match event {
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            } | KeyEvent {
                code: KeyCode::Char('d'),
                modifiers: KeyModifiers::CONTROL,
            } | KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: _,
            } => Ok(None),
            KeyEvent {
                code: KeyCode::Left,
                modifiers: _,
            } | KeyEvent {
                code: KeyCode::Char('h'),
                modifiers: _,
            } => Ok(Some(state.backward_commit(history))),
            KeyEvent {
                code: KeyCode::Right,
                modifiers: _,
            } | KeyEvent {
                code: KeyCode::Char('l'),
                modifiers: _,
            } => Ok(Some(state.forward_commit(history))),
            KeyEvent {
                code: KeyCode::Up,
                modifiers: _,
            } | KeyEvent {
                code: KeyCode::Char('k'),
                modifiers: _,
            } => Ok(Some(state.scroll_line_up())),
            KeyEvent {
                code: KeyCode::Down,
                modifiers: _,
            } | KeyEvent {
                code: KeyCode::Char('j'),
                modifiers: _,
            } => Ok(Some(state.scroll_line_down())),
            KeyEvent {
                code: KeyCode::PageUp,
                modifiers: _,
            } | KeyEvent {
                code: KeyCode::Char('u'),
                modifiers: _,
            } => Ok(Some(state.scroll_page_up())),
            KeyEvent {
                code: KeyCode::PageDown,
                modifiers: _,
            } | KeyEvent {
                code: KeyCode::Char('d'),
                modifiers: _,
            } => Ok(Some(state.scroll_page_down())),
            KeyEvent {
                code: KeyCode::Home,
                modifiers: _,
            } | KeyEvent {
                code: KeyCode::Char('g'),
                modifiers: _,
            } => Ok(Some(state.scroll_to_top())),
            KeyEvent {
                code: KeyCode::End,
                modifiers: _,
            } | KeyEvent {
                code: KeyCode::Char('G'),
                modifiers: _,
            } => Ok(Some(state.scroll_to_bottom())),
            _ => Ok(Some(state)),
        },
        Event::Resize(_width, height) => {
            Ok(Some(state.update_terminal_height(usize::from(height))))
        }
        _ => Ok(Some(state)),
    }
}
