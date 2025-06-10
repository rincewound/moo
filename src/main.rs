use std::{env, io};

use app::App;

mod app;
mod buffer;
mod bufferentry;
mod insertmode;
mod mode;
mod modeutil;
mod navigationmode;
mod normalmode;
mod selectmode;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file_name: Option<String> = None;

    if args.len() > 1 {
        file_name = Some(args[1].clone());
    }

    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal, file_name);
    ratatui::restore();
    app_result
}
