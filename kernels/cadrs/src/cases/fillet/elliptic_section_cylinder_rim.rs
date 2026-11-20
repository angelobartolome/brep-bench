//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, the circle round its top rounded
//! unequally, 1 across the top and 3 down the side, its section a quarter ellipse.
//!
//! The rim is given cadrs's asymmetric fillet: the size on the edge's first face, the second size
//! on its other, the first put on the top.
use crate::bench::{cylinder, p3, v3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, FilletProfile, FilletSpec, Kernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let c = cylinder(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 5.0, 10.0)?;
    let rim = k.edge_at(c, p3(5.0, 0.0, 10.0))?;
    let top = k.face_at(c, p3(0.0, 0.0, 10.0))?;
    let info = k.edges(c)?.into_iter().find(|e| e.id == rim).ok_or("the rim is gone")?;
    // 1 on the top and 3 on the side: flipped where the top is not the edge's first face
    let flip = info.faces[0] != Some(top);
    let profile = FilletProfile::Asymmetric { second: 3.0, flip };
    let spec = FilletSpec { profile, ..FilletSpec::radius(1.0) };
    Ok(k.fillet_with(c, &[rim], &spec)?.first()?)
}
