//! Four balls of radius 10 centred on the corners of a regular tetrahedron of side 10, (0, 0, 0),
//! (10, 0, 0), (5, 5√3, 0) and (5, 5√3/3, 10√(2/3)): the solid common to all four, a Reuleaux
//! tetrahedron.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let s3 = 3.0_f64.sqrt();
    let apex = p(5.0, 5.0 * s3 / 3.0, 10.0 * (2.0_f64 / 3.0).sqrt());
    let corners = [p(0.0, 0.0, 0.0), p(10.0, 0.0, 0.0), p(5.0, 5.0 * s3, 0.0), apex];
    let mut solid = Sphere::new(10.0).with_frame(frame(corners[0], Direction::Z)).build(m)?;
    for &c in &corners[1..] {
        let ball = Sphere::new(10.0).with_frame(frame(c, Direction::Z)).build(m)?;
        solid = boolean::intersection(m, solid, ball)?;
    }
    Ok(solid)
}
