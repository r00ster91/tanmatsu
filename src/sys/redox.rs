//! (Unfinished) terminal implementation for the Redox operating system.

// TODO: if the Redox backend is to be implemented, the methods should probably be defined in a trait

use crate::{
    event::{Event, KeyEvent, KeyModifier, MouseButton, MouseEvent, MouseEventKind},
    util::{Color, Point, Size},
    Terminal,
};
use std::io::{self, Write};
use std::time::Duration;
use termion::{raw::IntoRawMode, screen};

impl<'a> Terminal<'a> {
    pub fn enter_alternate_dimension(&mut self) {
        write!(self.stdout, "{}", screen::ToAlternateScreen);
    }
    pub fn exit_alternate_dimension(&mut self) {
        write!(self.stdout, "{}", screen::ToMainScreen);
    }

    pub fn set_title(&mut self, title: &str) {
        write!(self.stdout, "\u{1B}]0;{}\u{7}", title);
    }

    pub fn enable_raw_mode(&mut self) {
        self.stdout.into_raw_mode();
    }
}

// Also see `terminal` as a reference
