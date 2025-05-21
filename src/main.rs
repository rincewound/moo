use std::io;

use app::App;

mod app;
mod buffer;
mod insertmode;
mod mode;
mod modeutil;
mod normalmode;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}
