use std::io::stdout;

use crossterm::{
    cursor::MoveTo,
    event::KeyCode,
    execute,
    style::Color,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use glyph::{key::handle_key, shapes::rectangle::Rectangle, types::vec2::Vec2};

fn main() {
    let mut is_running = true;
    color_eyre::install().unwrap();
    let mut stdout = stdout();

    let mut rect = Rectangle::new(Vec2::new(30.0, 30.0), Vec2::new(0.0, 0.0), Color::Green);

    enable_raw_mode().unwrap();
    while is_running {
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)).unwrap();

        rect.draw();
        rect.update();
        rect.size += Vec2::new(0.1, 0.1);

        handle_key(KeyCode::Char('q'), || is_running = false);
    }
    disable_raw_mode().unwrap();

    println!();
}
