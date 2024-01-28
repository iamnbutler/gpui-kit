use gpui::svg;

pub enum IconName {
    Check,
    QuestionMark,
}

impl IconName {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Check => "icons/check.svg",
            Self::QuestionMark => "icons/question-mark.svg",
        }
    }
}

pub struct Icon {}

impl Icon {
    pub fn new(icon: IconName) -> gpui::Svg {
        svg().path(icon.path())
    }
}
