use crossterm::event::{KeyCode, KeyModifiers};

use crate::{
    mode::EditorMode,
    modeutil::{self, rotate_buffer},
};

#[derive(Default)]
pub struct NavigationMode {}

impl EditorMode for NavigationMode {
    fn mode_name(&self) -> &'static str {
        "NAV"
    }

    fn handle_key_event(
        &mut self,
        key_event: crossterm::event::KeyEvent,
        app_state: &mut crate::app::ApplicationState,
    ) {
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
            KeyCode::Char(c) => match c {
                'd' => buffer.skip_word_backward(),
                'k' => buffer.skip_word_forward(),
                'u' => buffer.move_cursor_up(),
                'n' => buffer.move_cursor_down(),
                's' => buffer.goto_line_start(),
                'l' => buffer.goto_line_end(),
                _ => (),
            },
            _ => (),
        }
    }

    fn render(&self, frame: &mut ratatui::Frame, app_state: &crate::app::ApplicationState) {
        modeutil::render(self.mode_name(), frame, app_state);
    }
}
