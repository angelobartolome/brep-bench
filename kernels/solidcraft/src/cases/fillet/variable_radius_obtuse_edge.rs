use crate::solids::*;

pub fn build() -> R<Body> {
    let corners: Vec<(f64, f64)> = (0..6).map(|k| (60.0 * k as f64).to_radians()).map(|a| (6.0 * a.cos(), 6.0 * a.sin())).collect();
    let prism = extrude(&Plane::XY, polygon(&corners), 0.0, 10.0)?;
    // the edge through (6, 0), radius 1 at the bottom growing linearly to 4 at the top
    solidcraft_kernel::fillet_variable(&prism, v3(6.0, 0.0, 5.0), v3(6.0, 0.0, 0.0), 1.0, 4.0)
}
