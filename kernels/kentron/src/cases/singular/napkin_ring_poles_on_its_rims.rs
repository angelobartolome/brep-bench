//! A ball of radius 5 centred on the origin, its poles on the line along (3, 0, 4), with a hole of
//! radius 3 bored through it along the z axis by a cylinder from z = −10 to z = 10. Each pole lies
//! on a rim of the ring: (3, 0, 4) on the upper, (−3, 0, −4) on the lower.
use crate::bench::{d, frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cylinder, Sphere};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let ball = Sphere::new(5.0).with_frame(frame(p(0.0, 0.0, 0.0), d(3.0, 0.0, 4.0))).build(m)?;
    let drill = Cylinder::new(3.0, 20.0).with_frame(frame(p(0.0, 0.0, -10.0), Direction::Z)).build(m)?;
    boolean::difference(m, ball, drill)
}
