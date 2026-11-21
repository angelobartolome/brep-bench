//! The cube from the origin to (10, 10, 10), its edge where the faces x = 10 and y = 10 meet
//! chamfered by d₁ = 1 + z/5 on the face x = 10 and d₂ = 2 on the face y = 10.
//!
//! No layer has a chamfer whose distance varies along its edge (the kernel's and ops's chamfers
//! hold their distances along it, and solidcore's chamfer section is a chord of two constant
//! offsets), so the corner beyond the chamfer is cut away instead. At height z the chamfer's
//! section is the line from P₁(z) = (10, 10 − d₁) to P₂ = (10 − d₂, 10), and the chamfer the
//! surface (1 − s)P₁(z) + sP₂ those lines rule. For a fixed s that point runs linearly in z, so the
//! ruled loft between two of the lines, each run on past both faces (s from −¼ to 5⁄4), is that
//! same hyperbolic paraboloid. Each line is closed round the corner, outside the cube, to (12, 12),
//! at z = −1 and z = 11, the two triangles lofted and the loft cut from the cube.
use crate::solids::*;

pub fn build() -> R<Body> {
    let cube = block((0.0, 0.0, 0.0), (10.0, 10.0, 10.0))?;
    let (d1, d2) = (|z: f64| 1.0 + z / 5.0, 2.0);
    let on_line = |s: f64, z: f64| {
        let (p1, p2) = (v2(10.0, 10.0 - d1(z)), v2(10.0 - d2, 10.0));
        p1 * (1.0 - s) + p2 * s
    };
    let section = |z: f64| {
        let level = Plane { origin: v3(0.0, 0.0, z), x: Vec3::X, y: Vec3::Y };
        (level, Loop2::polygon(&[on_line(-0.25, z), v2(12.0, 12.0), on_line(1.25, z)]))
    };
    let corner = solidcraft_kernel::loft(&[section(-1.0), section(11.0)])?;
    cut(&cube, &corner)
}
