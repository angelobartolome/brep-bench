//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, the circle where its side meets its
//! top blended by the kernel's conic fillet of rho 0.5 (a parabola) tangent to the top and the side
//! each 2 from the edge. The edge is selected by a point on it off the seam.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges_conic, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cylinder = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 10.0)?;
    Ok(fillet_edges_conic(&cylinder, &[p(5.0 * 1f64.cos(), 5.0 * 1f64.sin(), 10.0)], None, 2.0, 0.5, None)?)
}
