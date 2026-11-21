//! The cube from the origin to (30, 30, 30), drilled right through by fifteen holes of radius 2.5:
//! three along x (y = 7.5, 15, 22.5 at z = 15), three along y (x = 7.5, 15, 22.5 at z = 15) and
//! nine along z (x and y each 7.5, 15, 22.5). At each of the nine points (x, y, 15) three holes
//! cross square to each other. Each drill runs 1 past the faces at both ends.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

const GRID: [f64; 3] = [7.5, 15.0, 22.5];

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let mut axes = Vec::new();
    for a in GRID {
        axes.push(frame(p(-1.0, a, 15.0), Direction::X));
        axes.push(frame(p(a, -1.0, 15.0), Direction::Y));
        for b in GRID {
            axes.push(frame(p(a, b, -1.0), Direction::Z));
        }
    }
    let mut cube = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(30.0, 30.0, 30.0)).build(m)?;
    for axis in axes {
        let drill = Cylinder::new(2.5, 32.0).with_frame(axis).build(m)?;
        cube = boolean::difference(m, cube, drill)?;
    }
    Ok(cube)
}
