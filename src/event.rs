//! Terminal events defined specific to usage.

use crate::util::{Point, };

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

pub struct MouseEvent {
    pub kind: MouseEventKind,
    pub point: Point,
}

pub enum KeyModifier {
    Control,
}

pub enum KeyEvent {
    Up,
    Down,
    Left(Option<KeyModifier>),
    Right(Option<KeyModifier>),
    Char(char, Option<KeyModifier>),
    Tab,
    Esc,
    Backspace(Option<KeyModifier>),
    Enter,
}

pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    /// No `Size` included. Call [`crate::Terminal::size`] instead.
    Resize,
}
