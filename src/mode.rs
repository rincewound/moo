use crossterm::event::KeyEvent;
use ratatui::Frame;

use crate::app::ApplicationState;

#[derive(Default)]
pub enum Mode {
    #[default]
    Normal,
    Insert,
    Navigate,
    Select,
}

pub trait EditorMode {
    fn mode_name(&self) -> &'static str;
    fn handle_key_event(&mut self, key_event: KeyEvent, app_state: &mut ApplicationState);
    fn render(&self, frame: &mut Frame, app_state: &ApplicationState);
}
