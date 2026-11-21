//! No layer has a conic blend (the kernel's and ops's blends have a circular, curvature-continuous,
//! chord-sized or flat section; none takes a rho), so, as in `fillet/conic-edge`, the cylinder is
//! cut by the corner outside the parabola instead: the quadratic Bézier from (3, 10) to (5, 8)
//! about the corner (5, 10) in ρ and z (a conic of weight 1), closed round the corner outside the
//! cylinder and turned round the z axis.
use crate::solids::*;
use solidcore_record::Seg2;

pub fn build() -> R<Body> {
    let c = zcyl(0.0, 0.0, 5.0, 0.0, 10.0)?;
    let (on_top, on_side) = (v2(3.0, 10.0), v2(5.0, 8.0));
    let mut segs = vec![Seg2::Conic { a: on_top, apex: v2(5.0, 10.0), b: on_side, w: 1.0 }];
    let around = [on_side, v2(6.0, 8.0), v2(6.0, 11.0), v2(3.0, 11.0), on_top];
    segs.extend(around.windows(2).map(|p| Seg2::Line { a: p[0], b: p[1] }));
    let outside = Region2 { outer: Loop2 { segs }, holes: vec![] };
    cut(&c, &revolve_z(outside, 360.0)?)
}
