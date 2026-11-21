//! A cone with its base of radius 5 at z = 0 and its apex 10 above it on the z axis, less a cone
//! of base radius 4.999 with the same base plane and apex: a conical shell a thousandth thick at
//! its base, thinning to nothing at the apex the two share.
use kentron::boolean;
use kentron::primitives::Cone;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let cone = Cone::new(5.0, 0.0, 10.0).build(m)?;
    let inner = Cone::new(4.999, 0.0, 10.0).build(m)?;
    boolean::difference(m, cone, inner)
}
