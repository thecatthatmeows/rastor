use std::{io::stdout, thread::sleep, time::Duration};

use crossterm::{
    cursor::MoveTo,
    event::KeyCode,
    execute,
    style::Color,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use sigil::{
    key::handle_key,
    shapes::{Shape, circle::Circle, rectangle::Rectangle},
    types::vec2::Vec2,
    utils::get_terminal_size,
};
use rand::random_range;

fn main() -> color_eyre::Result<()> {
    // 40x40, purely rectangles (x, y, w, h)
    let mut patch_rects = Vec::new();
    for _ in 0..70 {
        let rand_x = random_range(0.0..23.0);
        let rand_y = random_range(0.0..15.0);
        patch_rects.push((rand_x, rand_y, 1.0, 1.0));
    }

    let term_size = get_terminal_size()?;
    let initial_pos = term_size / Vec2::splat(2);

    let mut circle = Circle::new(initial_pos.to_f32(), 10.0, 64, Color::Blue);
    circle.z_index = 0;
    let mut patches = Vec::new();

    for (rect_x, rect_y, rect_w, rect_h) in patch_rects {
        let mut patch = Rectangle::new(
            initial_pos.to_f32() + Vec2::new(rect_x as f32, rect_y as f32) - Vec2::new(12.0, 8.0),
            Vec2::new(rect_w as f32, rect_h as f32),
            Color::Green,
        );
        patch.z_index = 10;
        patches.push(patch);
    }

    let mut stdout = stdout().lock();
    let mut is_running = true;
    enable_raw_mode().unwrap();
    while is_running {
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)).unwrap();

        circle.draw();
        circle.update();
        for patch in &mut patches {
            patch.draw();
            patch.update();
        }

        handle_key(KeyCode::Char('q'), || is_running = false);
    }
    disable_raw_mode().unwrap();

    Ok(())
}
