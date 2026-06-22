## Rust crate

### Add

```toml
[dependencies]
par = { git = "https://github.com/ahnaf12c/Projection-and-Rotation-in-2-and-3-dimensions", subdir = "rust" }
```

### Usage

```rust
use par::{
    project, rotate_2d, rotate_3d_in_x, rotate_3d_in_y, rotate_3d_in_z,
    degree_to_radian, radian_to_degree,
    Coords2d, Coords3d,
};

// Project a 3D point to 2D with focal length 60.
let p = project(10.0, 20.0, 5.0, 60.0);

// Rotate a 3D point 45 degrees around the X axis.
let r = rotate_3d_in_x(1.0, 2.0, 3.0, degree_to_radian(45.0));

// Rotate a 2D point.
let r2 = rotate_2d(5.0, 0.0, degree_to_radian(90.0));
```

### Features

| Feature | Description |
|---------|-------------|
| *(default)* | `no_std` with `libm` for trigonometry |
| `"std"` | Opt into `std` (currently adds `Display` impls for `Coords2d`/`Coords3d`) |
