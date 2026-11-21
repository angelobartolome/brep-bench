//! A cone frustum on the z axis from the disk of radius 6 at z = 0 to the disk of radius 3 at
//! z = 6, its side the cone 2r + z = 12, both its rims rounded to radius 3/2. Both rounds are
//! deleted together: the ends and the cone run on until they meet in the circles of radius 3 at
//! z = 6 and radius 6 at z = 0, and the result is the plain frustum.
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
    // The cone's outward normal is (2, 1)/√5; the top's is up and the base's down.
    let cone_out = (2.0 / 5f64.sqrt(), 1.0 / 5f64.sqrt());
    let top_middle = corner_round_middle((3.0, 6.0), cone_out, (0.0, 1.0), 1.5);
    let base_middle = corner_round_middle((6.0, 0.0), cone_out, (0.0, -1.0), 1.5);
    deleted(m, rounded, &[p(0.0, top_middle.0, top_middle.1), p(0.0, base_middle.0, base_middle.1)])
}
