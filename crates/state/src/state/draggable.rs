use gpui::WindowContext;

pub trait Draggable {
    fn dragged(self) -> bool;
    fn set_dragged(&mut self, dragged: bool);
    fn on_drag(&mut self, handler: impl 'static + Fn(&mut WindowContext) + Send + Sync) -> Self;
    fn on_drop(&mut self, handler: impl 'static + Fn(&mut WindowContext) + Send + Sync) -> Self;
}
