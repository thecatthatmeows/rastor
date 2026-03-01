use crate::ui::style::border::Border;

pub struct ContainerStyle {
    pub border: Option<Border>,
}

impl ContainerStyle {
    pub fn new(border: Option<Border>) -> Self {
        Self { border }
    }
}
