use std::time::Duration;
use crossterm::event::{self, KeyCode};

pub fn handle_key<F>(key: KeyCode, mut handler: F) -> bool
where
    F: FnMut()
{
    if event::poll(Duration::from_millis(100)).unwrap()
    && let event::Event::Key(event) = event::read().unwrap() {
        if event.code == key {
            handler();
            return true;
        }
    }
    false
}
