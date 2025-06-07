use std::io::Write;

use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::Stylize,
    widgets::{Block, List, Paragraph, Wrap},
};

use crate::{
    app,
    bufferentry::BufferEntry,
    mode::EditorMode,
    modeutil::{render_mode_header, rotate_buffer},
};

#[derive(Default, PartialEq)]
enum ActivePopup {
    #[default]
    None,
    RenameBuffer,
    OpenFile,
}

#[derive(Default)]
pub struct NormalMode {
    active_popup: ActivePopup,
    fuzzy_open_search: String,
    fuzzy_open_suggestion: String,
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
            "o: Open File",
            "CTRL-Q: Enter Normal mode",
            "CTRL-W: Enter Select mode",
            "CTRL-E: Enter edit mode",
        ])
        .block(Block::bordered().title("Keys"));

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

    fn render_open_file_popup(
        &self,
        frame: &mut ratatui::Frame,
        dest: Rect,
        app_state: &app::ApplicationState,
    ) {
        let paragraph = Paragraph::new("Open File")
            .centered()
            .wrap(Wrap { trim: true });
        frame.render_widget(paragraph, dest);

        let block = Block::bordered().title("Fuzzy open file..").on_blue();
        let buffer_name = format!("{}{}", self.fuzzy_open_search, "_");
        let buffer_suggestion = format!("{}", self.fuzzy_open_suggestion);
        let area = popup_area(dest, 60, 30);
        frame.render_widget(block.clone(), area);

        let lst = List::new([
            buffer_name.fg(ratatui::style::Color::default()),
            buffer_suggestion.fg(ratatui::style::Color::DarkGray),
        ])
        .block(block.clone());

        frame.render_widget(lst, area);
        // frame.render_widget(Paragraph::new(buffer_suggestion.clone()).block(block), area);
    }

    fn rename_buffer(&mut self, _app_state: &app::ApplicationState) {
        if _app_state.buffers.is_empty() {
            return;
        }
        self.active_popup = ActivePopup::RenameBuffer;
    }

    fn open_file(&mut self, _app_state: &app::ApplicationState) {
        self.active_popup = ActivePopup::OpenFile;
    }

    fn handle_keys_open_file(
        &mut self,
        key_event: crossterm::event::KeyEvent,
        app_state: &mut app::ApplicationState,
    ) {
        match key_event.code {
            KeyCode::Enter => {
                if self.fuzzy_open_suggestion.is_empty() {
                    return;
                }

                let mut buffer = BufferEntry::from_file(self.fuzzy_open_suggestion.clone());
                buffer.name = self.fuzzy_open_suggestion.clone();
                app_state.buffers.push(buffer);
                app_state.current_buffer = app_state.buffers.len() - 1;
                self.active_popup = ActivePopup::None;
            }
            KeyCode::Char(c) => {
                self.fuzzy_open_search.push(c);
                self.update_suggsestions(app_state);
            }
            KeyCode::Backspace => {
                self.fuzzy_open_search.pop();
                self.update_suggsestions(app_state);
            }
            _ => (),
        }
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
                self.fuzzy_open_suggestion = String::new();
                self.fuzzy_open_search = String::new();
            }
            KeyCode::Char(c) => {
                let buffer = &mut app_state.buffers[app_state.current_buffer];
                buffer.name.push(c);
                self.update_suggsestions(app_state);
            }
            KeyCode::Backspace => {
                let buffer = &mut app_state.buffers[app_state.current_buffer];
                buffer.name.pop();
                self.update_suggsestions(app_state);
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
                'o' => self.open_file(app_state),
                _ => (),
            },
            KeyCode::Left => rotate_buffer(app_state, -1),
            KeyCode::Right => rotate_buffer(app_state, 1),
            KeyCode::Enter => {
                self.active_popup = ActivePopup::None
                // It would be neat to trigger a mode change to Insert mode here.
            }
            _ => {}
        }
    }

    fn update_suggsestions(&mut self, app_state: &mut app::ApplicationState) {
        if self.fuzzy_open_search.is_empty() {
            self.fuzzy_open_suggestion = String::new();
            return;
        }

        // Check the CWD for files that start with the typed in buffer name
        for file in std::fs::read_dir(".").unwrap() {
            let file_name = file.unwrap().file_name().to_string_lossy().to_string();
            if file_name.starts_with(self.fuzzy_open_search.as_str()) {
                self.fuzzy_open_suggestion = file_name;
            }
        }
    }
}

fn write_buffer(app_state: &mut app::ApplicationState) {
    let buffer = &mut app_state.buffers[app_state.current_buffer];
    buffer.modified = false;

    // persists buffer as it is to file
    let mut file = std::fs::File::create(&buffer.name).unwrap();

    // convert all lines to strings:
    for line in buffer.buffer.lines.iter() {
        let line_as_string = line.iter().map(|c| c.to_string()).collect::<String>();
        file.write_all(line_as_string.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }

    file.flush().unwrap();
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

        match self.active_popup {
            ActivePopup::None => self.handle_keys_default(key_event, app_state),
            ActivePopup::RenameBuffer => self.handle_keys_rename(key_event, app_state),
            ActivePopup::OpenFile => self.handle_keys_open_file(key_event, app_state),
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
            ActivePopup::OpenFile => {
                self.render_open_file_popup(frame, layout[2], app_state);
            }
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
    let mut buff = BufferEntry::default();
    buff.name = "untitled".to_string();
    app_state.buffers.push(buff);
    app_state.current_buffer = app_state.buffers.len() - 1;
}
