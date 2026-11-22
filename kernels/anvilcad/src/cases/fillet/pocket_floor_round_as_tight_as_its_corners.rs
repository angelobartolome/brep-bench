//! A block from (−40, −30, 0) to (40, 30, 16) with a pocket cut 10 deep into its top, the
//! rectangle from (−25, −15) to (25, 15) with its corners rounded to radius 4; the rim of the
//! pocket's floor rounded to radius 4, as tight as the corners.
use anvil_brep::{Frame, Solid, extrude};

use crate::shared::{block, e, fillet, region, rounded_rect, rounded_rect_sides, subtract};

pub fn build() -> Result<Solid, String> {
    let (lo, hi) = ((-25., -15.), (25., 15.));
    let cut = extrude::extrude(&Frame::world(), &region(&[rounded_rect(lo, hi, 4.)])?, 6., 17.)
        .map_err(e)?;
    let body = subtract(&block([-40., -30., 0.], [40., 30., 16.])?, &cut)?;
    fillet(&body, &rounded_rect_sides(lo, hi, 4., 6.), 4.)
}
