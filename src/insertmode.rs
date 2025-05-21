// use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout},
    style::Stylize,
    text::Line,
    widgets,
};

use crate::mode::EditorMode;

#[derive(Default)]
pub struct InsertMode;

impl EditorMode for InsertMode {
    fn mode_name(&self) -> &'static str {
        "INSERT"
    }

    fn handle_key_event(
        &self,
        key_event: crossterm::event::KeyEvent,
        app_state: &mut crate::app::ApplicationState,
    ) {
        // Insert mode will just append letters to the current line:
        let buffer = &mut app_state.buffers[app_state.current_buffer];
        if key_event.modifiers.contains(KeyModifiers::CONTROL) {
            return;
        }
        match key_event.code {
            KeyCode::Char(c) => {
                let current_line = buffer.buffer.line_at_mut(buffer.cursor_line);
                if let Some(line) = current_line {
                    line.insert(buffer.cursor_char, c);
                    buffer.cursor_char += 1;
                }
                buffer.modified = true;
            }
            KeyCode::Backspace => {
                let current_line = buffer.buffer.line_at_mut(buffer.cursor_line);
                if let Some(line) = current_line {
                    if buffer.cursor_char > 0 {
                        line.remove(buffer.cursor_char - 1);
                        buffer.cursor_char -= 1;
                    } else {
                        if buffer.buffer.num_lines() >= 1 {
                            buffer.buffer.remove_line_at(buffer.cursor_line);
                            buffer.cursor_line = buffer.cursor_line.saturating_sub(1);
                            if let Some(line) = buffer.buffer.line_at(buffer.cursor_line) {
                                buffer.cursor_char = line.len();
                            } else {
                                buffer.cursor_line = 0;
                            }
                        }
                    }
                }
                buffer.modified = true;
            }
            KeyCode::Enter => {
                buffer
                    .buffer
                    .break_line_at(buffer.cursor_line, buffer.cursor_char);
                buffer.cursor_line += 1;
                buffer.cursor_char = 0;
            }
            KeyCode::Up => {
                if buffer.cursor_line > 0 {
                    buffer.cursor_line -= 1;
                    buffer.cursor_char = buffer.buffer.line_at(buffer.cursor_line).unwrap().len();
                }
            }
            KeyCode::Down => {
                if buffer.cursor_line < buffer.buffer.num_lines() - 1 {
                    buffer.cursor_line += 1;
                    buffer.cursor_char = 0;
                }
            }
            KeyCode::Left => {
                if buffer.cursor_char > 0 {
                    buffer.cursor_char -= 1;
                }
            }
            KeyCode::Right => {
                if buffer.cursor_char < buffer.buffer.line_at(buffer.cursor_line).unwrap().len() {
                    buffer.cursor_char += 1;
                }
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
        let output_string = format!("{}{}", buffer.name, if buffer.modified { "●" } else { "" });

        let header_block = widgets::Block::new().borders(widgets::Borders::all());

        frame.render_widget(
            widgets::Paragraph::new(output_string).block(header_block),
            layout[0],
        );

        for (id, line) in buffer
            .buffer
            .lines
            .iter()
            .skip(buffer.scroll_offset)
            .enumerate()
        {
            let len = line.len() as u16;
            frame.render_widget(
                ratatui::widgets::Paragraph::new(line.clone())
                    .alignment(ratatui::layout::Alignment::Left),
                ratatui::layout::Rect::new(0, 3 + id as u16, len, 1),
            );

            // render cursor:
            let effective_line = id + buffer.scroll_offset;
            if buffer.cursor_line == effective_line {
                // get character under cursor
                let char = line.chars().nth(buffer.cursor_char);
                let cursor_char = if let Some(c) = char { c } else { '_' };

                let mut cursor = cursor_char.to_string().rapid_blink();
                if char.is_some() {
                    cursor = cursor.underlined();
                }

                let the_cusor = Line::from(vec![cursor]);

                frame.render_widget(
                    ratatui::widgets::Paragraph::new(the_cusor)
                        .alignment(ratatui::layout::Alignment::Left),
                    ratatui::layout::Rect::new(
                        buffer.cursor_char as u16,
                        (buffer.cursor_line - buffer.scroll_offset + 3) as u16,
                        1,
                        1,
                    ),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::app::BufferEntry;

    use super::*;

    fn make_default_app_state() -> crate::app::ApplicationState {
        let mut app_state = crate::app::ApplicationState::default();
        app_state.buffers.push(BufferEntry::default());
        app_state
    }

    #[test]
    pub fn inject_char_modifies_buffer() {
        let mut app_state = make_default_app_state();
        let insertmode = InsertMode::default();
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
    pub fn inject_backspace_modifies_buffer() {
        let mut app_state = make_default_app_state();
        let insertmode = InsertMode::default();

        app_state.buffers[0]
            .buffer
            .line_at_mut(0)
            .unwrap()
            .push('a');
        insertmode.handle_key_event(
            crossterm::event::KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE),
            &mut app_state,
        );

        assert_eq!(app_state.buffers[0].buffer.line_at(0), None);
    }
    #[test]
    pub fn inject_enter_modifies_buffer() {
        let mut app_state = make_default_app_state();
        let insertmode = InsertMode::default();

        app_state.buffers[0].buffer.lines.push("abc".to_string());
        app_state.buffers[0].cursor_line = 1;
        app_state.buffers[0].cursor_char = 2;
        insertmode.handle_key_event(
            crossterm::event::KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
            &mut app_state,
        );

        assert_eq!(
            app_state.buffers[0].buffer.line_at(2),
            Some(&"c".to_string())
        );
    }
}
