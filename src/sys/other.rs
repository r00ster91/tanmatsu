//! Terminal implementation for all non-Redox operating systems.

use crate::{
    event::{Event, KeyEvent, KeyModifier, MouseButton, MouseEvent, MouseEventKind},
    util::{Color, Point, Size},
    Terminal,
};
use crossterm::{cursor, event, style, terminal, QueueableCommand};

// TODO: return result instead of unwrapping

impl<'a> Terminal<'a> {
    pub fn enter_alternate_dimension(&mut self) {
        self.handle.queue(terminal::EnterAlternateScreen).unwrap();
    }
    pub fn exit_alternate_dimension(&mut self) {
        self.handle.queue(terminal::LeaveAlternateScreen).unwrap();
    }

    pub fn set_title(&mut self, title: &str) {
        self.handle.queue(terminal::SetTitle(title)).unwrap();
    }

    pub fn enable_raw_mode(&self) {
        terminal::enable_raw_mode().unwrap();
    }
    pub fn disable_raw_mode(&self) {
        terminal::disable_raw_mode().unwrap();
    }

    pub fn enable_mouse_capture(&mut self) {
        self.handle.queue(event::EnableMouseCapture).unwrap();
    }
    pub fn disable_mouse_capture(&mut self) {
        self.handle.queue(event::DisableMouseCapture).unwrap();
    }

    pub fn show_cursor(&mut self) {
        self.handle.queue(cursor::Show).unwrap();
    }
    pub fn hide_cursor(&mut self) {
        self.handle.queue(cursor::Hide).unwrap();
    }

    /// Reads an event. It also sets the new size if the terminal has been resized.
    pub fn read_event(&mut self) -> Option<Event> {
        let crossterm_event = Terminal::read();
        let event = match crossterm_event {
            event::Event::Mouse(event) => match event.kind {
                event::MouseEventKind::Moved => Event::Mouse(MouseEvent {
                    kind: MouseEventKind::Move,
                    point: Point {
                        x: event.column,
                        y: event.row,
                    },
                }),
                event::MouseEventKind::Drag(button) => {
                    let button = match button {
                        event::MouseButton::Left => MouseButton::Left,
                        event::MouseButton::Middle => MouseButton::Middle,
                        event::MouseButton::Right => MouseButton::Right,
                    };
                    Event::Mouse(MouseEvent {
                        kind: MouseEventKind::Drag(button),
                        point: Point {
                            x: event.column,
                            y: event.row,
                        },
                    })
                }
                event::MouseEventKind::Down(button) => {
                    let button = match button {
                        event::MouseButton::Left => MouseButton::Left,
                        event::MouseButton::Middle => MouseButton::Middle,
                        event::MouseButton::Right => MouseButton::Right,
                    };
                    Event::Mouse(MouseEvent {
                        kind: MouseEventKind::Press(button),
                        point: Point {
                            x: event.column,
                            y: event.row,
                        },
                    })
                }
                event::MouseEventKind::Up(button) => {
                    let button = match button {
                        event::MouseButton::Left => MouseButton::Left,
                        event::MouseButton::Middle => MouseButton::Middle,
                        event::MouseButton::Right => MouseButton::Right,
                    };
                    Event::Mouse(MouseEvent {
                        kind: MouseEventKind::Release(button),
                        point: Point {
                            x: event.column,
                            y: event.row,
                        },
                    })
                }
                event::MouseEventKind::ScrollUp => Event::Mouse(MouseEvent {
                    kind: MouseEventKind::ScrollUp,
                    point: Point {
                        x: event.column,
                        y: event.row,
                    },
                }),
                event::MouseEventKind::ScrollDown => Event::Mouse(MouseEvent {
                    kind: MouseEventKind::ScrollDown,
                    point: Point {
                        x: event.column,
                        y: event.row,
                    },
                }),
            },
            event::Event::Key(event::KeyEvent { code, modifiers }) => match code {
                event::KeyCode::Tab => Event::Key(KeyEvent::Tab),
                event::KeyCode::Char('w') if modifiers == event::KeyModifiers::CONTROL => {
                    Event::Key(KeyEvent::Backspace(Some(KeyModifier::Control)))
                }
                event::KeyCode::Char(key) => {
                    if modifiers == event::KeyModifiers::CONTROL {
                        Event::Key(KeyEvent::Char(key, Some(KeyModifier::Control)))
                    } else {
                        Event::Key(KeyEvent::Char(key, None))
                    }
                }
                event::KeyCode::Esc => Event::Key(KeyEvent::Esc),
                event::KeyCode::Backspace => Event::Key(KeyEvent::Backspace(None)),
                event::KeyCode::Left if modifiers == event::KeyModifiers::CONTROL => {
                    Event::Key(KeyEvent::Left(Some(KeyModifier::Control)))
                }
                event::KeyCode::Right if modifiers == event::KeyModifiers::CONTROL => {
                    Event::Key(KeyEvent::Right(Some(KeyModifier::Control)))
                }
                event::KeyCode::Up => Event::Key(KeyEvent::Up),
                event::KeyCode::Down => Event::Key(KeyEvent::Down),
                event::KeyCode::Left => Event::Key(KeyEvent::Left(None)),
                event::KeyCode::Right => Event::Key(KeyEvent::Right(None)),
                event::KeyCode::Enter => Event::Key(KeyEvent::Enter),
                _ => return None,
            },
            event::Event::Resize(width, height) => {
                self.size = Size::new(width, height);
                Event::Resize
            }
        };
        Some(event)
    }

    pub fn set_cursor(&mut self, point: Point) {
        self.handle.queue(cursor::MoveTo(point.x, point.y)).unwrap();
    }

    pub fn move_cursor_left(&mut self, cells: u16) {
        self.handle.queue(cursor::MoveLeft(cells)).unwrap();
    }
    pub fn move_cursor_right(&mut self, cells: u16) {
        self.handle.queue(cursor::MoveRight(cells)).unwrap();
    }
    pub fn move_cursor_up(&mut self, cells: u16) {
        self.handle.queue(cursor::MoveUp(cells)).unwrap();
    }
    pub fn move_cursor_down(&mut self, cells: u16) {
        self.handle.queue(cursor::MoveDown(cells)).unwrap();
    }

    // benchmark if implementing this without escape sequences is faster
    pub fn save_cursor_point(&mut self) {
        self.handle.queue(cursor::SavePosition).unwrap();
    }
    pub fn restore_cursor_point(&mut self) {
        self.handle.queue(cursor::RestorePosition).unwrap();
    }

    pub fn save_cursor_x(&mut self) {
        unimplemented!();
    }
    pub fn restore_cursor_y(&mut self) {
        unimplemented!();
    }

    pub fn set_foreground_color(&mut self, color: Color) {
        self.handle
            .queue(style::SetForegroundColor(Self::convert_color(color)))
            .unwrap();
    }
    pub fn set_background_color(&mut self, color: Color) {
        self.handle
            .queue(style::SetBackgroundColor(Self::convert_color(color)))
            .unwrap();
    }

    pub fn enable_italic(&mut self) {
        self.write_args(format_args!("{}", style::Attribute::Italic));
    }
    pub fn disable_italic(&mut self) {
        self.write_args(format_args!("{}", style::Attribute::NoItalic));
    }

    fn convert_color(color: Color) -> style::Color {
        match color {
            Color::Black => style::Color::Black,
            Color::DarkGray => style::Color::DarkGrey,
            Color::Red => style::Color::Red,
            Color::DarkRed => style::Color::DarkRed,
            Color::Green => style::Color::Green,
            Color::DarkGreen => style::Color::DarkGreen,
            Color::Yellow => style::Color::Yellow,
            Color::DarkYellow => style::Color::DarkYellow,
            Color::Blue => style::Color::Blue,
            Color::DarkBlue => style::Color::DarkBlue,
            Color::Magenta => style::Color::Magenta,
            Color::DarkMagenta => style::Color::DarkMagenta,
            Color::Cyan => style::Color::Cyan,
            Color::DarkCyan => style::Color::DarkCyan,
            Color::White => style::Color::White,
            Color::Gray => style::Color::Grey,
            Color::Rgb { r, g, b } => style::Color::Rgb { r, g, b },
            Color::Byte(rgb) => style::Color::AnsiValue(rgb),
        }
    }

    pub fn reset_colors(&mut self) {
        self.handle.queue(style::ResetColor).unwrap();
    }

    // TODO: these clear methods should probably never be needed.
    pub fn clear(&mut self) {
        self.handle
            .queue(terminal::Clear(terminal::ClearType::All))
            .unwrap();
    }
    pub fn clear_from_cursor_to_end(&mut self) {
        self.handle
            .queue(terminal::Clear(terminal::ClearType::FromCursorUp))
            .unwrap();
    }

    pub fn size() -> Size {
        let size = terminal::size().expect("retrieving terminal size failed");
        Size::new(size.0, size.1)
    }

    pub fn read() -> event::Event {
        crossterm::event::read().expect("reading event failed")
    }
}
