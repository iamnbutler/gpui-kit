use gpui::{
    div, point, prelude::*, px, rgb, size, App, Bounds, GlobalPixels, SharedString, Size,
    WindowBounds, WindowOptions,
};

struct Hello {
    text: SharedString,
}

impl Render for Hello {
    fn render(&mut self, _cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x000000))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", &self.text))
    }
}

fn main() {
    App::new().run(|cx| {
        let displays = cx.displays();
        let first_display = displays.first().expect("no displays");

        let window_size: Size<GlobalPixels> = size(px(800.), px(600.)).into();
        let window_origin = point(
            first_display.bounds().center().x - window_size.width / 2.,
            first_display.bounds().center().y - window_size.height / 2.,
        );

        cx.open_window(
            WindowOptions {
                bounds: WindowBounds::Fixed(Bounds::<GlobalPixels>::new(
                    window_origin,
                    size(px(800.), px(600.)).into(),
                )),
                ..Default::default()
            },
            |cx| {
                cx.new_view(|_cx| Hello {
                    text: "Vitesse".into(),
                })
            },
        );
    })
}
