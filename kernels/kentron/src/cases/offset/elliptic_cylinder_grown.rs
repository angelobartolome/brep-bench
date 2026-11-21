//! The elliptic cylinder of `geometry/elliptic-cylinder`, the ellipse x²/25 + y²/9 ≤ 1 at z = 0
//! extruded up to z = 10, grown by 1: every point within 1 of it. The side moves out
//! to the curve at distance 1 outside the ellipse, and each rim becomes a quarter of a tube of radius
//! 1 about the rim's ellipse.
use crate::shared::offset::elliptic_cylinder;
use kentron::offset::{self, Join};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let cylinder = elliptic_cylinder(m)?;
    offset::shape(m, cylinder, 1.0, Join::Arc)
}
