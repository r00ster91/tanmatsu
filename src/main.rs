use std::io;

fn main() {
    let stdout = io::stdout();
    let mut terminal = tanmatsu::Terminal::new(stdout.lock()).unwrap();

    // This file exists for testing purposes.

    terminal.flush();

    std::thread::park();
}
