#![allow(unused)]
use gpui::{prelude::*, *};

use crate::cursor::CursorLayout;

use super::input_state::InputState;

/// Takes a Model<Input> and builds a gpui::Element
pub struct InputElement {
    interactivity: Interactivity,
}
impl InputElement {
    pub fn new(
        input: Model<InputState>,
        focus_handle: FocusHandle,
        focused: bool,
        show_cursor: bool,
    ) -> Self {
        Self {
            interactivity: todo!(),
        }
    }
}

pub struct InputLayoutState {
    cursor: Option<CursorLayout>,
}

impl Element for InputElement {
    type RequestLayoutState = ();
    type PrepaintState = InputLayoutState;

    fn id(&self) -> Option<ElementId> {
        self.interactivity.element_id.clone()
    }

    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::RequestLayoutState) {
        todo!()
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

impl InteractiveElement for InputElement {
    fn interactivity(&mut self) -> &mut Interactivity {
        &mut self.interactivity
    }
}

impl IntoElement for InputElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
