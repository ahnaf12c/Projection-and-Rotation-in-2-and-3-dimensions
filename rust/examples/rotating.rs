//! Rotating `o` animation.
//!
//! Cross-platform Rust version of `Tests/rotating.c`.
//! Draws a circle by rotating a single 'o' character using ANSI escapes.

use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use par::{degree_to_radian, rotate_2d};

fn main() {
    // Clear screen + scrollback.
    print!("\x1b[2J\x1b[3J\x1b[H");
    let _ = io::stdout().flush();

    let center_x = 64.0;
    let center_y = 18.0;
    let radius_x = 15.0;
    let radius_y = 0.0; // start on the right
    let mut angle = 0.0;

    // Hide cursor.
    print!("\x1b[?25l");
    let _ = io::stdout().flush();

    // Erase the 'o' that was drawn last frame by re-visiting its previous
    // position and writing a space.  We keep track of the last screen
    // coordinates so we don't need to worry about terminal scroll.
    let mut prev_x = 0i32;
    let mut prev_y = 0i32;
    let mut first = true;

    loop {
        // ---- compute next position ----
        let rotated = rotate_2d(radius_x, radius_y, angle);
        let sx = (center_x + rotated.x * 2.0) as i32;
        let sy = (center_y + rotated.y) as i32;

        // Erase previous char.
        if !first {
            print!("\x1b[{};{}H ", prev_y, prev_x);
        }
        first = false;

        // Draw new char.
        print!("\x1b[{};{}Ho", sy, sx);
        let _ = io::stdout().flush();

        prev_x = sx;
        prev_y = sy;

        // Advance angle by 5 degrees.
        angle += degree_to_radian(5.0);
        if angle >= 2.0 * core::f64::consts::PI {
            angle -= 2.0 * core::f64::consts::PI;
        }

        thread::sleep(Duration::from_millis(33)); // ~30 FPS
    }
}
