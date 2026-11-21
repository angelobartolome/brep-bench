//! A cone frustum on the z axis from the disk of radius 6 at z = 0 to the disk of radius 3 at
//! z = 6, its side the cone 2r + z = 12, both its rims rounded to radius 3/2. The top round only is
//! deleted: the top and the cone run on until they meet in the circle of radius 3 at z = 6, and
//! the base's round stays.
use crate::bench::{edge_through, p};
use crate::shared::edit::{corner_round_middle, deleted};
use crate::Refusal;
use kentron::blend;
use kentron::primitives::Cone;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let frustum = Cone::new(6.0, 3.0, 6.0).build(m)?;
    let top = edge_through(m, frustum, p(0.0, 3.0, 6.0))?;
    let base = edge_through(m, frustum, p(0.0, 6.0, 0.0))?;
    let rounded = blend::fillet(m, frustum, &[top, base], 1.5)?;
    // The cone's outward normal is (2, 1)/√5 and the top's is up.
    let (r, z) = corner_round_middle((3.0, 6.0), (2.0 / 5f64.sqrt(), 1.0 / 5f64.sqrt()), (0.0, 1.0), 1.5);
    deleted(m, rounded, &[p(0.0, r, z)])
}
