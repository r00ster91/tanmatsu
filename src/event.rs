//! Terminal events defined specific to usage.

use crate::util::Point;

#[derive(Debug)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(Debug)]
pub enum MouseEventKind {
    ScrollUp,
    ScrollDown,
    Move,
    Drag(MouseButton),
    Press(MouseButton),
    Release(MouseButton),
}

#[derive(Debug)]
pub struct MouseEvent {
    pub kind: MouseEventKind,
    pub point: Point,
}

#[derive(Debug)]
pub enum KeyModifier {
    Control,
}

#[derive(Debug)]
pub enum KeyEvent {
    Up,
    Down,
    Left,
    Right,
    Char(char, Option<KeyModifier>),
    Tab,
    Esc,
    Backspace(Option<KeyModifier>),
    Enter,
    F(u8),
}

#[derive(Debug)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    /// No `Size` included. Call [`crate::Terminal::size`] instead.
    Resize,
}
