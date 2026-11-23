//! A plate 12 thick whose outline is six lobes (radius 8, 40 from the axis) and six hollows
//! (radius 30, tangent to the lobes beside them), both rims rounded to radius 1.
//!
//! The twelve arcs from the case's own construction (each hollow's centre on the ray between two
//! lobes', 38 from each, the farther such point; each tangent point on the line of centres, 8
//! from the lobe's), extruded from z = 0 to 12; both rims' 24 edges, by their middles, rounded by
//! the kernel's fillet.
use crate::solids::outline::{deg, Chain};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, fillet_edges, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let far = 20.0 * 3f64.sqrt() + 6.0 * 29f64.sqrt();
    let lobe = |k: usize| (40.0 * deg(60.0 * k as f64).cos(), 40.0 * deg(60.0 * k as f64).sin());
    let hollow = |k: usize| (far * deg(30.0 + 60.0 * k as f64).cos(), far * deg(30.0 + 60.0 * k as f64).sin());
    let angle = |from: (f64, f64), to: (f64, f64)| (to.1 - from.1).atan2(to.0 - from.0);
    let mut c = Chain::new();
    for k in 0..6 {
        let (l, before, after) = (lobe(k), hollow((k + 5) % 6), hollow(k));
        // Round the lobe from where the hollow before touches it to where the hollow after does,
        // then round the hollow after, the other way, to where the next lobe touches it.
        c.arc(l, 8.0, angle(l, before), angle(l, after), true)?;
        c.arc(after, 30.0, angle(after, l), angle(after, lobe((k + 1) % 6)), false)?;
    }
    let plate = extrude_profile_brep(&c.curves, p(0.0, 0.0, 1.0), 12.0)?;
    let mut rims = c.middles_at(0.0);
    rims.extend(c.middles_at(12.0));
    Ok(fillet_edges(&plate, &rims, None, 1.0, false, None)?)
}
