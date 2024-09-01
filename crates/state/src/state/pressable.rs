pub trait Pressable {
    fn pressed(self) -> bool;
    fn set_pressed(&mut self, focused: bool);
}
