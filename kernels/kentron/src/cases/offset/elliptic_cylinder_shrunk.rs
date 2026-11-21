//! The elliptic cylinder of `geometry/elliptic-cylinder`, the ellipse x²/25 + y²/9 ≤ 1 at z = 0
//! extruded up to z = 10, shrunk by 1: every point at least 1 from its
//! outside. The side moves in to the curve at distance 1 inside the ellipse, which stays smooth, and
//! the edges stay sharp.
use crate::shared::offset::elliptic_cylinder;
use kentron::offset::{self, Join};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let cylinder = elliptic_cylinder(m)?;
    offset::shape(m, cylinder, -1.0, Join::Arc)
}
