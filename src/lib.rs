pub mod event;
mod sys;
pub mod util;

use crate::util::{Point, Size};
use std::io::{self, Write};

// TODO: maybe do flushing on drop

// TODO: add `error` to abort program with message?

// TODO: return a result instead of `expect`ing?

// Once https://github.com/rust-lang/rust/pull/78515 is merged, some of this can be changed
pub struct Terminal {
    // This cannot be an `io::StdoutLock` and permanently lock because that doesn't work well with threads
    pub stdout: io::Stdout,
    pub size: Size,
    #[cfg(debug_assertions)]
    pub flush_count: usize,
    #[cfg(debug_assertions)]
    initialized: bool,
}

// TODO: for better panicking: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=018af4b20094fd7ec0d4cca21d6ff2a8

impl Default for Terminal {
    fn default() -> Self {
        Self::new()
    }
}

/// A terminal with an `io::Stdout` inside.
///
/// Optimally every program should have a single instance.
impl Terminal {
    pub fn new() -> Self {
        Self {
            stdout: io::stdout(),
            size: Self::size(),
            #[cfg(debug_assertions)]
            flush_count: 0,
            #[cfg(debug_assertions)]
            initialized: false,
        }
    }

    pub fn write(&mut self, string: &str) {
        self.stdout
            .write_all(string.as_bytes())
            .expect("write to stdout failed");
    }

    pub fn flush(&mut self) {
        self.stdout.flush().expect("flushing failed");

        #[cfg(debug_assertions)]
        {
            if self.initialized {
                self.flush_count += 1;
                self.save_cursor_point();
                self.set_cursor(Point { x: 0, y: 0 });
                let flush_count = self.flush_count;
                self.write(&format!("Flush count: {}", flush_count));
                self.restore_cursor_point();
            }
        }
    }

    pub fn get_centered_border_point(&self, size: &Size) -> Point {
        Point {
            x: self.size.width / 2 - size.width / 2,
            y: self.size.height / 2 - size.height / 2,
        }
    }

    pub fn get_center(&self) -> Point {
        Point {
            x: self.size.width / 2,
            y: self.size.height,
        }
    }

    pub fn initialize(&mut self) {
        self.enter_alternate_dimension();
        self.enable_raw_mode();
        self.enable_mouse_capture();
        self.hide_cursor();
        self.flush();

        #[cfg(debug_assertions)]
        {
            self.initialized = true;
        }
    }

    pub fn deinitialize(&mut self) {
        self.show_cursor();
        self.disable_mouse_capture();
        self.disable_raw_mode();
        self.exit_alternate_dimension();
        self.flush();

        #[cfg(debug_assertions)]
        {
            self.initialized = false;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
