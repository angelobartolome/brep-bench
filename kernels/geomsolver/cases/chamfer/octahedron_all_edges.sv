// The octahedron |x| + |y| + |z| <= 10, every edge chamfered √6 on each face, and each corner cut
// flat by a square. Its middle square, drawn in the top plane, is a prism from z = -10 to z = 10,
// and from it is cut everything beyond each of its eight faces — the plane through three corners,
// its normal out — by a disc drawn on that plane about a corner and swept out along the normal past
// the prism. Every edge among its faces is then chamfered √6, measured across the face square to
// the edge: the library's `chamfer(d: …)` takes its leg as the distance from the other face, so `d`
// is that leg times the sine of the dihedral angle between two faces, cos θ = -1/3. The library's
// flats mitre at a corner, the four meeting in a point (chamfer/octahedron-all-edges-mitered), and
// it has no flat closing a corner, so a solid of the chamfered octahedron is cut by the six corner
// planes |x|, |y|, |z| <= 6. Its six corners are on the axes, each opposite one the half turn of
// the other: the middle square's corners, drawn in the top plane, are where it meets the front and
// side planes, the origin midway between opposite ones, and the top corner is on z with the bottom
// its half turn about x. The edges between the corners on +x, +y and +z read one unknown length the
// solve finds (10√2), which makes it regular, and it stands 20 tall, the top 20 above the bottom;
// the corners on +x and +y lie toward +x and +y of their opposites, which says which root is meant,
// so no corner is seeded or held.
unit mm
use std
use std.solids
use std.solids (cut)
use std.rounds (chamfer)

in std.top {
  px := point
  py := point
  nx := point
  ny := point
  square := line(px, py) -> line(py, nx) -> line(nx, ny) -> line(ny, px) -> close
  px coincident std.front
  py coincident std.side
  std.top.origin midpoint line(nx, px)
  std.top.origin midpoint line(ny, py)
  nx distance(>= 0mm, along: std.x) px
  ny distance(>= 0mm, along: std.y) py
}
pz := point
nz := point
pz coincident std.z
pz symmetry(std.x) nz
nz distance(20mm, along: std.z) pz
param edge: Length hint(14mm)
px distance(edge) py
py distance(edge) pz
pz distance(edge) px

// a face of the octahedron, through corners `a`, `b`, `c` running counter-clockwise seen from
// outside: its plane over its edges from `a`, and the material beyond it, a disc of radius 40
// about `a` swept 40 out along the plane's normal
component Beyond(a: point, b: point, c: point) {
  ab := line(a, b)
  ac := line(a, c)
  flat := plane(u: ab, v: ac)
  in flat {
    rim := circle(center: flat.origin) hint(r: 40)
    radius(40mm) rim
  }
  cutter := solid(std.solids.Prism(face(rim), from: 0mm, to: 40mm))
}

octahedron := solid(std.solids.Prism(square, from: -10mm, to: 10mm))

ppp := Beyond(px, py, pz)
npp := Beyond(nx, pz, py)
pnp := Beyond(px, pz, ny)
nnp := Beyond(nx, ny, pz)
ppn := Beyond(px, nz, py)
npn := Beyond(nx, py, nz)
pnn := Beyond(px, ny, nz)
nnn := Beyond(nx, nz, ny)
ppp.cutter cut octahedron
npp.cutter cut octahedron
pnp.cutter cut octahedron
nnp.cutter cut octahedron
ppn.cutter cut octahedron
npn.cutter cut octahedron
pnn.cutter cut octahedron
nnn.cutter cut octahedron

chamfer(d: sqrt(6) * sin(acos(-1 / 3)) * 1mm) octahedron

// the chamfered octahedron, each corner then cut flat by the plane 6 from the origin square to
// its axis, |x| <= 6, |y| <= 6, |z| <= 6: a slab beyond it, a disc in the datum plane square to
// that axis swept from 6 to 20 along its normal (the side's +x, the front's -y, the top's +z)
part := solid(octahedron)
in std.side {
  across_x := circle(center: std.side.origin) hint(r: 20)
  radius(20mm) across_x
}
in std.front {
  across_y := circle(center: std.front.origin) hint(r: 20)
  radius(20mm) across_y
}
in std.top {
  across_z := circle(center: std.top.origin) hint(r: 20)
  radius(20mm) across_z
}
east := solid(std.solids.Prism(face(across_x), from: 6mm, to: 20mm))
west := solid(std.solids.Prism(face(across_x), from: -20mm, to: -6mm))
north := solid(std.solids.Prism(face(across_y), from: -20mm, to: -6mm))
south := solid(std.solids.Prism(face(across_y), from: 6mm, to: 20mm))
over := solid(std.solids.Prism(face(across_z), from: 6mm, to: 20mm))
under := solid(std.solids.Prism(face(across_z), from: -20mm, to: -6mm))
east cut part
west cut part
north cut part
south cut part
over cut part
under cut part
