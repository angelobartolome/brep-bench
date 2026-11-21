//! Five balls, each inside the one before without touching: radius 10 about the origin, 8 about
//! (1, 0, 0), 6 about (1, 1, 0), 4 about (1, 1, 1) and 2 about (1, 1.5, 1.5). The first less the
//! second, joined with the third less the fourth, joined with the fifth: three nested bodies.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let mut balls = Vec::new();
    let centres = [(10.0, p(0.0, 0.0, 0.0)), (8.0, p(1.0, 0.0, 0.0)), (6.0, p(1.0, 1.0, 0.0)), (4.0, p(1.0, 1.0, 1.0))];
    for (r, c) in centres.into_iter().chain([(2.0, p(1.0, 1.5, 1.5))]) {
        balls.push(Sphere::new(r).with_frame(frame(c, Direction::Z)).build(m)?);
    }
    let outer = boolean::difference(m, balls[0], balls[1])?;
    let middle = boolean::difference(m, balls[2], balls[3])?;
    let two = boolean::union(m, outer, middle)?;
    boolean::union(m, two, balls[4])
}
