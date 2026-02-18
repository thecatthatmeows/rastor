use color_eyre::Result;
use crate::types::vec2::Vec2;

pub fn get_terminal_size() -> Result<Vec2<u16>> {
    let size = crossterm::terminal::size()?;
    Ok(Vec2::new(size.0, size.1))
}
