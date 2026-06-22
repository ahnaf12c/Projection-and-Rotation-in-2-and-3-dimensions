//! Lightweight graphical projection viewer.
//!
//! Rust analogue of `Tests/test.py` – opens a window, projects a 3D shape
//! (cube or pyramid) with the library's [`par::project`], and draws its
//! wireframe edges using Bresenham line rasterisation.
//!
//! ## Usage
//!
//! ```text
//! cargo run --example test_py
//! ```
//!
//! You will be prompted for a shape name (`cube` or `pyramid`).
//! Close the window to exit.

use std::io::{self, Write};

use minifb::{Key, Window, WindowOptions};
use par::{Coords2d, project};

const W: usize = 640;
const H: usize = 480;

// ---------------------------------------------------------------------------
// 3D → 2D vertex lists (matches test.py exactly)
// ---------------------------------------------------------------------------

fn cube_shape() -> (Vec<[f64; 3]>, Vec<[usize; 2]>) {
    // Vertices — test.py projects each as: project(x, y, z, F=24)
    let vertices = vec![
        [20.0, 20.0, 10.0],   // A
        [-20.0, 20.0, 10.0],  // B
        [-20.0, -20.0, 10.0], // C
        [20.0, -20.0, 10.0],  // D
        [20.0, 20.0, 0.0],    // E
        [-20.0, 20.0, 0.0],   // F
        [-20.0, -20.0, 0.0],  // G
        [20.0, -20.0, 0.0],   // H
    ];
    // Edge pairs  (index order matches test.py's turtle stroke sequence)
    let edges = vec![
        [0, 1],
        [1, 2],
        [2, 3],
        [3, 0], // front face
        [0, 4],
        [4, 5],
        [5, 6],
        [6, 7],
        [7, 4], // top + back face
        [5, 1],
        [2, 6],
        [7, 3], // connecting edges (F→B, C→G, H→D)
    ];
    (vertices, edges)
}

fn pyramid_shape() -> (Vec<[f64; 3]>, Vec<[usize; 2]>) {
    // Vertices — test.py projects each as: project(x, y, z, F=60)
    let vertices = vec![
        [0.0, 40.0, 30.0],     // A  apex
        [-50.0, -50.0, -50.0], // B
        [50.0, -50.0, -50.0],  // C
        [50.0, 50.0, -50.0],   // D
        [-50.0, 50.0, -50.0],  // E
    ];
    // Edge pairs  (index order matches test.py)
    let edges = vec![
        [1, 2],
        [2, 3],
        [3, 4],
        [4, 1], // base square
        [1, 0],
        [0, 2],
        [0, 3],
        [0, 4], // apex edges
    ];
    (vertices, edges)
}

// ---------------------------------------------------------------------------
// Bresenham line rasterisation
// ---------------------------------------------------------------------------

fn draw_line(buf: &mut [u32], x0: i32, y0: i32, x1: i32, y1: i32, color: u32, w: i32) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut x = x0;
    let mut y = y0;
    loop {
        if x >= 0 && x < w && y >= 0 {
            let idx = y as usize * w as usize + x as usize;
            if idx < buf.len() {
                buf[idx] = color;
            }
        }
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() {
    // ---- shape selection (like turtle.textinput) ----
    print!("Shape (cube / pyramid): ");
    let _ = io::stdout().flush();
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let (vertices_3d, edges, focal) = match input.trim().to_lowercase().as_str() {
        "pyramid" => {
            let (v, e) = pyramid_shape();
            (v, e, 60.0)
        }
        _ => {
            let (v, e) = cube_shape();
            (v, e, 24.0)
        }
    };

    // ---- project 3D vertices to 2D ----
    let projected: Vec<Coords2d> = vertices_3d
        .iter()
        .map(|&[x, y, z]| project(x, y, z, focal))
        .collect();

    // ---- centre & auto-scale from projected bounding box ----
    let cx = (W / 2) as f64;
    let cy = (H / 2) as f64;

    // Find extents of the projected points.
    let (min_x, max_x) = projected
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(mn, mx), p| {
            (mn.min(p.x), mx.max(p.x))
        });
    let (min_y, max_y) = projected
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(mn, mx), p| {
            (mn.min(p.y), mx.max(p.y))
        });

    let range_x = (max_x - min_x).max(1.0);
    let range_y = (max_y - min_y).max(1.0);

    // Scale so the shape uses 80 % of the smaller screen dimension.
    let scale = ((W as f64) / range_x * 0.80).min((H as f64) / range_y * 0.80);

    // Centre the bounding-box midpoint on screen (flip Y).
    let mid_x = (min_x + max_x) / 2.0;
    let mid_y = (min_y + max_y) / 2.0;

    let points_2d: Vec<(i32, i32)> = projected
        .iter()
        .map(|p| {
            let sx = cx + (p.x - mid_x) * scale;
            let sy = cy - (p.y - mid_y) * scale;
            (sx as i32, sy as i32)
        })
        .collect();

    // ---- framebuffer window ----
    let mut buf = vec![0u32; W * H];
    let mut window = Window::new(
        "3D Projection (turtle-like)",
        W,
        H,
        WindowOptions::default(),
    )
    .expect("minifb window creation failed");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        std::thread::sleep(std::time::Duration::from_millis(16));
        // Clear to white.
        buf.fill(0x00_FF_FF_FF);

        // Draw edges — colour matches turtle's default black.
        for &[i, j] in &edges {
            let (x0, y0) = points_2d[i];
            let (x1, y1) = points_2d[j];
            draw_line(&mut buf, x0, y0, x1, y1, 0x00_00_00_00, W as i32);
        }

        // Draw vertex dots (like turtle.dot()).
        for &(x, y) in &points_2d {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let px = x + dx;
                    let py = y + dy;
                    if px >= 0 && px < W as i32 && py >= 0 && py < H as i32 {
                        buf[py as usize * W + px as usize] = 0x00_00_00_00;
                    }
                }
            }
        }

        let _ = window.update_with_buffer(&buf, W, H);
    }
}
