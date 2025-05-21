use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Flex, Layout},
    widgets::Block,
};

use crate::{app, mode::EditorMode, modeutil::render_mode_header};

#[derive(Default)]
pub struct NormalMode {}

impl NormalMode {
    fn render_logo(&self, frame: &mut ratatui::Frame, dest: ratatui::layout::Rect) {
        let logo = ratatui::widgets::List::new(vec![
            "   ____ ___  ____  ____ ",
            "  / __ `__ \\/ __ \\/ __ \\",
            " / / / / / / /_/ / /_/ /",
            "/_/ /_/ /_/\\____/\\____/",
            "      (c) 2025",
        ]);

        let [area] = Layout::horizontal([Constraint::Length(24)])
            .flex(Flex::Center)
            .areas(dest);

        frame.render_widget(logo, area);
    }

    fn render_menu(&self, frame: &mut ratatui::Frame, dest: ratatui::layout::Rect) {
        // Show a box with all keys
        let lst = ratatui::widgets::List::new(vec![
            "<- ->: Change Buffer",
            "w: Write Buffer",
            "n: New Buffer",
            "c: Close Buffer",
            "a: Name Buffer",
            "CTRL-E: Enter edit mode",
            "CTRL-N: Enter Normal mode",
            "CTRL-A: Enter Select mode",
            "CTRL-S: Enter Navigate mode",
        ])
        .block(Block::bordered().title("Keys"));

        let block = ratatui::widgets::Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .title("Normal Mode");

        frame.render_widget(lst, dest);
    }
}

impl EditorMode for NormalMode {
    fn mode_name(&self) -> &'static str {
        "NORMAL"
    }

    fn handle_key_event(
        &self,
        key_event: crossterm::event::KeyEvent,
        app_state: &mut crate::app::ApplicationState,
    ) {
        if key_event.modifiers.contains(KeyModifiers::CONTROL) {
            return;
        }

        match key_event.code {
            KeyCode::Char(c) => match c {
                'n' => new_buffer(app_state),
                'c' => close_buffer(app_state),
                'a' => rename_buffer(app_state),
                _ => (),
            },
            KeyCode::Left => rotate_buffer(app_state, -1),
            KeyCode::Right => rotate_buffer(app_state, 1),
            _ => {}
        }
    }

    fn render(&self, frame: &mut ratatui::Frame, _app_state: &crate::app::ApplicationState) {
        // do a horizontal layout containing the logo and the list of keys
        let layout = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Length(3),
                ratatui::layout::Constraint::Length(5),
                ratatui::layout::Constraint::Min(1),
            ])
            .split(frame.area());

        render_mode_header(frame, layout[0], self.mode_name(), _app_state);
        self.render_logo(frame, layout[1]);
        self.render_menu(frame, layout[2]);
    }
}

fn rotate_buffer(app_state: &mut app::ApplicationState, direction: i32) {
    let next_buffer_id = app_state.current_buffer as i32 + direction;
    app_state.current_buffer = (next_buffer_id % app_state.buffers.len() as i32) as usize;
}

fn rename_buffer(app_state: &mut app::ApplicationState) {
    todo!()
}

fn close_buffer(app_state: &mut app::ApplicationState) {
    app_state.buffers.remove(app_state.current_buffer);
    app_state.current_buffer = app_state.current_buffer.saturating_sub(1);
}

fn new_buffer(app_state: &mut app::ApplicationState) {
    let mut buff = app::BufferEntry::default();
    buff.name = "untitled".to_string();
    app_state.buffers.push(buff);
    app_state.current_buffer = app_state.buffers.len() - 1;
}
