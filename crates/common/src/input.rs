use gpui::prelude::FluentBuilder;
use gpui::{
    div, fill, hsla, point, px, relative, AppContext, Bounds, CursorStyle, Edges, Element,
    ElementId, EventEmitter, FocusHandle, FocusableView, GlobalElementId, Hitbox, Hsla,
    InputHandler, InteractiveElement, Interactivity, IntoElement, LayoutId, Model, MouseButton,
    ParentElement, Pixels, Point, Render, SharedString, Size, StatefulInteractiveElement, Styled,
    TextStyle, ViewContext, WindowContext,
};
use itertools::Itertools;
use std::mem;
use std::{fmt::Debug, ops::RangeInclusive};

use crate::cursor::CursorLayout;
use crate::{color::transparent, style::Outline};

// Input -----
//
// Text Input Handler - Handles text input for the inputs
//  - We can't use "TextInput" here as it's a reserved name for the trait
//
// Input - This carries the state
//  - text, focus, cursor position, etc
//
// Input Element -
//
// Input Style
//
// -----

struct TextInputHandler {
    input_text: Model<InputText>,
    // workspace: WeakView<Workspace>,
    cursor_bounds: Option<Bounds<Pixels>>,
}

impl InputHandler for TextInputHandler {
    fn selected_text_range(&mut self, cx: &mut WindowContext) -> Option<std::ops::Range<usize>> {
        Some(0..0)
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
        self.input_text.update(cx, |input_text, _| {
            input_text.set_value(text);
        });
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

struct InputText {
    selection: Option<std::ops::Range<usize>>,
    cursor: usize,
    value: String,
}

impl InputText {
    pub fn new() -> Self {
        Self {
            selection: None,
            cursor: 0,
            value: String::new(),
        }
    }

    pub fn set_value(&mut self, value: impl Into<String>) {
        self.value = value.into();
    }
}

pub struct Input {
    id: ElementId,
    focus_handle: FocusHandle,
    text: Model<InputText>,
    placeholder: Option<SharedString>,
    style: InputStyle,
}

impl Input {
    pub fn new(cx: &mut ViewContext<Self>, id: impl Into<ElementId>) -> Self {
        let focus_handle = cx.focus_handle();
        cx.on_focus(&focus_handle, Self::handle_focus).detach();
        cx.on_blur(&focus_handle, Self::handle_blur).detach();

        let text = cx.new_model(|_cx| InputText::new());

        Self {
            id: id.into(),
            focus_handle,
            text,
            placeholder: Some("Placeholder".into()),
            style: InputStyle::default(),
        }
    }

    pub fn set_placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

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
        let current_text = self.text.read(cx).value.clone();

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
                    // .font(self.style.text.font_family.clone())
                    .text_size(self.style.text.font_size)
                    .group_hover("input", |this| this.border_color(hsla(0.0, 0.0, 0.31, 1.0)))
                    .child(
                        div()
                            .relative()
                            .pl(px(self.style.padding.left))
                            .pr(px(self.style.padding.right))
                            .pt(px(self.style.padding.top))
                            .pb(px(self.style.padding.bottom))
                            .child(if let Some(placeholder) = self.placeholder.clone() {
                                if current_text.is_empty() {
                                    placeholder
                                } else {
                                    current_text.into()
                                }
                            } else {
                                current_text.into()
                            }),
                    ),
            )
    }
}

impl FocusableView for Input {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

/// The element that GPUI paints for the input
pub struct InputElement {
    input: Model<Input>,
    // Do we need workspace? We don't have access to this
    // since it isn't part of gpui
    // workspace: WeakView<Workspace>,
    focus: FocusHandle,
    focused: bool,
    cursor_visible: bool,
    interactivity: Interactivity,
}

impl InteractiveElement for InputElement {
    fn interactivity(&mut self) -> &mut Interactivity {
        &mut self.interactivity
    }
}

impl StatefulInteractiveElement for InputElement {}

impl InputElement {
    // pub fn new(
    //     input: Model<Input>,
    //     workspace: WeakView<Workspace>,
    //     focus: FocusHandle,
    //     focused: bool,
    //     cursor_visible: bool,
    // ) -> TerminalElement {
    // }

    // fn layout_grid(
    //     grid: &Vec<IndexedCell>,
    //     text_style: &TextStyle,
    //     text_system: &WindowTextSystem,
    //     hyperlink: Option<(HighlightStyle, &Range<usize>)>,
    //     cx: &WindowContext<'_>,
    // ) -> (Vec<LayoutCell>, Vec<LayoutRect>) {
    // }

    // fn shape_cursor(
    //     cursor_point: DisplayCursor,
    //     size: usize,
    //     text_fragment: &ShapedLine,
    // ) -> Option<(Point<Pixels>, Pixels)> {
    // }

    // fn cell_style(
    //     indexed: &IndexedCell,
    //     fg: terminal::alacritty_terminal::vte::ansi::Color,
    //     // bg: terminal::alacritty_terminal::ansi::Color,
    //     colors: &Theme,
    //     text_style: &TextStyle,
    //     hyperlink: Option<(HighlightStyle, &RangeInclusive<AlacPoint>)>,
    // ) -> TextRun {
    // }

    // fn generic_button_handler<E>(
    //     connection: Model<Terminal>,
    //     origin: Point<Pixels>,
    //     focus_handle: FocusHandle,
    //     f: impl Fn(&mut Terminal, Point<Pixels>, &E, &mut ModelContext<Terminal>),
    // ) -> impl Fn(&E, &mut WindowContext) {
    // }

    // fn register_mouse_listeners(
    //     &mut self,
    //     origin: Point<Pixels>,
    //     mode: TermMode,
    //     hitbox: &Hitbox,
    //     cx: &mut WindowContext,
    // ) {
    // }
}

/// The information generated during layout that is necessary for painting.
pub struct LayoutState {
    hitbox: Hitbox,
    cells: Vec<LayoutCell>,
    rects: Vec<LayoutRect>,
    cursor: Option<CursorLayout>,
    background_color: Hsla,
    dimensions: Size<Pixels>,
    display_offset: usize,
}

/// Helper struct for converting data between Alacritty's cursor points, and displayed cursor points.
struct DisplayCursor {
    line: i32,
    col: usize,
}

impl DisplayCursor {
    fn from(cursor_point: Point<f32>, display_offset: usize) -> Self {
        Self {
            line: cursor_point.line.0 + display_offset as i32,
            col: cursor_point.column.0,
        }
    }

    pub fn line(&self) -> i32 {
        self.line
    }

    pub fn col(&self) -> usize {
        self.col
    }
}

#[derive(Debug, Default)]
struct LayoutCell {
    point: Point<i32>,
    text: gpui::ShapedLine,
}

impl LayoutCell {
    fn new(point: Point<i32>, text: gpui::ShapedLine) -> LayoutCell {
        LayoutCell { point, text }
    }

    fn paint(
        &self,
        origin: Point<Pixels>,
        layout: &LayoutState,
        _visible_bounds: Bounds<Pixels>,
        cx: &mut WindowContext,
    ) {
        let pos = {
            let point = self.point;

            Point::new(
                (origin.x + point.column as f32 * layout.dimensions.cell_width).floor(),
                origin.y + point.line as f32 * layout.dimensions.line_height,
            )
        };

        self.text.paint(pos, layout.dimensions.line_height, cx).ok();
    }
}

#[derive(Clone, Debug, Default)]
struct LayoutRect {
    point: Point<i32>,
    num_of_cells: usize,
    color: Hsla,
}

impl LayoutRect {
    fn new(point: Point<i32>, num_of_cells: usize, color: Hsla) -> LayoutRect {
        LayoutRect {
            point,
            num_of_cells,
            color,
        }
    }

    fn extend(&self) -> Self {
        LayoutRect {
            point: self.point,
            num_of_cells: self.num_of_cells + 1,
            color: self.color,
        }
    }

    fn paint(&self, origin: Point<Pixels>, layout: &LayoutState, cx: &mut WindowContext) {
        let position = {
            let point = self.point;
            point(
                (origin.x + point.column as f32 * layout.dimensions.cell_width).floor(),
                origin.y + point.line as f32 * layout.dimensions.line_height,
            )
        };
        let size = point(
            (layout.dimensions.cell_width * self.num_of_cells as f32).ceil(),
            layout.dimensions.line_height,
        )
        .into();

        cx.paint_quad(fill(Bounds::new(position, size), self.color));
    }
}

impl Element for InputElement {
    type RequestLayoutState = ();
    type PrepaintState = LayoutState;

    fn id(&self) -> Option<ElementId> {
        self.interactivity.element_id.clone()
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::RequestLayoutState) {
        self.interactivity.occlude_mouse();
        let layout_id = self
            .interactivity
            .request_layout(global_id, cx, |mut style, cx| {
                style.size.width = relative(1.).into();
                style.size.height = relative(1.).into();
                let layout_id = cx.request_layout(&style, None);

                layout_id
            });
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) -> Self::PrepaintState {
        todo!()
    }

    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        cx: &mut WindowContext,
    ) {
        todo!()
    }
}
