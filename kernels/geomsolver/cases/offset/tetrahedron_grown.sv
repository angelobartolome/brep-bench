// The regular tetrahedron on the corners (3, 3, 3), (3, −3, −3), (−3, 3, −3) and (−3, −3, 3), grown
// by 1. Its three axes of symmetry are the world's: one corner and its half turns about x, y and z
// are its four. Its three edges from that corner are equal, which makes it regular, their length an
// unknown the solve finds (6√2); it stands 6 tall, the corner 6 above its half turn about x; and
// the corner lies toward +x of its half turn about y and toward +y of its half turn about z, which
// says which of the four roots is meant. It is the hull of its corners, written as the join of two
// opposite edges: the segments from some point of one edge to some point of the other. Grown, it is
// the points a ball of radius 1 about some point of it reaches.
unit mm
use std
use std.sets

param edge: Length hint(8mm)
a := point
b := point
c := point
d := point
a symmetry(std.x) b
a symmetry(std.y) c
a symmetry(std.z) d
a distance(edge) b
a distance(edge) c
a distance(edge) d
b distance(6mm, along: std.z) a
c distance(>= 0mm, along: std.x) a
d distance(>= 0mm, along: std.y) a

tetrahedron := solid({ p | some q in std.sets.Segment(a, b), r in std.sets.Segment(c, d):
  p in std.sets.Segment(q, r) })
part := solid({ p | some q in tetrahedron: p distance(<= 1mm) q })
