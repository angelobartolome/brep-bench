//! A block, the box from (−10, −10, −10) to (40, 40, 0), grooved by a bull-nose mill run along a
//! line, an arc and a line: upright, a cylinder of radius 2 from z = −1.5 to 5 whose bottom corner
//! is rounded to 0.5, its axis carried from the vertical line through (0, 0) along x to (20, 0),
//! a quarter turn about the vertical line through (20, 10) to (30, 10), and along y to (30, 30).
//! Everything the mill passes through, cut.
//!
//! kentron has no sweep of a solid (`sweep::pipe` refuses one: "a pipe of a Solid"), so the sweep
//! is built as elementary geometry gives it: at each height the mill is a disc about its axis,
//! whose turning about that axis changes nothing, and a disc whose centre runs along a plane path
//! passes through the band its diameter square to the path sweeps, with the disc where it starts
//! and where it ends (the arc, of radius 10, is wider than the disc, so the band does not fold). So
//! the mill passes through its section through its axis square to the path, piped along the path,
//! and itself at either end.
use crate::bench::{p, v};
use crate::shared::parts::xz_plane;
use crate::shared::swept::cut_each;
use kentron::geometry::curve::{Circle, Curve};
use kentron::make;
use kentron::math::{Axis, Direction, Frame, Transform};
use kentron::modify::{self, TransformOptions};
use kentron::primitives::Cuboid;
use kentron::sketch::{self, Outcome, Step};
use kentron::sweep;
use kentron::topology::{Model, Shape};
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    use Step::{Arc, Heading, Start, To};
    let block = Cuboid::from_corners(p(-10.0, -10.0, -10.0), p(40.0, 40.0, 0.0)).build(m)?;
    // The path: the line, the arc about (20, 10) from (20, 0) to (30, 10), the line.
    let first = make::edge_between(m, p(0.0, 0.0, 0.0), p(20.0, 0.0, 0.0))?;
    let circle = Circle::new(Frame::new(p(20.0, 10.0, 0.0), Direction::Z, Direction::NEG_Y), 10.0);
    let arc = make::edge_from_curve_range(m, Curve::Circle(circle), 0.0, FRAC_PI_2)?;
    let last = make::edge_between(m, p(30.0, 10.0, 0.0), p(30.0, 30.0, 0.0))?;
    let path = make::wire(m, &[first, arc, last])?;
    // The mill: its half section, out from its axis (the flat bottom to 1.5, the round's quarter
    // circle about (1.5, −1), the side up to the top), turned whole about it.
    let half = [Start(0.0, -1.5), To(1.5, -1.5), Arc { radius: 0.5, angle: 90.0 }, To(2.0, 5.0), To(0.0, 5.0)];
    let half = sketch::profile(m, xz_plane(0.0), &half, Outcome::Face)?;
    let start = sweep::revolve(m, half, Axis::Z, TAU)?.shape;
    let end = modify::transform(m, start, &Transform::translation(v(30.0, 30.0, 0.0)), TransformOptions::default())?;
    // Its whole section through its axis where it starts, in the plane x = 0 (u along y, v up),
    // both rounds included.
    let across = Frame::new(p(0.0, 0.0, 0.0), Direction::X, Direction::Y);
    let whole = [
        Start(-2.0, 5.0),
        To(-2.0, -1.0),
        Heading(-90.0),
        Arc { radius: 0.5, angle: 90.0 },
        To(1.5, -1.5),
        Arc { radius: 0.5, angle: 90.0 },
        To(2.0, 5.0),
    ];
    let whole = sketch::profile(m, across, &whole, Outcome::Face)?;
    let groove = sweep::pipe(m, whole, path)?.shape;
    cut_each(m, block, &[groove, start, end])
}
