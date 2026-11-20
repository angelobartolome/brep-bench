// The solid a regular tetrahedron fills as it is carried along a screw. The tetrahedron is centred
// 4 along x, and its three axes of symmetry run through its centre along x, y and z: one corner and
// its half turns about the three are its four, (5, 1, 1), (5, -1, -1), (3, 1, -1) and (3, -1, 1).
// Its three edges from that corner are equal, which makes it regular, their length an unknown the
// solve finds (2√2); it stands 2 tall, the corner 2 above its half turn about x; and the corner
// lies toward +x of its half turn about y and toward +y of its half turn about z, which says which
// of the four roots is meant. It is the join of two opposite edges: every point of a segment
// between a point of the one and a point of the other. The screw is about z, right-handed, rising 5
// a turn; the part is the kernel's sweep of the tetrahedron from a turn of 0 to 540 degrees.
unit mm
use std
use std.sets
use std.motion (screws)

centre := point
centre coincident std.x
std.origin distance(4mm, along: std.x) centre
along_y := axis
along_y parallel(sense: same) std.y
centre coincident along_y
along_z := axis
along_z parallel(sense: same) std.z
centre coincident along_z

param edge: Length hint(3mm)
a := point
b := point
c := point
d := point
a symmetry(std.x) b
a symmetry(along_y) c
a symmetry(along_z) d
a distance(edge) b
a distance(edge) c
a distance(edge) d
b distance(2mm, along: std.z) a
c distance(>= 0mm, along: std.x) a
d distance(>= 0mm, along: std.y) a
construction tetrahedron := solid({ p | some q in std.sets.Segment(a, b), r in std.sets.Segment(c, d):
  p in std.sets.Segment(q, r) })

feed := frame
feed screws(5mm) std.z
part := solid({ p | some t in (0deg, 540deg): p in tetrahedron at(under: feed) t })
