//! The box from (−40, −8, −5) to (40, 8, 5) united with the cylinder of radius 14 about the y axis
//! from y = −8 to 8; the four concave edges where the eye's side meets the bar's top and bottom
//! rounded to radius 3.
use anvil_brep::Solid;

use crate::shared::{block, fillet, frame, cylinder, union};

pub fn build() -> Result<Solid, String> {
    let eye = cylinder(&frame([0., 0., 0.], [0., 1., 0.], [1., 0., 0.])?, 14., -8., 8.)?;
    let body = union(&block([-40., -8., -5.], [40., 8., 5.])?, &eye)?;
    let x = 171f64.sqrt();
    fillet(
        &body,
        &[[x, 0., 5.], [-x, 0., 5.], [x, 0., -5.], [-x, 0., -5.]],
        3.,
    )
}
