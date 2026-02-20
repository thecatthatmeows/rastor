use color_eyre::Result;
use crossterm::{
    cursor::MoveTo,
    event::KeyCode,
    execute,
    style::Color,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use glyph::{
    key::handle_key, shapes::rectangle::Rectangle, types::vec2::Vec2, utils::get_terminal_size,
};
use std::{f32::consts::PI, io::stdout};

fn main() -> Result<()> {
    let mut is_running = true;
    color_eyre::install().unwrap();
    let mut stdout = stdout();

    let term_size = get_terminal_size()?;
    let initial_pos = term_size / Vec2::splat(2);
    let mut rect = Rectangle::new(initial_pos.to_f32(), Vec2::splat(10.0), Color::Green);

    let rad = PI / 16.0;

    enable_raw_mode().unwrap();
    while is_running {
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)).unwrap();

        rect.draw();
        rect.update();
        // rect.size += Vec2::splat(0.5);

        rect.rotate(rad);
        // rad = (rad * 2.0) % (PI * 2.0);

        handle_key(KeyCode::Char('q'), || is_running = false);
    }
    disable_raw_mode().unwrap();

    println!();
    Ok(())
}
