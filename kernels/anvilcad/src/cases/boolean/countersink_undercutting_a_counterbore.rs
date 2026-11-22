//! A disc of radius 100 from z = −20 to 20, counterbored to radius 60 from z = 0 up, with a
//! countersunk hole of radius 3.5 from z = −15 up to the floor, its axis at (59.9, 0), 0.1 inside
//! the counterbore's wall, its 90° countersink 4 across the floor: hole and countersink reach
//! under the wall.
use anvil_brep::Solid;

use crate::shared::{at, cylinder, polygon, subtract, turned};

pub fn build() -> Result<Solid, String> {
    let disc = cylinder(&at([0., 0., 0.]), 100., -20., 20.)?;
    let bored = subtract(&disc, &cylinder(&at([0., 0., 0.]), 60., 0., 21.)?)?;
    // the hole and its countersink as one tool, as a hole feature cuts them: the half section
    // revolved about the hole's axis
    let drill = turned(
        &at([59.9, 0., 0.]),
        polygon(&[(0., -15.), (3.5, -15.), (3.5, -0.5), (4., 0.), (0., 0.)]),
    )?;
    subtract(&bored, &drill)
}
