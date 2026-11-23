//! A cube of side 10 between the origin and (10, 10, 10), the edge from (10, 10, 0) to
//! (10, 10, 10) chamfered with a distance on the face x = 10 growing linearly along it from 1 at
//! z = 0 to 3 at z = 10, and 2 on the face y = 10 all along: the kernel's two-distance chamfer
//! with distances linear along a straight edge, ruled between its two end sections (here a
//! hyperbolic paraboloid, its rails skew).
use crate::solids::fillet::faces_of_edge_through;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{chamfer_edges_asymmetric_linear, make_box_brep, AnalyticSurface, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cube = make_box_brep(p(0.0, 0.0, 0.0), 10.0, 10.0, 10.0)?;
    let middle = p(10.0, 10.0, 5.0);
    // The kernel sets its first distances back on the edge's first face: the varying one where
    // that is the face x = 10.
    let x_first = match faces_of_edge_through(&cube, middle).first() {
        Some(face) => match face.surface.analytic() {
            Some(AnalyticSurface::Plane { origin, u_dir, v_dir, .. }) => {
                let normal = u_dir.cross(*v_dir);
                (origin.x - 10.0).abs() <= 1e-9 && normal.y.abs() + normal.z.abs() <= 1e-9 * normal.length()
            }
            _ => false,
        },
        None => return Err(Refusal::from(format!("no edge through {middle:?}"))),
    };
    let (start, end) = if x_first { ([1.0, 2.0], [3.0, 2.0]) } else { ([2.0, 1.0], [2.0, 3.0]) };
    Ok(chamfer_edges_asymmetric_linear(&cube, &[middle], p(10.0, 10.0, 0.0), start, end, None)?)
}
