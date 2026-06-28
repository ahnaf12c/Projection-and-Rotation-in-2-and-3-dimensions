//! 3D projection + 2D rotation test.
//!
//! Cross-platform Rust version of `Tests/test.c`.
//! Projects a point, prints it, then rotates that projection in 2D.

use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use par::{degree_to_radian, project, rotate_2d, rotate_3d_in_x};

fn move_cursor(x: i32, y: i32) {
    print!("\x1b[{};{}H", y, x);
}

fn clear_screen() {
    print!("\x1b[2J\x1b[3J\x1b[H");
    let _ = io::stdout().flush();
}

fn main() {
    let angle = degree_to_radian(10.0);

    // Rotate (5, 6, 5) by 45 degrees around X.
    let r = rotate_3d_in_x(5.0, 6.0, 5.0, 45.0);
    println!("{}  {}  {}", r.x, r.y, r.z);

    // Project with focal length 25.
    let p = project(r.x, r.y, r.z, 25.0);

    clear_screen();
    move_cursor(p.x as i32 + 64, p.y as i32 + 18);
    println!("O");

    // Now rotate the projected 2D point in a loop.
    let mut new = p;
    loop {
        new = rotate_2d(new.x, new.y, angle);
        move_cursor((new.x * 2.0 + 64.0) as i32, (new.y + 18.0) as i32);
        println!("o");
        let _ = io::stdout().flush();
        thread::sleep(Duration::from_millis(500));
    }
}
