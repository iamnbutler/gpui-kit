use gpui::*;

/// Styles an Input
pub struct InputStyle {
    pub background: Hsla,
    pub padding: Edges<f32>,
    pub text: TextStyle,
}

impl Default for InputStyle {
    fn default() -> Self {
        Self {
            background: hsla(0.0, 0.0, 1.0, 1.0),
            padding: Edges {
                top: 0.0,
                bottom: 0.0,
                left: 4.0,
                right: 4.0,
            },
            text: TextStyle::default(),
            // margin: Edges::all(0.0),
            // ring: None,
            // border: Outline::new(hsla(0.0, 0.0, 0.31, 0.4)),
        }
    }
}
