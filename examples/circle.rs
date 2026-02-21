use std::{io::stdout, thread::sleep, time::Duration};

use crossterm::{
    cursor::MoveTo,
    event::KeyCode,
    execute,
    style::Color,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use glyph::{
    key::handle_key,
    shapes::{Shape, circle::Circle, rectangle::Rectangle},
    types::vec2::Vec2,
    utils::get_terminal_size,
};

fn main() -> color_eyre::Result<()> {
    let term_size = get_terminal_size()?;
    let initial_pos = term_size / Vec2::splat(2);

    // let mut circle = Circle::new(initial_pos.to_f32(), 10.0, 8, Color::Green);
    let mut circle = Circle::new(initial_pos.to_f32(), 10.0, 32, Color::Green);

    let mut stdout = stdout().lock();
    let mut is_running = true;
    enable_raw_mode().unwrap();
    while is_running {
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)).unwrap();

        circle.draw();
        circle.update();

        handle_key(KeyCode::Char('q'), || is_running = false);
    }
    disable_raw_mode().unwrap();

    Ok(())
}
