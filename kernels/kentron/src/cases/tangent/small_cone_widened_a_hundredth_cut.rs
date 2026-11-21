//! A small cone with its base of radius 0.00075 at z = 0 and its apex 0.0015 above it, and a copy
//! a hundredth wider at its base, radius 0.0007575, with the same base plane and apex. The copy
//! less the cone: a conical shell thinning to nothing at the shared apex.
use kentron::boolean;
use kentron::primitives::Cone;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let cone = Cone::new(0.00075, 0.0, 0.0015).build(m)?;
    let wider = Cone::new(0.0007575, 0.0, 0.0015).build(m)?;
    boolean::difference(m, wider, cone)
}
