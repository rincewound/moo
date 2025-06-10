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
        if key_event.modifiers.contains(KeyModifiers::CONTROL) {
            match key_event.code {
                KeyCode::Left => rotate_buffer(app_state, -1),
                KeyCode::Right => rotate_buffer(app_state, 1),

                _ => (),
            }
        }

        let buffer = &mut app_state.buffers[app_state.current_buffer];
        match key_event.code {
            KeyCode::Char(c) => match c {
                's' => buffer.goto_line_start(),
                'd' => buffer.skip_word_backward(),
                'f' => buffer.move_cursor_left(),
                'k' => buffer.skip_word_forward(),
                'j' => buffer.move_cursor_right(),
                'l' => buffer.goto_line_end(),

                'v' => buffer.move_cursor_up(app_state.window_size.1),
                'b' => buffer.move_cursor_down(app_state.window_size.1),
                'c' => buffer.move_cursor_page_up(app_state.window_size.1),
                'n' => buffer.move_cursor_page_down(app_state.window_size.1),
                _ => (),
            },
            _ => (),
        }
    }

    fn render(&self, frame: &mut ratatui::Frame, app_state: &crate::app::ApplicationState) {
        modeutil::render(self.mode_name(), frame, app_state);
    }
}
