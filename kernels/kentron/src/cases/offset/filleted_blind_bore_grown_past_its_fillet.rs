//! The cylinder of radius 10 on the z axis from z = 0 to z = 8 with a blind bore of radius 5 down to
//! a floor at z = 2, the concave edge round the floor filleted to radius 2, grown by 3: every point
//! within 3 of it. The fillet grown past its radius drops; the moved bore and floor meet in a sharp
//! concave edge, the circle of radius 2 at z = 5.
use crate::Refusal;
use crate::bench::{frame, p};
use crate::shared::offset::{on_rim, rounded};
use kentron::boolean;
use kentron::math::Direction;
use kentron::offset::{self, Join};
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cylinder::new(10.0, 8.0).build(m)?;
    let bore = Cylinder::new(5.0, 7.0).with_frame(frame(p(0.0, 0.0, 2.0), Direction::Z)).build(m)?;
    let bored = boolean::difference(m, block, bore)?;
    let filleted = rounded(m, bored, &[on_rim(5.0, 2.0)], 2.0)?;
    Ok(offset::shape(m, filleted, 3.0, Join::Arc)?)
}
