use gpui::*;

use crate::color::DEFAULT_ACCENT;

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

#[derive(Clone, Refineable)]
pub struct Styles {
    text: TextStyle,
    link: HighlightStyle,
}

impl Styles {
    pub fn init(cx: &WindowContext) -> Self {
        let text = cx.text_style();
        let link = HighlightStyle {
            color: Some(DEFAULT_ACCENT),
            font_weight: None,
            font_style: None,
            background_color: None,
            underline: Some(UnderlineStyle {
                thickness: px(1.0),
                color: Some(DEFAULT_ACCENT),
                wavy: false,
            }),
            strikethrough: None,
            fade_out: None,
        };

        Self { text, link }
    }
}

impl Global for Styles {}
