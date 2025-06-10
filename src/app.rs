use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use crate::{
    bufferentry::BufferEntry,
    insertmode::InsertMode,
    mode::{EditorMode, Mode},
    navigationmode::NavigationMode,
    normalmode::NormalMode,
    selectmode::SelectMode,
};

#[derive(Default)]
pub struct ApplicationState {
    pub buffers: Vec<BufferEntry>,
    pub current_buffer: usize,
    pub window_size: (u16, u16),
    pub ctrl_active: bool,
}

#[derive(Default)]
pub struct App {
    exit: bool,
    current_mode: Mode,

    app_state: ApplicationState,

    normal_mode: NormalMode,
    insert_mode: InsertMode,
    navigation_mode: NavigationMode,
    select_mode: SelectMode,
}

impl App {
    pub fn new() -> App {
        let app = App {
            ..Default::default()
        };
        app
    }

    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        file_name: Option<String>,
    ) -> io::Result<()> {
        if let Some(file_name) = file_name {
            self.app_state
                .buffers
                .push(BufferEntry::from_file(file_name));
            self.app_state.current_buffer = self.app_state.buffers.len() - 1;
            self.current_mode = Mode::Insert;
        }

        while !self.exit {
            let _ = terminal.clear();
            terminal.draw(|frame| self.draw(frame))?;

            let s = terminal.size().unwrap();
            self.app_state.window_size = (s.width, s.height - crate::modeutil::TOP_BAR_HEIGHT);

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
            Mode::Navigate => func(&self.navigation_mode, &self.app_state),
            Mode::Select => func(&self.select_mode, &self.app_state),
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
            Mode::Navigate => func(&mut self.navigation_mode, &mut self.app_state),
            Mode::Select => func(&mut self.select_mode, &mut self.app_state),
            _ => panic!("Unknown mode"),
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.app_state.ctrl_active =
                    key_event.modifiers.contains(event::KeyModifiers::CONTROL);

                if key_event.modifiers.contains(event::KeyModifiers::CONTROL) {
                    // change modes
                    match key_event.code {
                        KeyCode::Char('e') => {
                            if self.app_state.buffers.len() > 0 {
                                self.current_mode = Mode::Insert
                            }
                        }
                        //KeyCode::Char('a') => self.current_mode = Mode::Navigate,
                        KeyCode::Char('w') => self.current_mode = Mode::Select,
                        KeyCode::Char('q') => self.current_mode = Mode::Normal,
                        KeyCode::Char(_) => self
                            .navigation_mode
                            .handle_key_event(key_event, &mut self.app_state),
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
