//! The box from the origin to (40, 30, 8), drilled right through along z by two holes of radius 3
//! about the vertical lines through (15, 15) and (25, 15). Both walls are offset 3 into the plate,
//! away from their axes, to radius 6: the holes overlap, and each wall is trimmed by the other
//! into one opening.
use crate::bench::{face_through, frame, p};
use crate::shared::edit::{drilled_block, moved};
use crate::Refusal;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let plate = drilled_block(m, [40.0, 30.0, 8.0], 15.0, 15.0, 3.0)?;
    let drill = Cylinder::new(3.0, 10.0).with_frame(frame(p(25.0, 15.0, -1.0), Direction::Z)).build(m)?;
    let plate = boolean::difference(m, plate, drill)?;
    let left = face_through(m, plate, p(12.0, 15.0, 4.0))?;
    let right = face_through(m, plate, p(28.0, 15.0, 4.0))?;
    // Each wall's outward normal points into its hole, toward its axis: away from it is the other
    // way.
    Ok(moved(m, plate, &[(left, -3.0), (right, -3.0)])?)
}
