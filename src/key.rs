use crossterm::event;
use std::{collections::HashSet, time::Duration};

// for export
pub type KeyCode = event::KeyCode;

pub struct KeyInput {
    current: HashSet<KeyCode>,
    previous: HashSet<KeyCode>,
}

impl KeyInput {
    pub fn new() -> Self {
        Self {
            current: HashSet::new(),
            previous: HashSet::new(),
        }
    }

    pub fn is_key_pressed<F>(&mut self, key: KeyCode, mut handler: F) -> bool
    where
        F: FnMut(),
    {
        self.previous = HashSet::from([key]);

        if event::poll(Duration::from_millis(0)).unwrap()
            && let event::Event::Key(event) = event::read().unwrap()
        {
            match event.kind {
                event::KeyEventKind::Press => {
                    self.current.insert(event.code);
                    if event.code == key {
                        handler();
                        return true;
                    }
                }
                event::KeyEventKind::Release => { self.current.remove(&event.code); }
                _ => {}
            }
        }
        false
    }

    pub fn is_key_released<F>(&mut self, key: KeyCode, mut handler: F) -> bool
    where
        F: FnMut(),
    {
        self.previous = HashSet::from([key]);

        if event::poll(Duration::from_millis(0)).unwrap()
            && let event::Event::Key(event) = event::read().unwrap()
        {
            match event.kind {
                event::KeyEventKind::Release => {
                    self.current.remove(&event.code);
                    if event.code == key {
                        handler();
                        return true;
                    }
                }
                event::KeyEventKind::Press => { self.current.insert(event.code); }
                _ => {}
            }
        }
        false
    }
}
