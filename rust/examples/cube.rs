//! Terminal spinning cube.
//!
//! Cross-platform Rust version of `Tests/cube.c` / `Tests/cubeLinux.c`.
//! Renders a wireframe cube using ANSI escape codes.

use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use par::{Coords2d, Coords3d, project, rotate_3d_in_x, rotate_3d_in_y};

// ---- geometry -----------------------------------------------------------

/// 8 vertices of a cube centred at (0,0,0) with side length 20.
const CUBE_VERTICES: [Coords3d; 8] = [
    Coords3d {
        x: -10.0,
        y: -10.0,
        z: -10.0,
    },
    Coords3d {
        x: 10.0,
        y: -10.0,
        z: -10.0,
    },
    Coords3d {
        x: 10.0,
        y: 10.0,
        z: -10.0,
    },
    Coords3d {
        x: -10.0,
        y: 10.0,
        z: -10.0,
    },
    Coords3d {
        x: -10.0,
        y: -10.0,
        z: 10.0,
    },
    Coords3d {
        x: 10.0,
        y: -10.0,
        z: 10.0,
    },
    Coords3d {
        x: 10.0,
        y: 10.0,
        z: 10.0,
    },
    Coords3d {
        x: -10.0,
        y: 10.0,
        z: 10.0,
    },
];

/// 12 edges as index pairs into [`CUBE_VERTICES`].
const CUBE_EDGES: [(usize, usize); 12] = [
    (0, 1),
    (1, 2),
    (2, 3),
    (3, 0), // back face
    (4, 5),
    (5, 6),
    (6, 7),
    (7, 4), // front face
    (0, 4),
    (1, 5),
    (2, 6),
    (3, 7), // connecting edges
];

/// Interpolation points per edge (5 intermediate dots).
const POINTS_PER_EDGE: usize = 5;

// ---- render -------------------------------------------------------------

fn main() {
    let center_x = 64.0;
    let center_y = 18.0;
    let f = 60.0; // focal length / camera distance

    let mut angle_x = 0.0;
    let mut angle_y = 0.0;

    // Storage: 8 vertices + 12 edges × 5 interpolated points = 68 points.
    let mut pts = [Coords2d::default(); 128];
    let mut rotated = [Coords3d::default(); 8];

    // Hide cursor.
    print!("\x1b[?25l");
    let _ = io::stdout().flush();

    loop {
        // Clear screen + scrollback, return cursor to (1,1).
        print!("\x1b[2J\x1b[3J\x1b[H");
        let _ = io::stdout().flush();

        let mut n = 0;

        // ---- rotate & project the 8 vertices ----
        for i in 0..8 {
            let v = CUBE_VERTICES[i];
            let r = rotate_3d_in_x(v.x, v.y, v.z, angle_x);
            let r = rotate_3d_in_y(r.x, r.y, r.z, angle_y);
            rotated[i] = r;
            pts[n] = project(r.x, r.y, r.z, f);
            n += 1;
        }

        // ---- interpolate along edges ----
        for &(i1, i2) in &CUBE_EDGES {
            let p1 = rotated[i1];
            let p2 = rotated[i2];
            // The C original puts 5 evenly-spaced dots between the vertices.
            for j in 1..=POINTS_PER_EDGE {
                let t = j as f64 / (POINTS_PER_EDGE as f64 + 1.0);
                let ix = p1.x + (p2.x - p1.x) * t;
                let iy = p1.y + (p2.y - p1.y) * t;
                let iz = p1.z + (p2.z - p1.z) * t;
                pts[n] = project(ix, iy, iz, f);
                n += 1;
            }
        }

        // ---- blit ----
        for p in pts[..n].iter() {
            let sx = (center_x + p.x * 2.0) as i32;
            let sy = (center_y + p.y) as i32;
            if sx > 0 && sx < 128 && sy > 0 && sy < 36 {
                print!("\x1b[{};{}Ho", sy, sx);
            }
        }
        let _ = io::stdout().flush();

        // Advance rotation.
        angle_x += 0.03;
        angle_y += 0.04;

        thread::sleep(Duration::from_millis(40)); // ~25 FPS
    }
}
