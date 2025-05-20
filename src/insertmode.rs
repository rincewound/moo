// use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{style::Stylize, text::Line};

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
            }
            KeyCode::Backspace => {
                let current_line = buffer.buffer.line_at_mut(buffer.cursor_line);
                if let Some(line) = current_line {
                    if buffer.cursor_char > 0 {
                        line.remove(buffer.cursor_char - 1);
                        buffer.cursor_char -= 1;
                    } else {
                        if buffer.buffer.num_lines() > 1 {
                            buffer.buffer.remove_line_at(buffer.cursor_line);
                            buffer.cursor_line -= 1;
                            buffer.cursor_char =
                                buffer.buffer.line_at(buffer.cursor_line).unwrap().len();
                        }
                    }
                }
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

        let buffer = &app_state.buffers[app_state.current_buffer];

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
                ratatui::layout::Rect::new(0, id as u16, len, 1),
            );

            // render cursor:
            let effective_line = id + buffer.scroll_offset;
            if buffer.cursor_line == effective_line {
                // get character under cursor
                let char = line.chars().nth(buffer.cursor_char);
                let cursor_char = if let Some(c) = char { c } else { '_' };

                let mut cursor = cursor_char.to_string().slow_blink();
                if char.is_some() {
                    cursor = cursor.on_dark_gray();
                }

                let the_cusor = Line::from(vec![cursor]);

                frame.render_widget(
                    ratatui::widgets::Paragraph::new(the_cusor)
                        .alignment(ratatui::layout::Alignment::Left),
                    ratatui::layout::Rect::new(
                        buffer.cursor_char as u16,
                        (buffer.cursor_line - buffer.scroll_offset) as u16,
                        1,
                        1,
                    ),
                );
            }
        }
    }
}
