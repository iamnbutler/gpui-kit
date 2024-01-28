pub trait Hoverable {
    fn hovered(self) -> bool;
    fn set_hovered(&mut self, focused: bool);
}
