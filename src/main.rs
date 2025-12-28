use filter_former::app::App;
use std::io;

fn main() -> Result<(), io::Error> {
    let app = App::init()?;

    let terminal = ratatui::init();
    let app_result = app.run(terminal);

    ratatui::restore();
    app_result
}
