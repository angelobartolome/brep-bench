//! A block, the box from (−6, −6, −8) to (36, 26, 0), grooved by a ring standing across a spline:
//! the torus centred at (0, 0, 0.5), axis along y, radii 1.5 and 0.5, in a frame carried along the
//! cubic Bézier curve through control points (0, 0, 0), (10, 0, 0), (20, 20, 0) and (30, 20, 0),
//! its x axis the curve's tangent and its z axis up.
//!
//! Refused. The frame along a Bézier curve is no motion of kentron's cut by a motion
//! (`sweep::cut_by_motion`), whose steps are screws and turns at constant rates, and `sweep::pipe`
//! takes no solid. Where the curve bends the torus touches its sweep along curves that are none
//! of its circles, an envelope kentron has no operation to find.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron has no sweep of a solid along a curve's frame (`sweep::pipe` refuses a solid, and \
         `sweep::cut_by_motion` takes screws and turns at constant rates only); the groove is the \
         envelope of the torus along the curve, which kentron has no operation to find",
    ))
}
