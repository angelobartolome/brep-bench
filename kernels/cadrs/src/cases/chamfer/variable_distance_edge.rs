//! A cube of side 10 between the origin and (10, 10, 10), the edge where its faces x = 10 and
//! y = 10 meet chamfered with a distance growing along it on the face x = 10, d₁ = 1 + z/5, and a
//! constant 2 on the face y = 10: the chamfer the hyperbolic paraboloid ruled by the horizontal
//! lines from (10, 10 − d₁, z) to (8, 10, z).
//!
//! cadrs's Chamfer has no distance that varies along its edge, so the corner beyond the chamfer is
//! cut away instead: each horizontal section of it is a triangle whose one side lies on the
//! chamfer's line, run on past both faces, and those sides' ends move linearly in z, so the loft
//! of two such triangles, below and above the cube, is bounded by the same hyperbolic paraboloid
//! (every line between corresponding points of its two sections lies on it).
use crate::bench::{block, cut, p3};
use crate::Refusal;
use cadrs_kernel::{BodyId, BodyKind, DefaultKernel, Kernel, LoftEnd, LoftSection, LoftSpec, Loop, Plane, Profile};

/// The distance on the face x = 10, growing along the edge.
fn d1(z: f64) -> f64 {
    1.0 + z / 5.0
}

/// The distance on the face y = 10.
const D2: f64 = 2.0;

/// The corner the chamfer cuts off, at height `z`: the chamfer's line from (10, 10 − d₁) to
/// (10 − d₂, 10) run on half its length past each end, closed round the corner outside the cube.
fn corner(z: f64) -> Profile {
    let (a, b) = ([10.0, 10.0 - d1(z)], [10.0 - D2, 10.0]);
    let past = |p: [f64; 2], q: [f64; 2]| [p[0] + (p[0] - q[0]) / 2.0, p[1] + (p[1] - q[1]) / 2.0];
    Profile::of(Plane::xy(z), vec![Loop::polygon(&[past(a, b), [13.0, 13.0], past(b, a)])])
}

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let cube = block(k, p3(0.0, 0.0, 0.0), p3(10.0, 10.0, 10.0))?;
    let spec = LoftSpec {
        body: BodyKind::Solid,
        sections: vec![LoftSection::Profile(corner(-1.0)), LoftSection::Profile(corner(11.0))],
        start: LoftEnd::default(),
        end: LoftEnd::default(),
        source: 1,
    };
    let beyond = k.loft_with(&spec)?.bodies.first().copied().ok_or("the loft made no body")?;
    cut(k, cube, beyond)
}
