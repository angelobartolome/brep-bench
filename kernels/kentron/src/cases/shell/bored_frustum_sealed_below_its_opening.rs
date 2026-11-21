//! The cone frustum on the z axis from the disc of radius 6 at z = 0 to the disc of radius 3 at z = 4,
//! bored through along the axis by a hole of radius 1, shelled inward to a wall 1 thick, open at its
//! top. The side, the base and the bore, each moved 1 inward and extended until they meet, close the
//! cavity below the opening.
use crate::Refusal;
use crate::bench::{frame, p};
use crate::shared::offset::frustum;
use crate::shared::shell;
use kentron::boolean;
use kentron::math::Direction;
use kentron::offset::Join;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let solid = frustum(m, 6.0, 3.0, 4.0)?;
    let drill = Cylinder::new(1.0, 6.0).with_frame(frame(p(0.0, 0.0, -1.0), Direction::Z)).build(m)?;
    let bored = boolean::difference(m, solid, drill)?;
    // open at the top, the annulus at z = 4
    shell::inward(m, bored, &[p(0.0, 2.0, 4.0)], 1.0, Join::Arc)
}
