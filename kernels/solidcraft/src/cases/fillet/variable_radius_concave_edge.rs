use crate::solids::*;

pub fn build() -> R<Body> {
    let l = polygon(&[(0.0, 0.0), (20.0, 0.0), (20.0, 4.0), (6.0, 4.0), (6.0, 12.0), (0.0, 12.0)]);
    // the plane of `xz` has its normal along −y: from y = 10 to y = 0
    let block = extrude(&xz(), l, -10.0, 0.0)?;
    // the inside edge, radius 1 at its end on y = 0 growing linearly to 3 at y = 10
    solidcraft_kernel::fillet_variable(&block, v3(6.0, 5.0, 4.0), v3(6.0, 0.0, 4.0), 1.0, 3.0)
}
