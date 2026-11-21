//! Refused. The ball has no orientation to keep, so its sweep is the tube of radius 1 about its
//! centre's path with a half ball at each end (the path turning no tighter than 1.75). But that
//! path, (5 cos πt + 25t, 5 sin πt, 0), is a curtate trochoid: transcendental, so none of the
//! lines, arcs, conics, Béziers or NURBS a path or curve of any layer is made of. No layer has a
//! sweep of a solid, a trochoid, or a sweep along a motion that turns and slides at once, and a
//! spline fitted to the trochoid here would be geometry made outside the kernel.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("no layer (solidcraft-kernel, solidcore-ops, solidcore) has a sweep of a solid, a trochoid, \
         or a sweep along a motion that turns and slides at once: the ball's sweep is the tube \
         about its centre's path, a curtate trochoid, which no layer's lines, arcs, conics, \
         Béziers or NURBS can hold"
        .into())
}
