// An elbow tighter than its tube: a tube of radius 20 turned a quarter turn about the y axis,
// right-handed, its centre-line on the circle of radius 18 from (0, 0, 18) to (18, 0, 0), and a
// straight leg of length 40 back from each end — along −x from the disk `rim` in the side plane
// (x = 0) and along −z from the disk `foot` in the top plane (z = 0) — all in union.
// The turn is every point `rim`'s disk passes through as it turns. The disk crosses the axis,
// which a revolution refuses ("the revolution axis crosses the profile"), and its part beyond the
// axis turns into the leg along −x; so the turn is the disk cut at the axis: the long arc from
// `s` round through (0, 0, 38) to `e`, both on the axis, closed along it by `chord`.
unit mm
use std
use std.solids
use std.solids (union)

in std.side {
  c := point
  fix((0, 18)) c
  rim := circle(center: c) hint(r: 20)
  radius(20mm) rim
  s := point hint((8.7, 0))
  e := point hint((-8.7, 0))
  fix(y == 0mm) s
  fix(y == 0mm) e
  outer := arc(center: c, start: s, end: e)
  radius(20mm) outer
  chord := line(e, s)
}
in std.top {
  k := point
  fix((18, 0)) k
  foot := circle(center: k) hint(r: 20)
  radius(20mm) foot
}

turn := solid(std.solids.Revolution(face(outer, chord), about: std.y, from: 0deg, to: 90deg))
leg_x := solid(std.solids.Prism(face(rim), from: -40mm, to: 0mm))
leg_z := solid(std.solids.Prism(face(foot), from: -40mm, to: 0mm))
part := solid(turn)
leg_x union part
leg_z union part
