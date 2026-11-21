//! The half x ≥ 0 of the cylinder of radius 5 on the z axis from z = 0 to z = 6, its curved top
//! rim chamfered by d = 3/2 + φ/π on both faces, φ the angle round the axis from +x toward +y.
//!
//! No layer has a chamfer whose distance varies along its edge (the kernel's and ops's chamfers
//! hold their distances along it, and solidcore's chamfer section is a chord of two constant
//! offsets), so what the chamfer takes off is cut away instead. In the half plane at φ it is the
//! part of the section ρ ≤ 5, z ≤ 6 where ρ + z > 11 − d = 19/2 − φ/π: the half plane at 0°'s
//! line ρ + z = 19/2, lowered by φ/π. Turning by φ and lowering by φ/π is the screw about the z axis
//! that falls 2 a turn, so the corner beyond the chamfer is a triangle beyond that line, reaching
//! past the cylinder's side and top, screwed from −100° to 100° by the kernel's helical sweep
//! (`sweep_helix`) and cut from the half cylinder. Its wall on the line is the kernel's helical
//! sweep of it, which holds the helicoid to a ten-thousandth of the pitch; the helicoid meets the
//! top along the spiral ρ = 5 − d and the side along the helix z = 6 − d. The sweeps of the
//! triangle's other sides stay outside the half cylinder.
use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::FRAC_PI_2;

/// The screw's lead: the fall of the chamfer's line, d's growth, over a turn.
const LEAD: f64 = -2.0;

pub fn build() -> R<Body> {
    let half = vec![
        Seg2::Arc { center: v2(0.0, 0.0), radius: 5.0, start: -FRAC_PI_2, sweep: 2.0 * FRAC_PI_2 },
        Seg2::Line { a: v2(0.0, 5.0), b: v2(0.0, -5.0) },
    ];
    let cylinder = extrude(&Plane::XY, Region2 { outer: Loop2 { segs: half }, holes: vec![] }, 0.0, 6.0)?;
    // the triangle in the half plane at 0°, in ρ and z (`xz`): beyond ρ + z = 19/2, within ρ ≤ 7
    // and z ≤ 9, clear of the axis
    let corner = polygon(&[(7.0, 2.5), (7.0, 9.0), (0.5, 9.0)]);
    // carried to where the screw starts, the turn of −100°
    let (start, stop) = (-100f64, 100f64);
    let (s, c) = start.to_radians().sin_cos();
    let at = Plane { origin: v3(0.0, 0.0, LEAD * start / 360.0), x: v3(c, s, 0.0), y: Vec3::Z };
    let swept = solidcraft_kernel::sweep_helix(&at, &corner, Vec3::ZERO, Vec3::Z, LEAD, (stop - start) / 360.0)?;
    cut(&cylinder, &swept)
}
