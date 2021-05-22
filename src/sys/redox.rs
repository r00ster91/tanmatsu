//! (Unfinished) terminal implementation for the Redox operating system.

// TODO: if the Redox backend is to be implemented, the methods should probably be defined in a trait

use std::io;
use termion::screen;

pub fn enter_alternate_dimension(stdout: &mut io::Stdout) {
    write!(stdout, screen::ToAlternateScreen);
}

pub fn exit_alternate_dimension(stdout: &mut io::Stdout) {
    write!(stdout, screen::ToMainScreen);
}
