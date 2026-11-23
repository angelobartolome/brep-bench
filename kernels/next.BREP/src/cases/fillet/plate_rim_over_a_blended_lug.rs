//! A plate (corners rounded to 10) with a round lug on its north side, the lug blended into the
//! side to radius 8, then the plate's bottom rim rounded to radius 2 all the way round.
//!
//! The plate's rounded rectangle extruded, united with the lug (the cylinder of radius 9 about the
//! upright through (0, 25)); the two concave upright edges at (±9, 25) rounded to 8 by the kernel's
//! fillet; then the bottom rim, by its pieces' middles, rounded to 2 along the whole tangent chain
//! through them.
use crate::solids::boolean::union;
use crate::solids::outline::{deg, Chain};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, fillet_edges, fillet_edges_tangent_chains, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let mut plan = Chain::new();
    plan.line((-30.0, -25.0), (30.0, -25.0))?
        .arc((30.0, -15.0), 10.0, deg(270.0), deg(360.0), true)?
        .line((40.0, -15.0), (40.0, 15.0))?
        .arc((30.0, 15.0), 10.0, deg(0.0), deg(90.0), true)?
        .line((30.0, 25.0), (-30.0, 25.0))?
        .arc((-30.0, 15.0), 10.0, deg(90.0), deg(180.0), true)?
        .line((-40.0, 15.0), (-40.0, -15.0))?
        .arc((-30.0, -15.0), 10.0, deg(180.0), deg(270.0), true)?;
    let plate = extrude_profile_brep(&plan.curves, p(0.0, 0.0, 1.0), 13.0)?;
    let lug = make_cylinder_brep(p(0.0, 25.0, 0.0), p(0.0, 0.0, 1.0), 9.0, 13.0)?;
    let joined = union(&plate, &lug)?;
    let blended = fillet_edges(&joined, &[p(9.0, 25.0, 6.5), p(-9.0, 25.0, 6.5)], None, 8.0, false, None)?;
    // The blended outline's twelve pieces, for their middles (the case's closed forms: each blend
    // about (±15, 33), touching the lug where the line from (0, 25) to its centre crosses it).
    let phi = (8.0f64).atan2(15.0);
    let mut rim = Chain::new();
    rim.line((-30.0, -25.0), (30.0, -25.0))?
        .arc((30.0, -15.0), 10.0, deg(270.0), deg(360.0), true)?
        .line((40.0, -15.0), (40.0, 15.0))?
        .arc((30.0, 15.0), 10.0, deg(0.0), deg(90.0), true)?
        .line((30.0, 25.0), (15.0, 25.0))?
        .arc((15.0, 33.0), 8.0, deg(270.0), std::f64::consts::PI + phi, false)?
        .arc((0.0, 25.0), 9.0, phi, std::f64::consts::PI - phi, true)?
        .arc((-15.0, 33.0), 8.0, -phi, deg(-90.0), false)?
        .line((-15.0, 25.0), (-30.0, 25.0))?
        .arc((-30.0, 15.0), 10.0, deg(90.0), deg(180.0), true)?
        .line((-40.0, 15.0), (-40.0, -15.0))?
        .arc((-30.0, -15.0), 10.0, deg(180.0), deg(270.0), true)?;
    // The whole tangent chain through the picked edges: the lug's arc is two edges split at the
    // cylinder's seam, and its middle is that vertex, so picking by middles alone would round only
    // one half.
    Ok(fillet_edges_tangent_chains(&blended, &rim.middles_at(0.0), None, 2.0, None)?)
}
