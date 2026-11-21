//! A solid paraboloid of revolution about the z axis, a cup with its vertex at the origin filled up
//! to z = 5 (x² + y² ≤ 10z), united with a ball of radius 5 about (0, 0, 5), its poles on the z
//! axis. The two curve alike at the vertex, where the ball's pole touches the cup's bottom; the
//! union is the cup with a half ball on its top.
use crate::bench::p;
use crate::shared::tangent::paraboloid;
use kentron::boolean;
use kentron::math::{Direction, Frame};
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let cup = paraboloid(m, 2.5, 5.0)?;
    let center = Frame::from_z(p(0.0, 0.0, 5.0), Direction::Z);
    let ball = Sphere::new(5.0).with_frame(center).build(m)?;
    boolean::union(m, cup, ball)
}
