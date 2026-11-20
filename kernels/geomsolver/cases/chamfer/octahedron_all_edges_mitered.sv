// The octahedron |x| + |y| + |z| <= 10: its middle square, drawn in the top plane, as a prism from
// z = -10 to z = 10, and from it cut everything beyond each of its eight faces — the plane through
// three corners, its normal out — by a disc drawn on that plane about a corner and swept out along
// the normal past the prism. Then every edge among its faces chamfered √6 on each face, measured
// across the face square to the edge: the library's `chamfer(d: …)` takes its leg as the distance
// from the other face, so `d` is that leg times the sine of the dihedral angle between two faces,
// cos θ = -1/3. Each flat is its own, so the four at a corner run on until they meet in a point,
// the corner mitred. Its six corners are on the axes, each opposite one the half turn of the other:
// the middle square's corners, drawn in the top plane, are where it meets the front and side
// planes, the origin midway between opposite ones, and the top corner is on z with the bottom its
// half turn about x. The edges between the corners on +x, +y and +z read one unknown length the
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

part := solid(std.solids.Prism(square, from: -10mm, to: 10mm))

ppp := Beyond(px, py, pz)
npp := Beyond(nx, pz, py)
pnp := Beyond(px, pz, ny)
nnp := Beyond(nx, ny, pz)
ppn := Beyond(px, nz, py)
npn := Beyond(nx, py, nz)
pnn := Beyond(px, ny, nz)
nnn := Beyond(nx, nz, ny)
ppp.cutter cut part
npp.cutter cut part
pnp.cutter cut part
nnp.cutter cut part
ppn.cutter cut part
npn.cutter cut part
pnn.cutter cut part
nnn.cutter cut part

chamfer(d: sqrt(6) * sin(acos(-1 / 3)) * 1mm) part
