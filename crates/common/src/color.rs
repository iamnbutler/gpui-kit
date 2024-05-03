use gpui::{hsla, Hsla};

pub const DEFAULT_ACCENT: Hsla = Hsla {
    h: 246. / 360.,
    s: 0.91,
    l: 0.49,
    a: 1.0,
};
pub const DEFAULT_SELECTION: Hsla = Hsla {
    h: 0.6,
    s: 0.67,
    l: 0.46,
    a: 0.5,
};
pub const DEFAULT_BORDER: Hsla = Hsla {
    h: 0.0,
    s: 0.0,
    l: 0.73,
    a: 1.0,
};

pub const DEFAULT_BORDER_HOVER: Hsla = Hsla {
    h: 0.0,
    s: 0.0,
    l: 0.31,
    a: 1.0,
};

pub fn transparent() -> Hsla {
    hsla(0.0, 0.0, 0.0, 0.0)
}
