//! 3D projection and rotation math.
//!
//! `#![no_std]` by default; All math uses [`libm`] so trig
//! works without `std`.
//!
//! # Types
//!
//! - [`Coords2d`] – 2D point (x, y)
//! - [`Coords3d`] – 3D point (x, y, z)
//!
//! # Functions
//!
//! - [`degree_to_radian`], [`radian_to_degree`] – angle conversion
//! - [`rotate_2d`] – 2D rotation
//! - [`project`] – perspective projection 3D → 2D
//! - [`rotate_3d_in_x`], [`rotate_3d_in_y`], [`rotate_3d_in_z`] – 3D axis rotations
//!
//! Port of `projectingAndRotating.h` (C) to Rust.

#![no_std]
#![deny(unsafe_code)]

use core::f64::consts::PI;
use libm::{cos, sin};

/// A 2D coordinate pair.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Coords2d {
    pub x: f64,
    pub y: f64,
}

/// A 3D coordinate triple.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Coords3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// ---------------------------------------------------------------------------
// Angle conversion
// ---------------------------------------------------------------------------

/// Convert **degrees** to **radians**.
///
/// `result = D × π / 180`
#[inline]
pub fn degree_to_radian(d: f64) -> f64 {
    d * PI / 180.0
}

/// Convert **radians** to **degrees**.
///
/// `result = 180 × R / π`
#[inline]
pub fn radian_to_degree(r: f64) -> f64 {
    180.0 * r / PI
}

// ---------------------------------------------------------------------------
// 2D rotation
// ---------------------------------------------------------------------------

/// Rotate a 2D point `(x, y)` by angle `a` (radians) about the origin.
///
/// Uses the standard rotation matrix:
///
/// ```text
/// x' = x·cos(a) – y·sin(a)
/// y' = x·sin(a) + y·cos(a)
/// ```
#[inline]
pub fn rotate_2d(x: f64, y: f64, a: f64) -> Coords2d {
    let ca = cos(a);
    let sa = sin(a);
    Coords2d {
        x: x * ca - y * sa,
        y: x * sa + y * ca,
    }
}

// ---------------------------------------------------------------------------
// Perspective projection
// ---------------------------------------------------------------------------

/// Project a 3D point `(x, y, z)` onto a 2D plane at focal (camera) distance `F`.
///
/// Returns `(0, 0)` when `F ≤ z` (point at or behind the viewer).
///
/// ```text
/// x' = x × F / (F – z)
/// y' = y × F / (F – z)
/// ```
#[inline]
pub fn project(x: f64, y: f64, z: f64, f: f64) -> Coords2d {
    if f <= z {
        return Coords2d::default();
    }
    let fz = f / (f - z);
    Coords2d {
        x: x * fz,
        y: y * fz,
    }
}

// ---------------------------------------------------------------------------
// 3D rotations (single-axis helpers)
// ---------------------------------------------------------------------------

/// Rotate `(x, y, z)` **about the X axis** by `a` radians.
#[inline]
pub fn rotate_3d_in_x(x: f64, y: f64, z: f64, a: f64) -> Coords3d {
    let ca = cos(a);
    let sa = sin(a);
    Coords3d {
        x,
        y: y * ca - z * sa,
        z: y * sa + z * ca,
    }
}

/// Rotate `(x, y, z)` **about the Y axis** by `a` radians.
#[inline]
pub fn rotate_3d_in_y(x: f64, y: f64, z: f64, a: f64) -> Coords3d {
    let ca = cos(a);
    let sa = sin(a);
    Coords3d {
        x: x * ca + z * sa,
        y,
        z: -x * sa + z * ca,
    }
}

/// Rotate `(x, y, z)` **about the Z axis** by `a` radians.
#[inline]
pub fn rotate_3d_in_z(x: f64, y: f64, z: f64, a: f64) -> Coords3d {
    let ca = cos(a);
    let sa = sin(a);
    Coords3d {
        x: x * ca - y * sa,
        y: x * sa + y * ca,
        z,
    }
}

impl core::fmt::Display for Coords2d {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl core::fmt::Display for Coords3d {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
