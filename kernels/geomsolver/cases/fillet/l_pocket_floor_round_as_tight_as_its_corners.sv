// An L-shaped pocket's floor rounded as tight as its corners. A block 40 by 40 by 8, from
// (−20, −20, 0) to (20, 20, 8), and an L-shaped pocket cut down into its top: the L through
// (−12, −12), (12, −12), (12, 0), (0, 0), (0, 12) and (−12, 12) drawn in the top plane by its
// corners, the five convex corners rounds of radius 1 and the re-entrant one at the origin a round
// of radius 3, each tangent to the sides it joins, its prism running from the floor, z = 3, out
// past the top. The rim of the pocket's floor (`far`) against every face the pocket gave — six
// straight edges and six arcs, tangent all the way round — rounded to radius 1, filled.
unit mm
use std
use std.solids
use std.solids (cut)
use std.rounds (round)

in std.top {
  block_sq := step(u: 20mm, v: -20mm) (bs := line) -> step(v: 20mm) (be := line) ->
              step(u: -20mm) (bn := line) -> step(v: -20mm) (bw := line) -> close
  p0 := point hint((-12, -12))
  p1 := point hint((12, -12))
  p2 := point hint((12, 0))
  p3 := point hint((0, 0))
  p4 := point hint((0, 12))
  p5 := point hint((-12, 12))
  ell := horizontal (e0 := line(p0, p1)) -> round(r: 1mm) vertical (e1 := line(p1, p2)) ->
         round(r: 1mm) horizontal (e2 := line(p2, p3)) -> round(r: 3mm)
         vertical (e3 := line(p3, p4)) -> round(r: 1mm) horizontal (e4 := line(p4, p5)) ->
         round(r: 1mm) vertical (e5 := line(p5, p0)) -> round(r: 1mm) close
  fix((-12, -12)) p0
  fix((0, 0)) p3
  p0 distance(24mm) p1
  p3 distance(12mm) p4
}

part := solid(std.solids.Prism(block_sq, from: 0mm, to: 8mm))
pocket := solid(std.solids.Prism(ell, from: 3mm, to: 9mm))
pocket cut part
part.pocket.far round(r: 1mm) part.pocket
