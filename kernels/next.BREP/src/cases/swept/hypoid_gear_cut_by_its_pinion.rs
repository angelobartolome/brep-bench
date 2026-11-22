//! The straight bevel case's blank with one tooth space cut by a pinion tooth on a shaft that
//! misses the gear's (a hypoid): the tooth is the straight bevel case's frustum of a pyramid with
//! apex O, turned by φ (sin φ = 24/(75 − 5√5)) about the gear's axis; the pinion turns by t about
//! its axis (through (0, 12, 0) along x) while the gear turns by −7t/15 about z, t from −70° to 70°.
//!
//! Seen from the blank the tooth is turned by t about the pinion's axis and then by 7t/15 about z:
//! two turns about skew axes, which the kernel sweeps whole for a convex polyhedron
//! (`sweep_solid_tumbled`). The tooth is that polyhedron, its 8 corners the trapezoid's at the two
//! end planes, and its sweep is cut from the blank, the case's quadrilateral revolved.
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{boolean_operation_nary, revolve_profile_brep, sweep_solid_tumbled, BooleanOperation, BrepSolid, SolidTool, TwoTurns, Vec3};
use std::f64::consts::{FRAC_PI_2, TAU};

const S5: f64 = 2.236_067_977_499_79;

pub fn build() -> Result<BrepSolid, Refusal> {
    // g along the pitch cone's generator, d from the pinion into the gear, e = y (at azimuth 0).
    let (g, d, e) = (p(2.0 / S5, 0.0, 1.0 / S5), p(-1.0 / S5, 0.0, 2.0 / S5), p(0.0, 1.0, 0.0));
    let s0 = 15.0 * S5;
    let t20 = 20f64.to_radians().tan();
    let trapezoid = [[-2.5, FRAC_PI_2 + 2.5 * t20], [2.5, FRAC_PI_2 - 2.5 * t20], [2.5, -(FRAC_PI_2 - 2.5 * t20)], [-2.5, -(FRAC_PI_2 + 2.5 * t20)]];
    let phi = (24.0 / (75.0 - 5.0 * S5)).asin();
    let turned = |x: Vec3| p(phi.cos() * x.x - phi.sin() * x.y, phi.sin() * x.x + phi.cos() * x.y, x.z);
    // The trapezoid at P = s0·g, scaled from O to the end planes square to g at s0 − 12 and s0 + 2.
    let mut vertices = Vec::new();
    for s in [s0 - 12.0, s0 + 2.0] {
        for q in trapezoid {
            vertices.push(turned(g.scale(s0).add(d.scale(q[0])).add(e.scale(q[1])).scale(s / s0)));
        }
    }
    let faces = vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![0, 1, 5, 4], vec![1, 2, 6, 5], vec![2, 3, 7, 6], vec![3, 0, 4, 7]];
    let tooth = SolidTool::Polyhedron { vertices, faces };
    let deg = 1f64.to_radians();
    let motion = TwoTurns {
        axes: [(p(0.0, 12.0, 0.0), p(1.0, 0.0, 0.0)), (p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0))],
        rates: [1.0, 7.0 / 15.0],
        from: -70.0 * deg,
        to: 70.0 * deg,
    };
    let space = sweep_solid_tumbled(&tooth, &motion)?;
    let q = [(30.894427, 13.211146), (27.316718, 20.366563), (18.372447, 15.894427), (21.683489, 9.272343)];
    let meridian = polygon(&q.iter().map(|&(x, z)| p(x, 0.0, z)).collect::<Vec<_>>())?;
    let blank = revolve_profile_brep(&meridian, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), TAU)?;
    Ok(boolean_operation_nary(&[blank, space], BooleanOperation::Subtract)?)
}
