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

    pub fn update<F>(&mut self) {
        self.previous = self.current.clone();

        if event::poll(Duration::from_millis(0)).unwrap()
            && let event::Event::Key(event) = event::read().unwrap()
        {
            match event.kind {
                event::KeyEventKind::Press | event::KeyEventKind::Repeat => { self.current.insert(event.code); }
                event::KeyEventKind::Release => { self.current.remove(&event.code); }
            }
        }
    }

    pub fn is_down(&mut self, key: &KeyCode) -> bool {
        self.current.contains(key)
    }

    pub fn is_pressed(&mut self, key: &KeyCode) -> bool {
        self.current.contains(key) && !self.previous.contains(key)
    }

    pub fn is_released(&mut self, key: &KeyCode) -> bool {
        !self.current.contains(key) && self.previous.contains(key)
    }
}
