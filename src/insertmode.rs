// use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{KeyCode, KeyModifiers};

use crate::{
    mode::EditorMode,
    modeutil::{self, rotate_buffer},
};

#[derive(Default)]
pub struct InsertMode;

impl EditorMode for InsertMode {
    fn mode_name(&self) -> &'static str {
        "INSERT"
    }

    fn handle_key_event(
        &mut self,
        key_event: crossterm::event::KeyEvent,
        app_state: &mut crate::app::ApplicationState,
    ) {
        // Insert mode will just append letters to the current line:
        let buffer = &mut app_state.buffers[app_state.current_buffer];
        if key_event.modifiers.contains(KeyModifiers::CONTROL) {
            match key_event.code {
                KeyCode::Left => rotate_buffer(app_state, -1),
                KeyCode::Right => rotate_buffer(app_state, 1),
                _ => (),
            }

            return;
        }
        match key_event.code {
            KeyCode::Char(c) => {
                buffer.add_character(c);
            }
            KeyCode::Backspace => {
                buffer.remove_character(app_state.window_size.1);
            }
            KeyCode::Enter => {
                buffer.new_line(app_state.window_size.1);
            }
            KeyCode::Up => {
                buffer.move_cursor_up(app_state.window_size.1);
            }
            KeyCode::Down => {
                buffer.move_cursor_down(app_state.window_size.1);
            }
            KeyCode::Left => {
                buffer.move_cursor_left();
            }
            KeyCode::Right => {
                buffer.move_cursor_right();
            }
            _ => (),
        }
    }

    fn render(&self, frame: &mut ratatui::Frame, app_state: &crate::app::ApplicationState) {
        modeutil::render(self.mode_name(), frame, app_state);
    }
}

#[cfg(test)]
mod tests {

    // use crate::bufferentry::BufferEntry;

    // use super::*;

    // fn make_default_app_state() -> crate::app::ApplicationState {
    //     let mut app_state = crate::app::ApplicationState::default();
    //     app_state.buffers.push(BufferEntry::default());
    //     app_state
    // }

    // #[test]
    // pub fn inject_char_modifies_buffer() {
    //     let mut app_state = make_default_app_state();
    //     let mut insertmode = InsertMode::default();
    //     insertmode.handle_key_event(
    //         crossterm::event::KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE),
    //         &mut app_state,
    //     );

    //     assert_eq!(
    //         app_state.buffers[0].buffer.line_at(0),
    //         Some(&"a".to_string())
    //     );
    // }

    // #[test]
    // pub fn inject_enter_modifies_buffer() {
    //     let mut app_state = make_default_app_state();
    //     let mut insertmode = InsertMode::default();

    //     app_state.buffers[0].buffer.lines.push("abc".to_string());
    //     app_state.buffers[0].cursor_line = 1;
    //     app_state.buffers[0].cursor_byte_position = 2;
    //     insertmode.handle_key_event(
    //         crossterm::event::KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
    //         &mut app_state,
    //     );

    //     assert_eq!(
    //         app_state.buffers[0].buffer.line_at(2),
    //         Some(&"c".to_string())
    //     );
    // }

    // #[test]
    // pub fn inject_diacritic_sets_renderpos_correctly() {
    //     let mut app_state = make_default_app_state();
    //     let mut insertmode = InsertMode::default();

    //     app_state.buffers[0].buffer.lines.push("abc".to_string());
    //     app_state.buffers[0].cursor_line = 1;
    //     app_state.buffers[0].cursor_byte_position = 2;
    //     app_state.buffers[0].cursor_render_position = 2;
    //     insertmode.handle_key_event(
    //         crossterm::event::KeyEvent::new(KeyCode::Char('ä'), KeyModifiers::NONE),
    //         &mut app_state,
    //     );

    //     assert_eq!(app_state.buffers[0].cursor_byte_position, 4);
    //     assert_eq!(app_state.buffers[0].cursor_render_position, 3);
    // }

    // #[test]
    // pub fn remove_diacritic_sets_renderpos_correctly() {
    //     let mut app_state = make_default_app_state();
    //     let mut insertmode = InsertMode::default();

    //     app_state.buffers[0].buffer.lines.push("abcÖ".to_string());
    //     app_state.buffers[0].cursor_line = 1;
    //     app_state.buffers[0].cursor_byte_position = 5;
    //     app_state.buffers[0].cursor_render_position = 4;
    //     insertmode.handle_key_event(
    //         crossterm::event::KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE),
    //         &mut app_state,
    //     );

    //     assert_eq!(app_state.buffers[0].cursor_byte_position, 3);
    //     assert_eq!(app_state.buffers[0].cursor_render_position, 3);
    //     assert_eq!(app_state.buffers[0].buffer.lines[1], "abc");
    // }

    // #[test]
    // pub fn move_cursor_to_line_with_diacritic_sets_position_correctly() {
    //     let mut app_state = make_default_app_state();
    //     let mut insertmode = InsertMode::default();

    //     app_state.buffers[0].buffer.lines.push("abcÖde".to_string());
    //     app_state.buffers[0].buffer.lines.push("foobar".to_string());
    //     app_state.buffers[0].cursor_line = 2;
    //     app_state.buffers[0].cursor_byte_position = 4;
    //     app_state.buffers[0].cursor_render_position = 4;
    //     insertmode.handle_key_event(
    //         crossterm::event::KeyEvent::new(KeyCode::Up, KeyModifiers::NONE),
    //         &mut app_state,
    //     );

    //     assert_eq!(app_state.buffers[0].cursor_line, 1);
    //     assert_eq!(app_state.buffers[0].cursor_byte_position, 7);
    //     assert_eq!(app_state.buffers[0].cursor_render_position, 4);
    // }
}
