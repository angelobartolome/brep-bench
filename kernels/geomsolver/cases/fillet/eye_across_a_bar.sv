// An eye across a bar. The bar, the box from (−40, −8, −5) to (40, 8, 5), its footprint drawn in
// the top plane; the eye, the circle of radius 14 about the origin drawn in the front plane, its
// prism from y = −8 to y = 8, flush with the bar's sides. Joined, and where the eye's side meets
// the bar's top (`near`) and its bottom (`far`) — four straight edges along y — rounded to radius 3,
// filled, each round running the bar's whole width.
unit mm
use std
use std.solids
use std.solids (union)
use std.rounds (round)

in std.top {
  bar_sq := step(u: 40mm, v: -8mm) (bs := line) -> step(v: 8mm) (be := line) ->
            step(u: -40mm) (bn := line) -> step(v: -8mm) (bw := line) -> close
}
in std.front {
  o := point
  fix((0, 0)) o
  eye_c := circle(center: o)
  radius(14mm) eye_c
}

part := solid(std.solids.Prism(bar_sq, from: -5mm, to: 5mm))
eye := solid(std.solids.Prism(face(eye_c), from: -8mm, to: 8mm))
eye union part
part.eye.eye_c round(r: 3mm) part.near
part.eye.eye_c round(r: 3mm) part.far
