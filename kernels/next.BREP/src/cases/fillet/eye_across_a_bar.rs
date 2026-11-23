//! A bar with an eye across its middle, the four concave edges where the eye meets the bar's top
//! and bottom rounded to radius 3.
//!
//! The bar (the box from (−40, −8, −5) to (40, 8, 5)) united with the eye (the cylinder of radius
//! 14 about the y axis from y = −8 to 8), and the four edges, by their middles (±√171, 0, ±5),
//! rounded by the kernel's fillet.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let bar = make_box_brep(p(-40.0, -8.0, -5.0), 80.0, 16.0, 10.0)?;
    let eye = make_cylinder_brep(p(0.0, -8.0, 0.0), p(0.0, 1.0, 0.0), 14.0, 16.0)?;
    let joined = union(&bar, &eye)?;
    let x = 171f64.sqrt();
    let edges = [p(x, 0.0, 5.0), p(-x, 0.0, 5.0), p(x, 0.0, -5.0), p(-x, 0.0, -5.0)];
    Ok(fillet_edges(&joined, &edges, None, 3.0, false, None)?)
}
