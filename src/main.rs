fn main() {
    let mut terminal = tanmatsu::Terminal::new().unwrap();

    // This file exists for testing purposes.

    terminal.flush();

    std::thread::park();
}
