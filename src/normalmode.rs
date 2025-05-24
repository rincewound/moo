use std::io::Write;

use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::Stylize,
    widgets::{Block, Paragraph, Wrap},
};

use crate::{
    app,
    mode::EditorMode,
    modeutil::{render_mode_header, rotate_buffer},
};

#[derive(Default, PartialEq)]
enum ActivePopup {
    #[default]
    None,
    RenameBuffer,
}

#[derive(Default)]
pub struct NormalMode {
    active_popup: ActivePopup,
}

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

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

    fn render_default_view(
        &self,
        frame: &mut ratatui::Frame<'_>,
        layout: std::rc::Rc<[ratatui::prelude::Rect]>,
    ) {
        self.render_logo(frame, layout[1]);
        self.render_menu(frame, layout[2]);
    }

    fn render_rename_popup(
        &self,
        frame: &mut ratatui::Frame,
        dest: Rect,
        app_state: &app::ApplicationState,
    ) {
        let paragraph = Paragraph::new("Rename Buffer")
            .centered()
            .wrap(Wrap { trim: true });
        frame.render_widget(paragraph, dest);

        let block = Block::bordered().title("Current Buffer Name").on_blue();
        let buffer_name = format!(
            "{}{}",
            app_state.buffers[app_state.current_buffer].name, "_"
        );
        let area = popup_area(dest, 60, 20);
        frame.render_widget(block.clone(), area);
        frame.render_widget(Paragraph::new(buffer_name.clone()).block(block), area);
    }

    fn rename_buffer(&mut self, _app_state: &app::ApplicationState) {
        if _app_state.buffers.is_empty() {
            return;
        }
        self.active_popup = ActivePopup::RenameBuffer;
    }

    fn handle_keys_rename(
        &mut self,
        key_event: crossterm::event::KeyEvent,
        app_state: &mut app::ApplicationState,
    ) {
        match key_event.code {
            KeyCode::Enter => {
                let buffer = &mut app_state.buffers[app_state.current_buffer];
                buffer.name = buffer.name.trim().to_string();
                if buffer.name.is_empty() {
                    return;
                }
                self.active_popup = ActivePopup::None;
            }
            KeyCode::Char(c) => {
                let buffer = &mut app_state.buffers[app_state.current_buffer];
                buffer.name.push(c);
            }
            KeyCode::Backspace => {
                let buffer = &mut app_state.buffers[app_state.current_buffer];
                buffer.name.pop();
            }
            _ => (),
        }
    }

    fn handle_keys_default(
        &mut self,
        key_event: crossterm::event::KeyEvent,
        app_state: &mut app::ApplicationState,
    ) {
        match key_event.code {
            KeyCode::Char(c) => match c {
                'n' => new_buffer(app_state),
                'c' => close_buffer(app_state),
                'a' => self.rename_buffer(app_state),
                'w' => write_buffer(app_state),
                _ => (),
            },
            KeyCode::Left => rotate_buffer(app_state, -1),
            KeyCode::Right => rotate_buffer(app_state, 1),
            KeyCode::Enter => self.active_popup = ActivePopup::None,
            _ => {}
        }
    }
}

fn write_buffer(app_state: &mut app::ApplicationState) {
    let buffer = &mut app_state.buffers[app_state.current_buffer];
    buffer.modified = false;

    // persists buffer as it is to file
    let mut file = std::fs::File::create(&buffer.name).unwrap();
    // Note: No explicit line endings are added.
    file.write_all(buffer.buffer.lines.join("").as_bytes())
        .unwrap();
}

impl EditorMode for NormalMode {
    fn mode_name(&self) -> &'static str {
        "NORMAL"
    }

    fn handle_key_event(
        &mut self,
        key_event: crossterm::event::KeyEvent,
        app_state: &mut crate::app::ApplicationState,
    ) {
        if key_event.modifiers.contains(KeyModifiers::CONTROL) {
            return;
        }

        if self.active_popup == ActivePopup::RenameBuffer {
            self.handle_keys_rename(key_event, app_state);
        } else {
            self.handle_keys_default(key_event, app_state);
        }
    }

    fn render(&self, frame: &mut ratatui::Frame, app_state: &crate::app::ApplicationState) {
        // do a horizontal layout containing the logo and the list of keys
        let layout = ratatui::layout::Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                ratatui::layout::Constraint::Length(3),
                ratatui::layout::Constraint::Length(5),
                ratatui::layout::Constraint::Min(1),
            ])
            .split(frame.area());

        render_mode_header(frame, layout[0], self.mode_name(), app_state);

        match self.active_popup {
            ActivePopup::None => {
                self.render_default_view(frame, layout);
            }
            ActivePopup::RenameBuffer => {
                self.render_rename_popup(frame, layout[2], app_state);
            }
            _ => {}
        }
    }
}

fn close_buffer(app_state: &mut app::ApplicationState) {
    if app_state.buffers[app_state.current_buffer].modified {
        // ask user if they want to save changes
    }

    app_state.buffers.remove(app_state.current_buffer);
    app_state.current_buffer = app_state.current_buffer.saturating_sub(1);
}

fn new_buffer(app_state: &mut app::ApplicationState) {
    let mut buff = app::BufferEntry::default();
    buff.name = "untitled".to_string();
    app_state.buffers.push(buff);
    app_state.current_buffer = app_state.buffers.len() - 1;
}
