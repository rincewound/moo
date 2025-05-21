use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use crate::{
    buffer::Buffer,
    insertmode::InsertMode,
    mode::{EditorMode, Mode},
    normalmode::NormalMode,
};

#[derive(Default)]
pub struct BufferEntry {
    pub name: String,
    pub buffer: Buffer,
    pub cursor_line: usize,
    pub cursor_char: usize,
    pub modified: bool,
    pub scroll_offset: usize,

    pub selection_start: Option<(usize, usize)>, // line + char
    pub selection_end: Option<(usize, usize)>,   // line + char
}

#[derive(Default)]
pub struct ApplicationState {
    pub buffers: Vec<BufferEntry>,
    pub current_buffer: usize,
}

#[derive(Default)]
pub struct App {
    exit: bool,
    current_mode: Mode,

    app_state: ApplicationState,

    normal_mode: NormalMode,
    insert_mode: InsertMode,
}

impl App {
    pub fn new() -> App {
        let app = App {
            ..Default::default()
        };

        // app.app_state.buffers.push(BufferEntry::default());
        // app.app_state.buffers[0].name = "untitled".to_string();

        app
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        self.with_active_mode(|mode, app_state| {
            mode.render(frame, app_state);
        });
    }

    fn with_active_mode(&self, func: impl FnOnce(&dyn EditorMode, &ApplicationState)) {
        match self.current_mode {
            Mode::Normal => func(&self.normal_mode, &self.app_state),
            Mode::Insert => func(&self.insert_mode, &self.app_state),
            _ => (),
        }
    }

    fn with_active_mode_mut(
        &mut self,
        func: impl FnOnce(&mut dyn EditorMode, &mut ApplicationState),
    ) {
        match self.current_mode {
            Mode::Normal => func(&mut self.normal_mode, &mut self.app_state),
            Mode::Insert => func(&mut self.insert_mode, &mut self.app_state),
            _ => panic!("Unknown mode"),
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                if key_event.modifiers.contains(event::KeyModifiers::CONTROL) {
                    // change modes
                    match key_event.code {
                        KeyCode::Char('e') => {
                            if self.app_state.buffers.len() > 0 {
                                self.current_mode = Mode::Insert
                            }
                        }
                        KeyCode::Char('a') => self.current_mode = Mode::Navigate,
                        KeyCode::Char('s') => self.current_mode = Mode::Select,
                        KeyCode::Char('n') => self.current_mode = Mode::Normal,
                        _ => (),
                    }
                }
                match key_event.code {
                    KeyCode::Esc => self.exit = true,
                    _ => (),
                }

                self.with_active_mode_mut(|mode, appstate| {
                    mode.handle_key_event(key_event, appstate);
                });
            }
            _ => {}
        };
        Ok(())
    }
}
