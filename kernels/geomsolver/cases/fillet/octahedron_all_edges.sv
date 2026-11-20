// The octahedron |x| + |y| + |z| ≤ 10, as the join of a square and its axis: the points on a
// segment from some point of the square, drawn in the top plane, to some point of the segment
// between its top and bottom corners. Every edge among its faces rounded to radius 2; four rounds
// meet at each corner. Its six corners are on the axes, each opposite one the half turn of the
// other: the middle square's corners, drawn in the top plane, are where it meets the front and side
// planes, the origin midway between opposite ones, and the top corner is on z with the bottom its
// half turn about x. The edges between the corners on +x, +y and +z read one unknown length the
// solve finds (10√2), which makes it regular, and it stands 20 tall, the top 20 above the bottom;
// the corners on +x and +y lie toward +x and +y of their opposites, which says which root is meant,
// so no corner is seeded or held.
unit mm
use std
use std.sets
use std.rounds (round)

in std.top {
  e := point
  n := point
  w := point
  s := point
  equator := line(e, n) -> line(n, w) -> line(w, s) -> line(s, e) -> close
  e coincident std.front
  n coincident std.side
  std.top.origin midpoint line(w, e)
  std.top.origin midpoint line(s, n)
  w distance(>= 0mm, along: std.x) e
  s distance(>= 0mm, along: std.y) n
  square := face(equator)
}
top := point
bottom := point
top coincident std.z
top symmetry(std.x) bottom
bottom distance(20mm, along: std.z) top
param edge: Length hint(14mm)
e distance(edge) n
n distance(edge) top
top distance(edge) e

part := solid({ p | some q in square, r in std.sets.Segment(bottom, top): p in std.sets.Segment(q, r) })
round(r: 2mm) part
