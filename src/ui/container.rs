pub trait UIElement {
    fn draw(&self);
    fn update(&mut self);
}

pub struct UIContainer {
    children: Vec<Box<dyn UIElement>>,
}

impl UIContainer {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Box<dyn UIElement>) {
        self.children.push(child);
    }
}

impl UIElement for UIContainer {
    fn draw(&self) {
        for child in &self.children {
            child.draw();
        }
    }

    fn update(&mut self) {
        for child in &mut self.children {
            child.update();
        }
    }
}