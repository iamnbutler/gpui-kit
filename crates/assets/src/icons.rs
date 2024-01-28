use gpui::{rgb, svg, Styled};

pub enum IconName {
    Check,
    QuestionMark,
}

impl IconName {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Check => "icons/lucide/check.svg",
            Self::QuestionMark => "icons/lucide/help-circle.svg",
        }
    }
}

pub struct Icon {}

impl Icon {
    pub fn new(icon: IconName) -> gpui::Svg {
        svg().size_4().text_color(rgb(0xff00ff)).path(icon.path())
    }
}
