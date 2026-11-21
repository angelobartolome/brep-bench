//! Two spherical sectors of a ball of radius 10 about the origin, each the part within 45° of its
//! axis: the first about the z axis, toward +z, and the second the first turned 60° about the y
//! axis. The solid common to both. A sector is its section (the arc of the ball from its pole to
//! 45° down, the ruling back to the apex, and the axis) turned about the z axis.
use crate::bench::p;
use crate::shared::geometry::{arc, flat, segment, turned};
use kentron::boolean;
use kentron::geometry::curve::{Circle, Curve};
use kentron::math::{Axis, Direction, Frame, Transform};
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_4;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let apex = p(0.0, 0.0, 0.0);
    // in the xz plane, from the pole (0, 0, 10) toward +x as its parameter grows
    let meridian = Curve::Circle(Circle::new(Frame::new(apex, Direction::Y, Direction::Z), 10.0));
    let rim = p(10.0 * FRAC_PI_4.sin(), 0.0, 10.0 * FRAC_PI_4.cos());
    let cap = arc(m, meridian, 0.0, FRAC_PI_4)?;
    let ruling = segment(m, rim, apex)?;
    let axis = segment(m, apex, p(0.0, 0.0, 10.0))?;
    let section = flat(m, &[cap, ruling, axis])?;
    let first = turned(m, section)?;
    let second = first.moved(&Transform::rotation(Axis::Y, 60f64.to_radians()));
    boolean::intersection(m, first, second)
}
