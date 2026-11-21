//! The elliptic cylinder of `geometry/elliptic-cylinder`, the ellipse x²/25 + y²/9 ≤ 1 at z = 0
//! extruded up to z = 10, shrunk by 2.5: every point at
//! least 2.5 from its outside. Moved in by more than the ellipse's least radius of curvature, 1.8,
//! the side's curve would loop at the ends of the long axis; trimmed, it meets itself there in a
//! sharp edge.
use crate::shared::offset::elliptic_cylinder;
use kentron::offset::{self, Join};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let cylinder = elliptic_cylinder(m)?;
    offset::shape(m, cylinder, -2.5, Join::Arc)
}
