//! Two slabs united along the plane z = 4, where each has a flat Bézier face. The first is the box
//! from the origin to (10, 10, 4), its top the bicubic patch on the poles (xᵢ, yⱼ, 4), x = 0, 2,
//! 7, 10 and y = 0, 4, 5, 10, swept down 4. The second is the box from (5, 3, 4) to (15, 13, 7),
//! its bottom the biquadratic patch on (xᵢ, yⱼ, 4), x = 5, 12, 15 and y = 3, 4, 13, swept up 3.
//! The two patches overlap on 5 ≤ x ≤ 10, 3 ≤ y ≤ 10.
use crate::bench::{p, v};
use kentron::geometry::surface::{BezierSurface, Surface};
use kentron::topology::{Model, Shape};
use kentron::{boolean, make, sweep};

/// The slab the flat Bézier patch on the poles (xᵢ, yⱼ, 4) sweeps along `(0, 0, dz)`.
fn slab(m: &mut Model, xs: &[f64], ys: &[f64], dz: f64) -> kentron::Result<Shape> {
    let poles = xs.iter().map(|&x| ys.iter().map(|&y| p(x, y, 4.0)).collect()).collect();
    let face = make::face_on_surface(m, Surface::Bezier(BezierSurface::new(poles)))?;
    Ok(sweep::extrude(m, face, v(0.0, 0.0, dz))?.shape)
}

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let lower = slab(m, &[0.0, 2.0, 7.0, 10.0], &[0.0, 4.0, 5.0, 10.0], -4.0)?;
    let upper = slab(m, &[5.0, 12.0, 15.0], &[3.0, 4.0, 13.0], 3.0)?;
    boolean::union(m, lower, upper)
}
