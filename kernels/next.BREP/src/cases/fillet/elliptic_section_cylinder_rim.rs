//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, the circle where its side meets its
//! top blended unequally by the kernel's conic blend with two distances: 1 across the top (the
//! face nearest the axis's top point) and 3 down the side, rho √2 − 1, so its section is the
//! quarter ellipse of semi-axes 1 and 3. The edge is selected by a point on it off the seam.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges_conic_distances, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cylinder = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 10.0)?;
    let rim = p(5.0 * 1f64.cos(), 5.0 * 1f64.sin(), 10.0);
    // 1 across the top (the face by the axis's top point), 3 down the side; rho √2 − 1, the quarter ellipse.
    Ok(fillet_edges_conic_distances(&cylinder, &[rim], None, p(0.0, 0.0, 10.0), 1.0, 3.0, std::f64::consts::SQRT_2 - 1.0, None)?)
}
