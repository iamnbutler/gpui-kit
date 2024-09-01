use gpui::WindowContext;

pub trait Focusable {
    fn focused(self) -> bool;
    fn set_focused(&mut self, focused: bool);
    fn on_focus(&mut self, handler: impl 'static + Fn(&mut WindowContext) + Send + Sync) -> Self;
    fn on_blur(&mut self, handler: impl 'static + Fn(&mut WindowContext) + Send + Sync) -> Self;
}
