//! A square plate with a round post across its edge, drilled through the post, both rims of the
//! hole chamfered 1.5 at 45°.
//!
//! The plate (the box from (−50, −50, 0) to (50, 50, 20)) and the post (radius 15 about (45, 0),
//! z 0 to 40) united, the hole (radius 6 on the post's axis) cut right through, and the two rims,
//! by a point on each, chamfered by the kernel with 1.5 along each face.
use crate::solids::boolean::{cut, union};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{chamfer_edges_asymmetric, make_box_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let up = p(0.0, 0.0, 1.0);
    let plate = make_box_brep(p(-50.0, -50.0, 0.0), 100.0, 100.0, 20.0)?;
    let post = make_cylinder_brep(p(45.0, 0.0, 0.0), up, 15.0, 40.0)?;
    let hole = make_cylinder_brep(p(45.0, 0.0, -1.0), up, 6.0, 42.0)?;
    let drilled = cut(&union(&plate, &post)?, &hole)?;
    Ok(chamfer_edges_asymmetric(&drilled, &[p(39.0, 0.0, 0.0), p(39.0, 0.0, 40.0)], None, 1.5, 1.5, None)?)
}
