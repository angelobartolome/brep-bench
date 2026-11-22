// A disc of radius 100 about the z axis, from z = −20 to z = 20, counterbored to radius 60 from the
// floor z = 0 up through its top. A hole of radius 3.5 about the vertical line through (59.9, 0),
// 0.1 inside the counterbore's wall, 15 deep from the floor and flush with it, countersunk 90° to a
// circle of radius 4 on the floor plane: the hole and its countersink reach under the wall.
unit mm
use std
use std.solids
use std.solids (cut)

in std.top {
  o := point
  fix((0, 0)) o
  rim := circle(center: o) hint(r: 100)
  radius(100mm) rim
  bore := circle(center: o) hint(r: 60)
  radius(60mm) bore

  h := point hint((59.9, 0))
  fix((59.9, 0)) h
  drill := circle(center: h) hint(r: 3.5)
  radius(3.5mm) drill
  sink := circle(center: h) hint(r: 4)
  radius(4mm) sink
}

part := solid(std.solids.Prism(face(rim), from: -20mm, to: 20mm))
counterbore := solid(std.solids.Prism(face(bore), from: 0mm, to: 20mm))
counterbore cut part
hole := solid(std.solids.Prism(face(drill), from: -15mm, to: 0mm))
hole cut part
countersink := solid(std.solids.Countersink(face(sink), angle: 90deg))   // below the floor plane
countersink cut part
