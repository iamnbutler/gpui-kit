use gpui::WindowContext;

pub trait Disableable {
    fn disabled(self) -> bool;
    fn set_disabled(&mut self, focused: bool);
    fn on_disable(&mut self, handler: impl 'static + Fn(&mut WindowContext) + Send + Sync) -> Self;
    fn on_enable(&mut self, handler: impl 'static + Fn(&mut WindowContext) + Send + Sync) -> Self;
}
