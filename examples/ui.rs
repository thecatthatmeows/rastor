use std::{io::stdout, thread::sleep, time::Duration};

use color_eyre::Result;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::Color,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use rastor::{
    key::{KeyCode, KeyInput}, shapes::{Shape, rectangle::Rectangle}, types::vec2::Vec2, ui::{UIElement, container::{UIContainer, style::ContainerStyle}, style::border::{Border, BorderStyle}, text::{Text, style::TextStyle}}, utils::get_terminal_size
};

fn main() -> Result<()> {
    let mut is_running = true;
    color_eyre::install().unwrap();
    let mut stdout = stdout();

    let term_size = get_terminal_size()?;
    let initial_pos = term_size / Vec2::splat(2);
    let mut container = UIContainer::new(initial_pos.to_f32().into(), Vec2::new(20.0, 10.0));
    let mut text = Text::new(initial_pos.to_f32().into(), Vec2::new(2.0, 2.0), String::from("Hello, World!"));
    text.text_style = Some(TextStyle::new(text.size.x as u16, Color::Green, Color::Black));
    container.add_child(Box::new(text.clone()));

    container.style = Some(ContainerStyle::new(Some(Border::new(container.pos, container.size.x, Color::White, BorderStyle::Solid))));

    let mut key_input = KeyInput::new();

    enable_raw_mode().unwrap();
    while is_running {
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)).unwrap();

        container.draw();

        if key_input.is_down(&KeyCode::Char('q')) { is_running = false }

        sleep(Duration::from_millis(16));
    }
    disable_raw_mode().unwrap();

    println!();
    Ok(())
}
