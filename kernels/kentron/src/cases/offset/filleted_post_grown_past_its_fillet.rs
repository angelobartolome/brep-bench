//! The plate, the cylinder of radius 8 on the z axis from z = −4 to z = 0, and on it the post, the
//! cylinder of radius 3 up to z = 8, joined, the concave edge where they meet filleted to radius 2;
//! grown by 3: every point within 3 of it. The fillet grown past its radius drops; the moved post and
//! plate meet in a sharp concave edge, the circle of radius 6 at z = 3.
use crate::Refusal;
use crate::shared::offset::on_plate;
use kentron::offset::{self, Join};
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let post = Cylinder::new(3.0, 8.0).build(m)?;
    let solid = on_plate(m, 4.0, post, 3.0, 2.0)?;
    Ok(offset::shape(m, solid, 3.0, Join::Arc)?)
}
