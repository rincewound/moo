use crate::mode::{EditorMode, Mode};

#[derive(Default)]
pub struct NormalMode {}

impl EditorMode for NormalMode {
    fn mode_name(&self) -> &'static str {
        "NORMAL"
    }

    fn handle_key_event(
        &self,
        key_event: crossterm::event::KeyEvent,
        app_state: &mut crate::app::ApplicationState,
    ) {
    }

    fn render(&self, frame: &mut ratatui::Frame, app_state: &crate::app::ApplicationState) {}
}
