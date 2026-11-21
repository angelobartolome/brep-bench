use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::{FRAC_PI_2, PI};

pub fn build() -> R<Body> {
    // the half disk x ≥ 0: the half circle from (0, −5) through (5, 0) to (0, 5), closed by the y axis
    let segs = vec![
        Seg2::Arc { center: v2(0.0, 0.0), radius: 5.0, start: -FRAC_PI_2, sweep: PI },
        Seg2::Line { a: v2(0.0, 5.0), b: v2(0.0, -5.0) },
    ];
    let half = extrude(&Plane::XY, Region2 { outer: Loop2 { segs }, holes: vec![] }, 0.0, 6.0)?;
    // the top rim, radius 1 at its end (0, −5, 6) growing to 2 at (0, 5, 6): the law runs on the
    // edge curve's parameter, which round a circle is its angle
    solidcraft_kernel::fillet_variable(&half, v3(5.0, 0.0, 6.0), v3(0.0, -5.0, 6.0), 1.0, 2.0)
}
