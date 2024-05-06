#![allow(unused)]
use gpui::{prelude::*, *};

use crate::cursor::CursorLayout;

use crate::input::input_element::InputElement;
use crate::input::input_state::InputState;

pub struct Input {
    input: Model<InputState>,
    focus_handle: FocusHandle,
}

impl Input {
    fn dispatch_context(&self, cx: &mut ViewContext<Input>) {
        todo!()
    }

    fn cursor_visible(&self, focused: bool, cx: &ViewContext<Self>) -> bool {
        todo!()
    }
}

impl Render for Input {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let input_handle = self.input.clone();

        let focused = self.focus_handle.is_focused(cx);

        let child = InputElement::new(
            input_handle,
            self.focus_handle.clone(),
            focused,
            self.cursor_visible(focused, cx),
        );
        div()
            .size_full()
            .relative()
            .track_focus(&self.focus_handle)
            // .key_context(self.dispatch_context(cx))
            // .on_key_down(cx.listener(Self::key_down))
            .child(div().size_full().child(child))
        // .children(self.context_menu.as_ref().map(|(menu, position, _)| {
        //     deferred(
        //         anchored()
        //             .position(*position)
        //             .anchor(gpui::AnchorCorner::TopLeft)
        //             .child(menu.clone()),
        //     )
        //     .with_priority(1)
        // }))
    }
}

pub struct InputHandler {}
