//! Simple cursor-placement test.
//!
//! Cross-platform Rust version of `Tests/test2.c`.
//! Prints an 'X' at the terminal centre, then waits for the user to press Enter.

use std::io::{self, Write};

fn move_cursor(x: i32, y: i32) {
    print!("\x1b[{};{}H", y, x);
}

fn clear_screen() {
    print!("\x1b[2J\x1b[3J\x1b[H");
    let _ = io::stdout().flush();
}

fn main() {
    clear_screen();

    // Centre of an 128×36 virtual terminal (matches the other examples).
    move_cursor(64, 18);
    print!("X");
    let _ = io::stdout().flush();

    // Move cursor out of the way and wait for Enter (like scanf).
    move_cursor(1, 15);
    println!();

    let mut _buf = String::new();
    let _ = io::stdin().read_line(&mut _buf);
}
