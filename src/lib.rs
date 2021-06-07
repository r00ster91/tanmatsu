pub mod event;
mod sys;
pub mod util;

use crossterm::tty::IsTty;

use crate::util::{Point, Size};
use std::io::{self, Write};

// TODO: add `error` to abort program with message?

// TODO: return a result instead of `expect`ing?

// Once https://github.com/rust-lang/rust/pull/78515 is merged, some of this can be changed
pub struct Terminal<'a> {
    pub stdout: io::BufWriter<io::StdoutLock<'a>>,
    pub size: Size,
    #[cfg(debug_assertions)]
    pub flush_count: usize,
    #[cfg(debug_assertions)]
    initialized: bool,
}

#[derive(Debug)]
#[non_exhaustive] // Prevent instantiation
pub struct NotTTY;

/// A terminal with an `io::Stdout` inside.
///
/// Every program can have only a single instance for writing.
/// The standard output stream is locked and no other instance can write.
impl<'a> Terminal<'a> {
    pub fn new(stdout: io::StdoutLock<'a>) -> Result<Self, NotTTY> {
        if !stdout.is_tty() {
            return Err(NotTTY);
        }

        let writer = io::BufWriter::new(stdout);

        Ok(Self {
            stdout: writer,
            size: Self::size(),
            #[cfg(debug_assertions)]
            flush_count: 0,
            #[cfg(debug_assertions)]
            initialized: false,
        })
    }

    pub fn write(&mut self, string: &str) {
        self.stdout
            .write_all(string.as_bytes())
            .expect("write to stdout failed");
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.stdout
            .write_all(bytes)
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

    fn set_panic_hook() {
        use std::panic;

        let default_panic_hook = panic::take_hook();

        panic::set_hook(Box::new(move |panic_info| {
            let stdout = io::stdout();
            if let Ok(mut terminal) = Terminal::new(stdout.lock()) {
                terminal.deinitialize();
            }
            default_panic_hook(panic_info);
        }));
    }

    /// Makes the terminal ready for drawing and input
    /// and registers a panic hook that makes sure [`deinitialize`] is called before the panic output.
    pub fn initialize(&mut self) {
        self.enter_alternate_dimension();
        self.enable_raw_mode();
        self.enable_mouse_capture();
        self.hide_cursor();
        self.flush();

        Self::set_panic_hook();

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
