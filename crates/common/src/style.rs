use gpui::*;

/// Styles an outline, like a border or ring
#[derive(Clone)]
pub struct Outline {
    pub color: Hsla,
    pub radius: f32,
    pub width: f32,
}

impl Default for Outline {
    fn default() -> Self {
        Self {
            color: hsla(0.0, 0.0, 0.46, 1.0),
            radius: 2.0,
            width: 1.0,
        }
    }
}

impl Outline {
    pub fn new(color: Hsla) -> Self {
        Self {
            color,
            ..Default::default()
        }
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = color;
        self
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}
