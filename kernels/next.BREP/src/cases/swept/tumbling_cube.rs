//! The solid a cube fills as it tumbles: turned by t/4 about the vertical line through (2, 0, 0),
//! then by −t about the line through (0, 0.5, 0.5) along x, for t from −75° to 75°.
//!
//! Through the kernel's tumbled sweep (`sweep_solid_tumbled`), asked for the cube as a polyhedron
//! (the box (2.4, −0.6, 1.4)..(3.6, 0.6, 2.6), its six faces as loops of its corners) along those
//! two turns. The kernel sweeps a polyhedron by two turns only through its trim engine, which is
//! off by default (`BREP_SWEEP_TRIM_ENGINE`); off, it refuses.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{sweep_solid_tumbled, BrepSolid, SolidTool, TwoTurns};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (lo, hi) = ([2.4, -0.6, 1.4], [3.6, 0.6, 2.6]);
    // Corner i: x high if i & 4, y high if i & 2, z high if i & 1.
    let vertices = (0..8)
        .map(|i| {
            let pick = |k: usize, bit: usize| if i & bit != 0 { hi[k] } else { lo[k] };
            p(pick(0, 4), pick(1, 2), pick(2, 1))
        })
        .collect();
    let faces = vec![vec![4, 5, 7, 6], vec![0, 2, 3, 1], vec![2, 6, 7, 3], vec![0, 1, 5, 4], vec![1, 3, 7, 5], vec![0, 4, 6, 2]];
    let cube = SolidTool::Polyhedron { vertices, faces };
    let tumble = TwoTurns {
        axes: [(p(2.0, 0.0, 0.0), p(0.0, 0.0, 1.0)), (p(0.0, 0.5, 0.5), p(1.0, 0.0, 0.0))],
        rates: [0.25, -1.0],
        from: (-75f64).to_radians(),
        to: 75f64.to_radians(),
    };
    Ok(sweep_solid_tumbled(&cube, &tumble)?)
}
