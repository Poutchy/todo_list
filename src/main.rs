use color_eyre::Result;

mod app;
mod ui;
mod model;
mod utils;

use crate::app::App;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}
