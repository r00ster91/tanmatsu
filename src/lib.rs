pub mod event;
mod sys;
pub mod util;

use crate::util::{Point, Size};
use std::{
    fmt,
    io::{self, Write},
};

// TODO: maybe do flushing on drop

// TODO: add `error` to abort program with message?

// TODO: return a result instead of `expect`ing?

pub struct Terminal<'a> {
    pub handle: io::BufWriter<io::StdoutLock<'a>>,
    pub size: Size,
    #[cfg(debug_assertions)]
    pub flush_count: usize,
}

// TODO: for better panicking: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=018af4b20094fd7ec0d4cca21d6ff2a8

impl<'a> Terminal<'a> {
    pub fn new(handle: io::StdoutLock<'a>) -> Self {
        let size = Self::size();
        let handle = io::BufWriter::new(handle);
        Self {
            handle,
            size,
            #[cfg(debug_assertions)]
            flush_count: 0,
        }
    }

    pub fn write(&mut self, string: &str) {
        self.handle
            .write_all(string.as_bytes())
            .expect("write to the terminal failed");
    }

    pub fn write_args(&mut self, arguments: fmt::Arguments) {
        self.handle
            .write_fmt(arguments)
            .expect("formatted write to the terminal failed");
    }

    pub fn flush(&mut self) {
        self.handle.flush().expect("flushing failed");

        #[cfg(debug_assertions)]
        {
            self.flush_count += 1;
            self.save_cursor_point();
            self.set_cursor(Point { x: 0, y: 0 });
            let flush_count = self.flush_count;
            self.write_args(format_args!("Flush count: {}", flush_count));
            self.restore_cursor_point();
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
    }

    pub fn deinitialize(&mut self) {
        self.show_cursor();
        self.disable_mouse_capture();
        self.disable_raw_mode();
        self.exit_alternate_dimension();
        self.flush();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
