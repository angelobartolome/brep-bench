//! Two cylinders of radius 3 and length 20 centred on c = (600000, −800000, 0), the first along
//! (2, 3, 6)/7 and the second along (3, −6, 2)/7, square to each other: the solid common to both,
//! a Steinmetz bicylinder.
use crate::bench::{frame, p, v};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let center = p(600000.0, -800000.0, 0.0);
    let mut cylinders = Vec::new();
    for (x, y, z) in [(2.0, 3.0, 6.0), (3.0, -6.0, 2.0)] {
        // from c − 10u to c + 10u along its own direction u
        let start = center - v(x, y, z) * (10.0 / 7.0);
        let along = frame(start, Direction::new(x, y, z));
        cylinders.push(Cylinder::new(3.0, 20.0).with_frame(along).build(m)?);
    }
    boolean::intersection(m, cylinders[0], cylinders[1])
}
