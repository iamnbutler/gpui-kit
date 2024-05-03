use gpui::*;

use crate::style::Styles;

actions!(
    focus,
    [
        MoveLeft,
        MoveRight,
        Delete,
        MoveToStart,
        MoveToEnd,
        SelectAll,
        Copy,
        Cut,
        Paste,
        Undo,
        Redo
    ]
);

/// Initialzes app resources
pub fn init_app(cx: &mut AppContext) {
    bind_keys(cx);
}

/// Initialzes styles
pub fn init_window(cx: &mut WindowContext) {
    cx.set_global(Styles::init(cx));
}

/// Bind core actions to the keymap
pub fn bind_keys(cx: &mut AppContext) {
    cx.bind_keys([KeyBinding::new("left", MoveLeft, Some("input"))]);
    cx.bind_keys([KeyBinding::new("right", MoveRight, Some("input"))]);
    cx.bind_keys([KeyBinding::new("delete", Delete, Some("input"))]);
    cx.bind_keys([KeyBinding::new("home", MoveToStart, Some("input"))]);
    cx.bind_keys([KeyBinding::new("end", MoveToEnd, Some("input"))]);
    cx.bind_keys([KeyBinding::new("cmd+a", SelectAll, Some("input"))]);
    cx.bind_keys([KeyBinding::new("cmd+c", Copy, Some("input"))]);
    cx.bind_keys([KeyBinding::new("cmd+x", Cut, Some("input"))]);
    cx.bind_keys([KeyBinding::new("cmd+v", Paste, Some("input"))]);
    cx.bind_keys([KeyBinding::new("cmd+z", Undo, Some("input"))]);
    cx.bind_keys([KeyBinding::new("cmd+shift+z", Redo, Some("input"))]);
}
