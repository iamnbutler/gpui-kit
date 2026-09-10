use gpui::{App, Window};

pub trait WindowCompat {
    fn blur(&mut self, cx: &mut App);
}

impl WindowCompat for Window {
    #[inline]
    fn blur(&mut self, _cx: &mut App) {
        #[cfg(feature = "gpui-1-18")]
        {
            Window::blur(self); 
        }
        #[cfg(feature = "gpui-1-19")]
        {
            Window::blur(self, _cx);
        }
    }

}
