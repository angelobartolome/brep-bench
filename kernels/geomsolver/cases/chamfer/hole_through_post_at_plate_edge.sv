// A square plate from (−50, −50) to (50, 50) in the top plane, 20 thick, and a post of radius 15
// about (45, 0), from z = 0 to z = 40, joined to it: its axis 5 inside the plate's side x = 50, so
// a segment of it overhangs, its foot flush with the plate's bottom. A hole of radius 6 drilled
// right through along the post's axis, and both of its rims — on the bottom face, across the line
// where the plate's bottom and the post's foot meet in one plane, and on the post's top —
// chamfered 1.5 along each face.
unit mm
use std
use std.solids
use std.solids (cut, union)
use std.rounds (chamfer)

in std.top {
  a := point hint((-50, -50))
  b := point hint((50, -50))
  c := point hint((50, 50))
  d := point hint((-50, 50))
  outline := horizontal line(a, b) -> vertical line(b, c) ->
             horizontal line(c, d) -> vertical line(d, a) -> close
  fix((-50, -50)) a
  a distance(100mm) b
  b distance(100mm) c

  o := point hint((45, 0))
  fix((45, 0)) o
  rim := circle(center: o) hint(r: 15)
  radius(15mm) rim
  bore := circle(center: o) hint(r: 6)
  radius(6mm) bore
}

plate := solid(std.solids.Prism(outline, from: 0mm, to: 20mm))
post := solid(std.solids.Prism(face(rim), from: 0mm, to: 40mm))
post union plate
drill := solid(std.solids.Through(face(bore)))
part := solid(plate)
drill cut part
part.drill chamfer(d: 1.5mm) part.plate        // the hole's two rims: its wall against every face
