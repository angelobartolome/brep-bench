//! A block over a plan of thirteen tangent pieces (a rounded rectangle with a lobe hanging from its
//! south side and a reverse arc climbing out of it), z 0 to 10, its whole top rim rounded to 2.
//!
//! The plan's pieces exactly as the case lists them, extruded; the thirteen rim edges, by their
//! middles at z = 10, rounded by the kernel's fillet as one chain.
use crate::solids::outline::{deg, Chain};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, fillet_edges, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let mut c = Chain::new();
    c.line((5.0, 0.0), (30.0, 0.0))?
        .arc((30.0, -5.0), 5.0, deg(90.0), deg(0.0), false)?
        .arc((45.0, -5.0), 10.0, deg(180.0), deg(360.0), true)?
        .line((55.0, -5.0), (55.0, 0.0))?
        .arc((60.0, 0.0), 5.0, deg(180.0), deg(90.0), false)?
        .line((60.0, 5.0), (75.0, 5.0))?
        .arc((75.0, 10.0), 5.0, deg(270.0), deg(360.0), true)?
        .line((80.0, 10.0), (80.0, 35.0))?
        .arc((75.0, 35.0), 5.0, deg(0.0), deg(90.0), true)?
        .line((75.0, 40.0), (5.0, 40.0))?
        .arc((5.0, 35.0), 5.0, deg(90.0), deg(180.0), true)?
        .line((0.0, 35.0), (0.0, 5.0))?
        .arc((5.0, 5.0), 5.0, deg(180.0), deg(270.0), true)?;
    let block = extrude_profile_brep(&c.curves, p(0.0, 0.0, 1.0), 10.0)?;
    Ok(fillet_edges(&block, &c.middles_at(10.0), None, 2.0, false, None)?)
}
