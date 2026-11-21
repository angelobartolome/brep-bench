//! A sheet of revolution about the z axis, a narrow cup: the floor out to radius 3, a bend of radius 2 out to the wall of radius 5, the wall up to z = 10,
//! its faces meeting tangentially; thickened by 4 to the inside, up from the floor and in from the
//! wall. That is more than the bend's radius: the bend's moved face turns inside out and drops, and
//! the moved floor and wall meet in a concave edge.
use crate::Refusal;
use crate::shared::thicken::{cup, thickened};
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let sheet = cup(m, 3.0, 2.0, 10.0)?;
    // the faces' normals point out of the cup, so inside is a negative distance
    thickened(m, sheet, -4.0, Join::Arc)
}
