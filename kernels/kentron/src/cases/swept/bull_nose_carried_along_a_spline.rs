//! A block, the box from (−6, −6, −8) to (36, 26, 0), grooved by a leaning bull-nose mill (about
//! the axis through (0, 0, −1) along (sin 30°, 0, cos 30°), radius 2, its end corner rounded to 0.5)
//! carried rigidly by the frame riding the cubic Bézier curve with control points (0, 0, 0),
//! (10, 0, 0), (20, 20, 0), (30, 20, 0), its x along the curve and its z up. Everything the mill
//! passes through, cut.
//!
//! Refused. kentron has no sweep of a solid, and `sweep::cut_by_motion` takes only screws and turns
//! of a solid of revolution about one fixed axis, or a prism moved in its plane. Here the frame
//! turns the leaning mill as the curve turns, so the curve along which it touches what it sweeps
//! moves over the mill from instant to instant: the groove's wall is the envelope of the moving
//! mill, which kentron has no operation to find. (Upright, its section through its axis piped along
//! the curve would do; leaning, no section of it is what it sweeps.)
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron has no sweep of a solid along a moving frame, and its cut by a motion takes only \
         screws and turns about one axis: the leaning mill, turned as the curve turns, touches its \
         sweep along a curve that moves over it, an envelope kentron has no operation to find",
    ))
}
