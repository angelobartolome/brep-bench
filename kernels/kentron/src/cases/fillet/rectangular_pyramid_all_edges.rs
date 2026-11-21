//! The pyramid on the rectangle from (−10, −6, 0) to (10, 6, 0) with its apex at (0, 0, 12), all
//! eight edges rounded to radius 3/2. Three rounds meet at each corner of the base; at the apex,
//! which has no inscribed ball, the rounds split along a short ridge.
use crate::bench::{edges, p};
use crate::shared::geometry::solid_of;
use crate::shared::loft::flat;
use crate::Refusal;
use kentron::blend;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    // The base and four triangles to the apex, flat faces sewn into the pyramid's shell.
    let base = [p(-10.0, -6.0, 0.0), p(10.0, -6.0, 0.0), p(10.0, 6.0, 0.0), p(-10.0, 6.0, 0.0)];
    let apex = p(0.0, 0.0, 12.0);
    let mut faces = vec![flat(m, &base)?];
    for (i, &a) in base.iter().enumerate() {
        faces.push(flat(m, &[a, base[(i + 1) % 4], apex])?);
    }
    let pyramid = solid_of(m, &faces)?;
    let all = edges(m, pyramid);
    Ok(blend::fillet(m, pyramid, &all, 1.5)?)
}
