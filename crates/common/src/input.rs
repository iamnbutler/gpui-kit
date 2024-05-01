use gpui::{prelude::*, *};

use crate::{color::transparent, style::Outline};

struct TextInputHandler {
    input: Model<Input>,
    // workspace: WeakView<Workspace>,
    cursor_bounds: Option<Bounds<Pixels>>,
}

impl InputHandler for TextInputHandler {
    fn selected_text_range(&mut self, cx: &mut WindowContext) -> Option<std::ops::Range<usize>> {
        None
    }

    fn marked_text_range(&mut self, _: &mut WindowContext) -> Option<std::ops::Range<usize>> {
        None
    }

    fn text_for_range(
        &mut self,
        _: std::ops::Range<usize>,
        _: &mut WindowContext,
    ) -> Option<String> {
        None
    }

    fn replace_text_in_range(
        &mut self,
        _replacement_range: Option<std::ops::Range<usize>>,
        text: &str,
        cx: &mut WindowContext,
    ) {
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        _range_utf16: Option<std::ops::Range<usize>>,
        _new_text: &str,
        _new_selected_range: Option<std::ops::Range<usize>>,
        _: &mut WindowContext,
    ) {
    }

    fn unmark_text(&mut self, _: &mut WindowContext) {}

    fn bounds_for_range(
        &mut self,
        _range_utf16: std::ops::Range<usize>,
        _: &mut WindowContext,
    ) -> Option<Bounds<Pixels>> {
        self.cursor_bounds
    }
}

pub enum InputEvent {
    Focus,
    Blur,
}

impl EventEmitter<InputEvent> for Input {}

#[derive(Clone)]
pub struct InputStyle {
    pub background: Hsla,
    pub padding: Edges<f32>,
    pub margin: Edges<f32>,
    pub ring: Option<Outline>,
    pub border: Outline,
    pub text: TextStyle,
}

impl Default for InputStyle {
    fn default() -> Self {
        Self {
            background: hsla(0.0, 0.0, 1.0, 1.0),
            padding: Edges {
                top: 0.0,
                bottom: 0.0,
                left: 4.0,
                right: 4.0,
            },
            margin: Edges::all(0.0),
            ring: None,
            border: Outline::new(hsla(0.0, 0.0, 0.31, 0.4)),
            text: TextStyle::default(),
        }
    }
}

pub struct Input {
    id: ElementId,
    focus_handle: FocusHandle,
    selection: Option<std::ops::Range<usize>>,
    cursor: usize,
    text: String,
    placeholder: Option<SharedString>,
    style: InputStyle,
}

impl Input {
    pub fn new(cx: &mut ViewContext<Self>, id: impl Into<ElementId>) -> Self {
        let focus_handle = cx.focus_handle();
        cx.on_focus(&focus_handle, Self::handle_focus).detach();
        cx.on_blur(&focus_handle, Self::handle_blur).detach();

        Self {
            id: id.into(),
            focus_handle,
            text: "".into(),
            cursor: 0,
            placeholder: None,
            selection: None,
            style: InputStyle::default(),
        }
    }

    pub fn set_placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    // pub fn set_text(mut self, cx: &ViewContext<Self>, text: String) -> Self {
    //     if text.find('\n').is_some() {
    //         panic!("Input text cannot contain newlines as it is a single line editor, this is a limitation of gpui")
    //     }

    //     let rem_size = cx.rem_size();
    //     let font_size_in_px: Pixels = self.style.text.font_size.to_pixels(rem_size);

    //     let shaped_line = cx
    //         .text_system()
    //         .shape_line(text.into(), font_size_in_px, &vec![]);

    //     self.text = Some(shaped_line.expect("something went wrong shaping the line"));
    //     self
    // }

    // pub fn value(&self, cx: &ViewContext<Self>) -> SharedString {
    //     if let Some(line) = self.text.as_ref() {
    //         line.text.clone()
    //     } else {
    //         "".into()
    //     }
    // }

    fn handle_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(InputEvent::Focus);
        // self.buffer.update(cx, |buffer, cx| {});
    }

    pub fn handle_blur(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(InputEvent::Blur);
        cx.notify();
    }

    pub fn is_focused(&self, cx: &ViewContext<Self>) -> bool {
        cx.focused() == Some(self.focus_handle.clone())
    }
}

impl Render for Input {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let text = self.text.clone();

        // == Size ==
        let padding_inset = 1.0;
        let padding = if let Some(ring) = self.style.ring.clone() {
            ring.width + padding_inset
        } else {
            2.0 + padding_inset
        };

        let height = 32.0;
        let calculated_height = height - padding * 2.0;

        let width = 188.0;
        let calculated_width = width - padding * 2.0;

        match self.is_focused(cx) {
            true => {
                self.style.ring = Some(Outline::new(hsla(0.6, 0.67, 0.46, 1.0)));
            }
            false => {
                self.style.ring = None;
            }
        }

        div()
            .id(self.id.clone())
            .group("input")
            .track_focus(&self.focus_handle)
            .key_context("input")
            .on_mouse_down(MouseButton::Left, |_, cx| cx.stop_propagation())
            .on_click(cx.listener(|_, _event, cx| cx.focus_self()))
            .relative()
            .flex()
            .h(px(calculated_height))
            // TODO: Width should be dynamic
            // need to be able to read the width of the input
            .w(px(calculated_width))
            .overflow_hidden()
            .cursor(CursorStyle::IBeam)
            .p(px(padding_inset))
            .border_2()
            .border_color(transparent())
            .when_some(self.style.ring.clone(), |this, ring| {
                this.when(ring.width > 0.0, |this| this)
                    .border_color(ring.color)
                    .rounded(px(ring.radius))
            })
            .child(
                div()
                    .id("input_inner")
                    .absolute()
                    .flex()
                    .h(px(calculated_height - padding_inset * 2.0))
                    .w(px(calculated_width - padding_inset * 2.0))
                    .top(px(-padding_inset))
                    .left(px(-padding_inset))
                    .items_center()
                    .bg(self.style.background)
                    .when(self.style.border.width > 0.0, |this| this.border())
                    .border_color(self.style.border.color)
                    .rounded(px(self.style.border.radius))
                    .overflow_hidden()
                    .bg(self.style.background)
                    .text_color(self.style.text.color)
                    .font(self.style.text.font_family.clone())
                    .text_size(self.style.text.font_size)
                    .group_hover("input", |this| this.border_color(hsla(0.0, 0.0, 0.31, 1.0)))
                    .child(
                        div()
                            .relative()
                            .pl(px(self.style.padding.left))
                            .pr(px(self.style.padding.right))
                            .pt(px(self.style.padding.top))
                            .pb(px(self.style.padding.bottom))
                            .child(text),
                    ),
            )
    }
}

impl FocusableView for Input {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}
