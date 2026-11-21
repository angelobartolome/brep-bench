//! A small cylinder of radius 0.00075 and height 0.0015 about the z axis from z = 0, and a copy a
//! hundredth wider, of radius 0.0007575, between the same two planes. The copy less the cylinder:
//! a tube whose wall is 0.0000075 thick.
use kentron::boolean;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let cylinder = Cylinder::new(0.00075, 0.0015).build(m)?;
    let wider = Cylinder::new(0.0007575, 0.0015).build(m)?;
    boolean::difference(m, wider, cylinder)
}
