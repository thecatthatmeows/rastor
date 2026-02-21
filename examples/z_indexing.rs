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
    shapes::{Shape, rectangle::Rectangle},
    types::vec2::Vec2,
    utils::get_terminal_size,
};

fn main() -> color_eyre::Result<()> {
    let term_size = get_terminal_size()?;
    let initial_pos = term_size / Vec2::splat(2);

    let mut rectangles = Vec::new();
    rectangles.push(Rectangle::new(
        initial_pos.to_f32(),
        Vec2::splat(5.0),
        Color::Green,
    ));
    rectangles.push(Rectangle::new(
        initial_pos.to_f32() + Vec2::splat(3.0),
        Vec2::splat(5.0),
        Color::Blue,
    ));
    rectangles[1].z_index = 10; // second rectangle = more important
    rectangles.push(Rectangle::new(
        initial_pos.to_f32() + Vec2::splat(6.0),
        Vec2::splat(5.0),
        Color::Yellow,
    ));
    rectangles[2].z_index = 20; // third rectangle = more important
    rectangles.push(Rectangle::new(
        initial_pos.to_f32() + Vec2::splat(9.0),
        Vec2::splat(5.0),
        Color::Magenta,
    ));
    rectangles[3].z_index = -10; // fourth rectangle = least important
    rectangles.sort_by_key(|rect| rect.z_index);

    let mut stdout = stdout().lock();
    let mut is_running = true;
    enable_raw_mode().unwrap();
    while is_running {
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)).unwrap();

        for rect in &mut rectangles {
            rect.draw();
            rect.update();
            // rect.size += Vec2::splat(0.5);
        }

        handle_key(KeyCode::Char('q'), || is_running = false);
    }
    disable_raw_mode().unwrap();

    Ok(())
}
