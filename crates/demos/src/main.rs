use gpui::*;
use gpui_kit::init;
use window::{window_options, ExampleWindow};

mod window;

fn main() {
    App::new().run(|cx: &mut AppContext| {
        init::init(cx);

        cx.open_window(window_options(540, 720, cx), |cx| {
            cx.new_view(|cx| ExampleWindow::new(cx))
        });
    });
}
