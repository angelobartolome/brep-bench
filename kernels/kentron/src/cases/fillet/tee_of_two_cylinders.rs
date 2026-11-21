//! A tee of two solid cylinders joined: the bar of radius 6 about the x axis from x = −20 to 20,
//! and the stem of radius 3 about the z axis from z = 0 to 15. The saddle-shaped curve where the
//! stem's side meets the bar's rounded to radius 1, adding material: a rolling-ball round that is
//! no torus.
use crate::bench::{edge_through, frame, p};
use crate::Refusal;
use kentron::blend;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let bar = Cylinder::new(6.0, 40.0).with_frame(frame(p(-20.0, 0.0, 0.0), Direction::X)).build(m)?;
    let stem = Cylinder::new(3.0, 15.0).build(m)?;
    let tee = boolean::union(m, bar, stem)?;
    // (The junction over y = 3, at its lowest, z = √27.)
    let junction = edge_through(m, tee, p(0.0, 3.0, 27f64.sqrt()))?;
    Ok(blend::fillet(m, tee, &[junction], 1.0)?)
}
