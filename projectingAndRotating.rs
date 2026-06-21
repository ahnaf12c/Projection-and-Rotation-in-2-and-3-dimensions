use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords2d {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn degree_to_radian(d: f64) -> f64 {
    d * PI / 180.0
}

pub fn radian_to_degree(r: f64) -> f64 {
    180.0 * r / PI
}

pub fn project(x: f64, y: f64, z: f64, f: f64) -> Option<Coords2d> {
    if f <= z {
        return None;
    }
    let scale = f / (f - z);
    Some(Coords2d {
        x: x * scale,
        y: y * scale,
    })
}

pub fn rotate_2d(x: f64, y: f64, angle: f64) -> Coords2d {
    let (s, c) = angle.sin_cos();
    Coords2d {
        x: x * c - y * s,
        y: x * s + y * c,
    }
}

pub fn rotate_3d_in_x(x: f64, y: f64, z: f64, angle: f64) -> Coords3d {
    let (s, c) = angle.sin_cos();
    Coords3d {
        x,
        y: y * c - z * s,
        z: y * s + z * c,
    }
}

pub fn rotate_3d_in_y(x: f64, y: f64, z: f64, angle: f64) -> Coords3d {
    let (s, c) = angle.sin_cos();
    Coords3d {
        x: x * c + z * s,
        y,
        z: -x * s + z * c,
    }
}

pub fn rotate_3d_in_z(x: f64, y: f64, z: f64, angle: f64) -> Coords3d {
    let (s, c) = angle.sin_cos();
    Coords3d {
        x: x * c - y * s,
        y: x * s + y * c,
        z,
    }
}
