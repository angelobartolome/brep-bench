//! Refused. The rim's radius grows linearly with the angle φ round the axis, r = 3/2 + φ/π. Every
//! layer's variable round (solidcraft-kernel's and solidcore-ops's `fillet_variable`, on
//! solidcore's variable-radius solver) takes its radius as a B-spline of the edge curve's
//! parameter, and an arc's parameter is its rational Bézier parameter, not its angle: no
//! polynomial of it is linear in the angle, so no layer can give this round its law.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("no layer (solidcraft-kernel, solidcore-ops, solidcore) has a round whose radius varies \
         with the angle round an arc: every variable round takes its radius as a B-spline of the \
         edge curve's parameter, and an arc's parameter is its rational Bezier parameter, not its \
         angle"
        .into())
}
