use crossterm::event::KeyCode;

use crate::{mode::EditorMode, modeutil};

#[derive(Default)]
pub struct SelectMode {}

impl EditorMode for SelectMode {
    fn handle_key_event(
        &mut self,
        key: crossterm::event::KeyEvent,
        app_state: &mut crate::app::ApplicationState,
    ) {
        let buffer = &mut app_state.buffers[app_state.current_buffer];

        match key.code {
            KeyCode::Char('q') => {
                buffer.clear_selection();
            }
            KeyCode::Char(c) => match c {
                's' => {
                    buffer.goto_line_start();
                    buffer.extend_selection_to_cursor();
                }
                'd' => {
                    buffer.skip_word_backward();
                    buffer.extend_selection_to_cursor();
                }
                'f' => {
                    buffer.move_cursor_left();
                    buffer.extend_selection_to_cursor();
                }
                'k' => {
                    buffer.skip_word_forward();
                    buffer.extend_selection_to_cursor();
                }
                'j' => {
                    buffer.move_cursor_right();
                    buffer.extend_selection_to_cursor();
                }
                'l' => {
                    buffer.goto_line_end();
                    buffer.extend_selection_to_cursor();
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn render(&self, frame: &mut ratatui::Frame, app_state: &crate::app::ApplicationState) {
        modeutil::render(self.mode_name(), frame, app_state);
    }

    fn mode_name(&self) -> &'static str {
        "SELECT"
    }
}
