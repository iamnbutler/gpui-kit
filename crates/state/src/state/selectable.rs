pub trait Selectable {
    fn selected(self) -> Selected;
    fn set_selected(&mut self, selected: Selected);
}

pub enum Selected {
    Selected,
    Unselected,
    Indeterminate,
}
