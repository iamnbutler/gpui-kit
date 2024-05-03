#![allow(unused)]
use std::ops::DerefMut;

use gpui::prelude::FluentBuilder;
use gpui::{
    div, fill, hsla, point, px, relative, AppContext, Bounds, Context, CursorStyle, DispatchPhase,
    Edges, Element, ElementId, EventEmitter, FocusHandle, FocusableView, GlobalElementId, Hitbox,
    Hsla, InputHandler, InteractiveElement, Interactivity, IntoElement, LayoutId, Model,
    ModifiersChangedEvent, MouseButton, ParentElement, Pixels, Point, Render, SharedString, Size,
    StatefulInteractiveElement, Styled, TextRun, TextStyle, ViewContext, WindowContext,
    WrappedLine,
};
use itertools::Itertools;

use crate::cursor::CursorLayout;
use crate::style::Styles;
use crate::{color::transparent, style::Outline};

const DEBUG_INPUT_WIDTH: Pixels = px(160.);
const DEBUG_INPUT_HEIGHT: Pixels = px(24.);
const DEBUG_CELL_WIDTH: Pixels = px(5.);
const DEBUG_LINE_HEIGHT: Pixels = px(5.);

struct TextInputHandler {
    input_text: Model<Input>,
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

struct Input {
    selection: Option<std::ops::Range<usize>>,
    cursor: usize,
    value: String,
}

impl Input {
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

    fn handle_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(InputEvent::Focus);
    }

    pub fn handle_blur(&mut self, cx: &mut ViewContext<Self>) {
        cx.emit(InputEvent::Blur);
        cx.notify();
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InputSize {
    pub cell_width: Pixels,
    pub line_height: Pixels,
    pub size: Size<Pixels>,
}

impl InputSize {
    pub fn new(line_height: Pixels, cell_width: Pixels, size: Size<Pixels>) -> Self {
        InputSize {
            cell_width,
            line_height,
            size,
        }
    }

    pub fn num_lines(&self) -> usize {
        (self.size.height / self.line_height).floor() as usize
    }

    pub fn num_columns(&self) -> usize {
        (self.size.width / self.cell_width).floor() as usize
    }

    pub fn height(&self) -> Pixels {
        self.size.height
    }

    pub fn width(&self) -> Pixels {
        self.size.width
    }

    pub fn cell_width(&self) -> Pixels {
        self.cell_width
    }

    pub fn line_height(&self) -> Pixels {
        self.line_height
    }
}

impl Default for InputSize {
    fn default() -> Self {
        InputSize::new(
            DEBUG_LINE_HEIGHT,
            DEBUG_CELL_WIDTH,
            Size {
                width: DEBUG_INPUT_WIDTH,
                height: DEBUG_INPUT_HEIGHT,
            },
        )
    }
}

/// The element that GPUI paints for the input
pub struct InputElement {
    input: Model<Input>,
    focus_handle: FocusHandle,
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
    pub fn new(cx: &mut WindowContext) -> Self {
        let input = cx.new_model(|cx| Input::new());
        let focus_handle = cx.focus_handle();

        Self {
            input,
            focus_handle,
            focused: false,
            cursor_visible: true,
            interactivity: Interactivity::default(),
        }
    }

    /// Shape the text for the input using the text system
    ///
    /// We could use `shape_line` here to to shape a single line
    /// but we'll need to be able to shape multiple lines for
    /// multiline inputs eventually, so we might as
    /// well start with that.
    fn shape_text(
        &self,
        cx: &WindowContext,
    ) -> anyhow::Result<smallvec::SmallVec<[WrappedLine; 1]>> {
        // TODO: For now just hard code the input style
        let style = InputStyle::default();
        let default_text_style = style.text.clone();

        let text_system = cx.text_system();
        let text = self.input.read(cx).value.clone();
        let current_font = default_text_style.font().clone();
        let font_size = default_text_style
            .font_size
            .clone()
            .to_pixels(cx.rem_size());
        let text_run = [TextRun {
            len: text.len().clone(),
            font: cx.text_style().font().clone(),
            color: Default::default(),
            background_color: None,
            underline: None,
            strikethrough: None,
        }];

        text_system.shape_text(text.into(), font_size, &text_run, None)
    }

    // fn shape_cursor(
    //     cursor_point: DisplayCursor,
    //     size: usize,
    //     text_fragment: &ShapedLine,
    // ) -> Option<(Point<Pixels>, Pixels)> {
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

#[derive(Debug, Default)]
struct LayoutCell {
    point: Point<Pixels>,
    text: gpui::ShapedLine,
}

impl LayoutCell {
    fn new(point: Point<Pixels>, text: gpui::ShapedLine) -> LayoutCell {
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
                (origin.x + point.x * layout.dimensions.width).floor(),
                origin.y + point.y * layout.dimensions.height,
            )
        };

        self.text.paint(pos, layout.dimensions.height, cx).ok();
    }
}

#[derive(Clone, Debug, Default)]
struct LayoutRect {
    point: Point<Pixels>,
    num_of_cells: usize,
    color: Hsla,
}

impl LayoutRect {
    fn new(point: Point<Pixels>, num_of_cells: usize, color: Hsla) -> LayoutRect {
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
            let current_point = self.point;
            point(
                (origin.x + current_point.y * layout.dimensions.width).floor(),
                origin.y + current_point.y * layout.dimensions.height,
            )
        };
        let size = point(
            (layout.dimensions.width * self.num_of_cells as f32).ceil(),
            layout.dimensions.height,
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
        global_id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) -> Self::PrepaintState {
        // todo!("Implement InputElement::prepaint");
        self.interactivity
            .prepaint(global_id, bounds, bounds.size, cx, |_, _, hitbox, cx| {
                let hitbox = hitbox.unwrap();
                let styles = Styles::get_global(cx.deref_mut());

                let font_family = &styles.text.font().family.clone();
                let font_features = &styles.text.font_features.clone();
                let line_height = &styles.text.line_height;
                let font_size = &styles.text.font_size;

                let link_style = &styles.link.clone();

                let text_style = &styles.text.clone();

                let text_system = cx.text_system();

                let dimensions = {
                    let rem_size = cx.rem_size();
                    let font_pixels = text_style.font_size.to_pixels(rem_size);
                    let line_height =
                        font_pixels * line_height.to_pixels(font_pixels.into(), rem_size);
                    let font_id = cx.text_system().resolve_font(&text_style.font());

                    let cell_width = text_system
                        .advance(font_id, font_pixels, 'm')
                        .unwrap()
                        .width;

                    let mut size = bounds.size;

                    InputSize::new(line_height, cell_width, size)
                };

                let background_color = styles.background.clone();

                // let InputContent {
                //     cells,
                //     mode,
                //     display_offset,
                //     cursor_char,
                //     selection,
                //     cursor,
                //     ..
                // } = &self.terminal.read(cx).last_content;

                // then have that representation be converted to the appropriate highlight data structure

                // let (cells, rects) = TerminalElement::layout_grid(
                //     cells,
                //     &text_style,
                //     &cx.text_system(),
                //     last_hovered_word
                //         .as_ref()
                //         .map(|last_hovered_word| (link_style, &last_hovered_word.word_match)),
                //     cx,
                // );

                // Layout cursor. Rectangle is used for IME, so we should lay it out even
                // if we don't end up showing it.
                // let cursor = if let AlacCursorShape::Hidden = cursor.shape {
                //     None
                // } else {
                //     let cursor_point = DisplayCursor::from(cursor.point, *display_offset);
                //     let cursor_text = {
                //         let str_trxt = cursor_char.to_string();
                //         let len = str_trxt.len();
                //         cx.text_system()
                //             .shape_line(
                //                 str_trxt.into(),
                //                 text_style.font_size.to_pixels(cx.rem_size()),
                //                 &[TextRun {
                //                     len,
                //                     font: text_style.font(),
                //                     color: theme.colors().terminal_background,
                //                     background_color: None,
                //                     underline: Default::default(),
                //                     strikethrough: None,
                //                 }],
                //             )
                //             .unwrap()
                //     };

                let focused = self.focused;
                //     TerminalElement::shape_cursor(cursor_point, dimensions, &cursor_text).map(
                //         move |(cursor_position, block_width)| {
                //             let (shape, text) = match cursor.shape {
                //                 AlacCursorShape::Block if !focused => (CursorShape::Hollow, None),
                //                 AlacCursorShape::Block => (CursorShape::Block, Some(cursor_text)),
                //                 AlacCursorShape::Underline => (CursorShape::Underscore, None),
                //                 AlacCursorShape::Beam => (CursorShape::Bar, None),
                //                 AlacCursorShape::HollowBlock => (CursorShape::Hollow, None),
                //                 //This case is handled in the if wrapping the whole cursor layout
                //                 AlacCursorShape::Hidden => unreachable!(),
                //             };

                //             CursorLayout::new(
                //                 cursor_position,
                //                 block_width,
                //                 dimensions.line_height,
                //                 theme.players().local().cursor,
                //                 shape,
                //                 text,
                //             )
                //         },
                //     )
                // };

                // LayoutState {
                //     hitbox,
                //     cells,
                //     cursor,
                //     background_color,
                //     dimensions,
                //     rects,
                //     relative_highlighted_ranges,
                //     mode: *mode,
                //     display_offset: *display_offset,
                //     hyperlink_tooltip,
                //     gutter,
                //     last_hovered_word,
                // }

                todo!("Finish implementing InputElement::prepaint")
            })
    }

    fn paint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        cx: &mut WindowContext,
    ) {
        cx.paint_quad(fill(bounds, prepaint.background_color));
        let origin = bounds.origin;

        let input_handler = TextInputHandler {
            cursor_bounds: prepaint
                .cursor
                .as_ref()
                .map(|cursor| cursor.bounding_rect(origin)),
            input_text: self.input.clone(),
        };

        // self.register_mouse_listeners(origin, prepaint.mode, &prepaint.hitbox, cx);

        cx.set_cursor_style(gpui::CursorStyle::IBeam, &prepaint.hitbox);

        let cursor = prepaint.cursor.take();
        self.interactivity
            .paint(global_id, bounds, Some(&prepaint.hitbox), cx, |_, cx| {
                cx.handle_input(&self.focus_handle, input_handler);

                // In fact we may not need any of this for now
                // cx.on_key_event({
                //     let this = self.input.clone();
                //     move |event: &ModifiersChangedEvent, phase, cx| {
                //         if phase != DispatchPhase::Bubble {
                //             return;
                //         }

                //         // Not aure if we need this, as we don't need to watch for
                //         // clickable hyperlinks like we do in a terminal or editor
                //         //
                //         // let handled = this
                //         //     .update(cx, |input, _| input.try_modifiers_change(&event.modifiers));

                //         if handled {
                //             cx.refresh();
                //         }
                //     }
                // });

                for rect in &prepaint.rects {
                    rect.paint(origin, &prepaint, cx);
                }

                // TODO: When we have highlighted ranges, we need to paint them here
                //
                // for (relative_highlighted_range, color) in
                //     prepaint.relative_highlighted_ranges.iter()
                // {
                //     if let Some((start_y, highlighted_range_lines)) =
                //         to_highlighted_range_lines(relative_highlighted_range, &prepaint, origin)
                //     {
                //         let hr = HighlightedRange {
                //             start_y, //Need to change this
                //             line_height: prepaint.dimensions.line_height,
                //             lines: highlighted_range_lines,
                //             color: *color,
                //             //Copied from editor. TODO: move to theme or something
                //             corner_radius: 0.15 * prepaint.dimensions.line_height,
                //         };
                //         hr.paint(bounds, cx);
                //     }
                // }

                for cell in &prepaint.cells {
                    cell.paint(origin, &prepaint, bounds, cx);
                }

                if self.cursor_visible {
                    if let Some(mut cursor) = cursor {
                        cursor.paint(origin, cx);
                    }
                }
            });
    }
}

impl IntoElement for InputElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

// -- old input

// pub struct Input1 {
//     id: ElementId,
//     focus_handle: FocusHandle,
//     text: Model<Input>,
//     placeholder: Option<SharedString>,
//     style: InputStyle,
// }

// impl Input1 {
//     pub fn new(cx: &mut ViewContext<Self>, id: impl Into<ElementId>) -> Self {
//         let focus_handle = cx.focus_handle();
//         cx.on_focus(&focus_handle, Self::handle_focus).detach();
//         cx.on_blur(&focus_handle, Self::handle_blur).detach();

//         let text = cx.new_model(|_cx| Input::new());

//         Self {
//             id: id.into(),
//             focus_handle,
//             text,
//             placeholder: Some("Placeholder".into()),
//             style: InputStyle::default(),
//         }
//     }

//     pub fn set_placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
//         self.placeholder = Some(placeholder.into());
//         self
//     }

//     fn handle_focus(&mut self, cx: &mut ViewContext<Self>) {
//         cx.emit(InputEvent::Focus);
//         // self.buffer.update(cx, |buffer, cx| {});
//     }

//     pub fn handle_blur(&mut self, cx: &mut ViewContext<Self>) {
//         cx.emit(InputEvent::Blur);
//         cx.notify();
//     }

//     pub fn is_focused(&self, cx: &ViewContext<Self>) -> bool {
//         cx.focused() == Some(self.focus_handle.clone())
//     }
// }

// impl Render for Input1 {
//     fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
//         let current_text = self.text.read(cx).value.clone();

//         let padding_inset = 1.0;
//         let padding = if let Some(ring) = self.style.ring.clone() {
//             ring.width + padding_inset
//         } else {
//             2.0 + padding_inset
//         };

//         let height = 32.0;
//         let calculated_height = height - padding * 2.0;

//         let width = 188.0;
//         let calculated_width = width - padding * 2.0;

//         match self.is_focused(cx) {
//             true => {
//                 self.style.ring = Some(Outline::new(hsla(0.6, 0.67, 0.46, 1.0)));
//             }
//             false => {
//                 self.style.ring = None;
//             }
//         }

//         div()
//             .id(self.id.clone())
//             .group("input")
//             .track_focus(&self.focus_handle)
//             .key_context("input")
//             .on_mouse_down(MouseButton::Left, |_, cx| cx.stop_propagation())
//             .on_click(cx.listener(|_, _event, cx| cx.focus_self()))
//             .relative()
//             .flex()
//             .h(px(calculated_height))
//             // TODO: Width should be dynamic
//             // need to be able to read the width of the input
//             .w(px(calculated_width))
//             .overflow_hidden()
//             .cursor(CursorStyle::IBeam)
//             .p(px(padding_inset))
//             .border_2()
//             .border_color(transparent())
//             .when_some(self.style.ring.clone(), |this, ring| {
//                 this.when(ring.width > 0.0, |this| this)
//                     .border_color(ring.color)
//                     .rounded(px(ring.radius))
//             })
//             .child(
//                 div()
//                     .id("input_inner")
//                     .absolute()
//                     .flex()
//                     .h(px(calculated_height - padding_inset * 2.0))
//                     .w(px(calculated_width - padding_inset * 2.0))
//                     .top(px(-padding_inset))
//                     .left(px(-padding_inset))
//                     .items_center()
//                     .bg(self.style.background)
//                     .when(self.style.border.width > 0.0, |this| this.border())
//                     .border_color(self.style.border.color)
//                     .rounded(px(self.style.border.radius))
//                     .overflow_hidden()
//                     .bg(self.style.background)
//                     .text_color(self.style.text.color)
//                     // .font(self.style.text.font_family.clone())
//                     .text_size(self.style.text.font_size)
//                     .group_hover("input", |this| this.border_color(hsla(0.0, 0.0, 0.31, 1.0)))
//                     .child(
//                         div()
//                             .relative()
//                             .pl(px(self.style.padding.left))
//                             .pr(px(self.style.padding.right))
//                             .pt(px(self.style.padding.top))
//                             .pb(px(self.style.padding.bottom))
//                             .child(if let Some(placeholder) = self.placeholder.clone() {
//                                 if current_text.is_empty() {
//                                     placeholder
//                                 } else {
//                                     current_text.into()
//                                 }
//                             } else {
//                                 current_text.into()
//                             }),
//                     ),
//             )
//     }
// }

// impl FocusableView for Input1 {
//     fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
//         self.focus_handle.clone()
//     }
// }
