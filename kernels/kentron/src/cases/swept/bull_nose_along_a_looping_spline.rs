//! A block, the box from (−10, −10, −8) to (30, 25, 0), grooved by an upright bull-nose mill (a
//! cylinder of radius 2 from z = −1.5 to 5, its bottom corner rounded to 0.5) whose axis runs the
//! cubic Bézier curve with control points (0, 0, 0), (30, 20, 0), (−10, 20, 0), (20, 0, 0), which
//! loops over itself. Everything the mill passes through, cut.
//!
//! Refused. kentron has no sweep of a solid. Elementary geometry gives the upright mill's sweep as
//! its section through its axis, square to the curve, piped along it (`sweep::pipe`) with the mill
//! at each end, but only where the curve turns no tighter than the mill: this one turns on a radius
//! of about 1.79, under the mill's 2, so that section's sweep folds through itself there, and the
//! groove's inner wall is only the part of the folded sheet the mill does not cut away again,
//! trimmed along the curve's own offset, which kentron has no operation to find.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron has no sweep of a solid, and the mill's section piped along this spline, which turns \
         on a radius of 1.79, under the mill's 2, folds through itself: trimming the fold back to the \
         groove's wall is an envelope kentron does not find",
    ))
}
