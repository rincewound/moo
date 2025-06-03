// use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout},
    style::Stylize,
    text::Line,
};

use crate::{
    mode::EditorMode,
    modeutil::{render_mode_header, rotate_buffer},
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
                buffer.remove_character();
            }
            KeyCode::Enter => {
                buffer.new_line();
            }
            KeyCode::Up => {
                buffer.move_cursor_up();
            }
            KeyCode::Down => {
                buffer.move_cursor_down();
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
        // ToDo: This should be generalized a bit for all modes!
        let layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(frame.area());

        let buffer = &app_state.buffers[app_state.current_buffer];

        // show buffer name + modified flag:
        render_mode_header(frame, layout[0], self.mode_name(), app_state);

        for (id, line) in buffer
            .buffer
            .lines
            .iter()
            .skip(buffer.scroll_offset)
            .enumerate()
        {
            let line_width = if line.len() < frame.area().width as usize - 1 {
                line.len() as u16
            } else {
                frame.area().width as u16 - 1
            };

            frame.render_widget(
                ratatui::widgets::Paragraph::new(line.clone())
                    .alignment(ratatui::layout::Alignment::Left),
                ratatui::layout::Rect::new(0, 3 + id as u16, line_width, 1),
            );

            // render cursor:
            let effective_line = id + buffer.scroll_offset;
            if buffer.cursor_line == effective_line {
                // get character under cursor
                let char = line.chars().nth(buffer.cursor_byte_position);
                let cursor_char = if let Some(c) = char { c } else { '_' };

                let mut cursor = cursor_char.to_string().rapid_blink();
                if char.is_some() {
                    cursor = cursor.underlined();
                }

                let the_cusor = Line::from(vec![cursor]);

                // ToDo This crashes, when the cursor ends up outside of the visible area!
                if buffer.cursor_render_position < frame.area().width as usize {
                    frame.render_widget(
                        ratatui::widgets::Paragraph::new(the_cusor)
                            .alignment(ratatui::layout::Alignment::Left),
                        ratatui::layout::Rect::new(
                            buffer.cursor_render_position as u16,
                            (buffer.cursor_line - buffer.scroll_offset + 3) as u16,
                            1,
                            1,
                        ),
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::bufferentry::BufferEntry;

    use super::*;

    fn make_default_app_state() -> crate::app::ApplicationState {
        let mut app_state = crate::app::ApplicationState::default();
        app_state.buffers.push(BufferEntry::default());
        app_state
    }

    #[test]
    pub fn inject_char_modifies_buffer() {
        let mut app_state = make_default_app_state();
        let mut insertmode = InsertMode::default();
        insertmode.handle_key_event(
            crossterm::event::KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE),
            &mut app_state,
        );

        assert_eq!(
            app_state.buffers[0].buffer.line_at(0),
            Some(&"a".to_string())
        );
    }

    #[test]
    pub fn inject_enter_modifies_buffer() {
        let mut app_state = make_default_app_state();
        let mut insertmode = InsertMode::default();

        app_state.buffers[0].buffer.lines.push("abc".to_string());
        app_state.buffers[0].cursor_line = 1;
        app_state.buffers[0].cursor_byte_position = 2;
        insertmode.handle_key_event(
            crossterm::event::KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
            &mut app_state,
        );

        assert_eq!(
            app_state.buffers[0].buffer.line_at(2),
            Some(&"c".to_string())
        );
    }

    #[test]
    pub fn inject_diacritic_sets_renderpos_correctly() {
        let mut app_state = make_default_app_state();
        let mut insertmode = InsertMode::default();

        app_state.buffers[0].buffer.lines.push("abc".to_string());
        app_state.buffers[0].cursor_line = 1;
        app_state.buffers[0].cursor_byte_position = 2;
        app_state.buffers[0].cursor_render_position = 2;
        insertmode.handle_key_event(
            crossterm::event::KeyEvent::new(KeyCode::Char('ä'), KeyModifiers::NONE),
            &mut app_state,
        );

        assert_eq!(app_state.buffers[0].cursor_byte_position, 4);
        assert_eq!(app_state.buffers[0].cursor_render_position, 3);
    }

    #[test]
    pub fn remove_diacritic_sets_renderpos_correctly() {
        let mut app_state = make_default_app_state();
        let mut insertmode = InsertMode::default();

        app_state.buffers[0].buffer.lines.push("abcÖ".to_string());
        app_state.buffers[0].cursor_line = 1;
        app_state.buffers[0].cursor_byte_position = 5;
        app_state.buffers[0].cursor_render_position = 4;
        insertmode.handle_key_event(
            crossterm::event::KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE),
            &mut app_state,
        );

        assert_eq!(app_state.buffers[0].cursor_byte_position, 3);
        assert_eq!(app_state.buffers[0].cursor_render_position, 3);
        assert_eq!(app_state.buffers[0].buffer.lines[1], "abc");
    }

    #[test]
    pub fn move_cursor_to_line_with_diacritic_sets_position_correctly() {
        let mut app_state = make_default_app_state();
        let mut insertmode = InsertMode::default();

        app_state.buffers[0].buffer.lines.push("abcÖde".to_string());
        app_state.buffers[0].buffer.lines.push("foobar".to_string());
        app_state.buffers[0].cursor_line = 2;
        app_state.buffers[0].cursor_byte_position = 4;
        app_state.buffers[0].cursor_render_position = 4;
        insertmode.handle_key_event(
            crossterm::event::KeyEvent::new(KeyCode::Up, KeyModifiers::NONE),
            &mut app_state,
        );

        assert_eq!(app_state.buffers[0].cursor_line, 1);
        assert_eq!(app_state.buffers[0].cursor_byte_position, 7);
        assert_eq!(app_state.buffers[0].cursor_render_position, 4);
    }
}
