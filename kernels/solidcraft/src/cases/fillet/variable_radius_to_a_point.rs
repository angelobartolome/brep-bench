use crate::solids::*;

pub fn build() -> R<Body> {
    let cube = block((0.0, 0.0, 0.0), (10.0, 10.0, 10.0))?;
    // radius 2 at the end on the bottom, shrinking linearly to nought at the corner (10, 10, 10)
    solidcraft_kernel::fillet_variable(&cube, v3(10.0, 10.0, 5.0), v3(10.0, 10.0, 0.0), 2.0, 0.0)
}
