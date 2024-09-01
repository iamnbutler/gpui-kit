use gpui::*;
use state::{Disableable, Draggable, Hoverable, Pressable, Selectable, Selected};

#[derive(Debug, Clone, IntoElement)]
pub struct Checkbox {}

impl Disableable for Checkbox {
    fn disabled(self) -> bool {
        todo!()
    }

    fn set_disabled(&mut self, focused: bool) {
        todo!()
    }

    fn on_disable(&mut self, handler: impl 'static + Fn(&mut WindowContext) + Send + Sync) -> Self {
        todo!()
    }

    fn on_enable(&mut self, handler: impl 'static + Fn(&mut WindowContext) + Send + Sync) -> Self {
        todo!()
    }
}

impl Draggable for Checkbox {
    fn dragged(self) -> bool {
        todo!()
    }

    fn set_dragged(&mut self, dragged: bool) {
        todo!()
    }

    fn on_drag(&mut self, handler: impl 'static + Fn(&mut WindowContext) + Send + Sync) -> Self {
        todo!()
    }

    fn on_drop(&mut self, handler: impl 'static + Fn(&mut WindowContext) + Send + Sync) -> Self {
        todo!()
    }
}

impl Hoverable for Checkbox {
    fn hovered(self) -> bool {
        todo!()
    }

    fn set_hovered(&mut self, focused: bool) {
        todo!()
    }
}

impl Pressable for Checkbox {
    fn pressed(self) -> bool {
        todo!()
    }

    fn set_pressed(&mut self, focused: bool) {
        todo!()
    }
}

impl Selectable for Checkbox {
    fn selected(self) -> Selected {
        todo!()
    }

    fn set_selected(&mut self, selected: Selected) {
        todo!()
    }
}

impl Checkbox {
    pub fn new(id: impl Into<ElementId>, selected: Selected) -> Self {
        unimplemented!()
    }

    pub fn on_click(
        mut self,
        handler: impl 'static + Fn(&Selected, &mut WindowContext) + Send + Sync,
    ) -> Self {
        unimplemented!()
    }
}

impl RenderOnce for Checkbox {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        div()
    }
}
