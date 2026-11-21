//! A sheet of revolution about the z axis, a pan: the floor out to radius 5, a bend of radius 1 out to the wall of radius 6, the wall up to z = 8,
//! its faces meeting tangentially; thickened by 3 to the inside, up from the floor and in from the
//! wall. That is more than the bend's radius: the bend's moved face turns inside out and drops, and
//! the moved floor and wall meet in a concave edge.
use crate::Refusal;
use crate::shared::thicken::{cup, thickened};
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let sheet = cup(m, 5.0, 1.0, 8.0)?;
    // the faces' normals point out of the cup, so inside is a negative distance
    thickened(m, sheet, -3.0, Join::Arc)
}
