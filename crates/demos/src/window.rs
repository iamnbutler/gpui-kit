#![allow(unused)]
use gpui::{prelude::*, *};

pub fn window_options(app_width: i32, app_height: i32, cx: &AppContext) -> WindowOptions {
    let display_id_maybe = cx.displays().last().map(|d| d.id());
    let bounds_maybe = cx.displays().last().map(|d| d.bounds());
    let bounds = bounds_maybe.unwrap_or(Bounds {
        origin: Point::new(DevicePixels::from(0), DevicePixels::from(0)),
        size: Size {
            width: DevicePixels::from(1920),
            height: DevicePixels::from(1080),
        },
    });

    let mut options = WindowOptions::default();
    let center = bounds.center();

    options.focus = true;
    options.display_id = display_id_maybe;
    let width = DevicePixels::from(app_width);
    let height = DevicePixels::from(app_height);
    let x: DevicePixels = center.x - width / 2;
    let y: DevicePixels = center.y - height / 2;

    let bounds: Bounds<DevicePixels> = Bounds::new(Point { x, y }, Size { width, height });
    options.bounds = Some(bounds);
    options.titlebar = Some(TitlebarOptions::default());
    options.is_movable = true;
    options.kind = WindowKind::PopUp;
    options
}

pub enum WindowEvent {
    Focus,
    Blur,
}

impl EventEmitter<WindowEvent> for ExampleWindow {}

pub struct ExampleWindow {
    focus_handle: FocusHandle,
}

impl ExampleWindow {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let focus_handle = cx.focus_handle();

        Self { focus_handle }
    }

    fn handle_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(WindowEvent::Focus);
    }

    pub fn handle_blur(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(WindowEvent::Blur);

        cx.notify();
    }
}

impl Render for ExampleWindow {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .id("example-window")
            .flex()
            .flex_col()
            .size_full()
            .justify_center()
            .items_center()
            .bg(hsla(1.0, 1.0, 1.0, 1.0))
            .text_xl()
            .text_color(hsla(0.0, 0.0, 0.0, 1.0))
            .on_click(cx.listener(|_, _event, cx| cx.focus_self()))
            .child(format!("Hello!"))
    }
}

impl FocusableView for ExampleWindow {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}
