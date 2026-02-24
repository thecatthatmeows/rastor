use std::io::stdout;

use color_eyre::Result;
use crossterm::{
    cursor::MoveTo,
    event::KeyCode,
    execute,
    style::Color,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use rastor::{
    key::handle_key,
    shapes::{Shape, rectangle::Rectangle},
    types::vec2::Vec2,
    utils::get_terminal_size,
};

fn main() -> Result<()> {
    let mut is_running = true;
    color_eyre::install().unwrap();
    let mut stdout = stdout();

    let term_size = get_terminal_size()?;
    let initial_pos = term_size / Vec2::splat(2);
    let mut rect = Rectangle::new(initial_pos.to_f32().into(), Vec2::splat(5.0), Color::Green);
    let mut other_rect = Rectangle::new((initial_pos.to_f32() + 2.0).into(), Vec2::splat(5.0), Color::Green);

    enable_raw_mode().unwrap();
    while is_running {
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)).unwrap();

        rect.draw();
        rect.update();
        other_rect.draw();
        other_rect.update();

        let rect_collides_with_other = rect.collides_with(&other_rect);
        println!("{}", rect_collides_with_other);

        handle_key(KeyCode::Char('q'), || is_running = false);
    }
    disable_raw_mode().unwrap();

    println!();
    Ok(())
}
