// A pocket's floor rounded as tight as its corners. A block 80 by 60 by 16, from (−40, −30, 0) to
// (40, 30, 16), and a pocket cut down into its top: the rectangle from (−25, −15) to (25, 15)
// drawn in the top plane by its corners, each corner a round of radius 4 tangent to the sides it
// joins, its prism running from the floor, z = 6, out past the top. The rim of the pocket's floor
// (`far`, the pocket's cap at the lower ordinate) against every face the pocket gave — four
// straight edges and the four arcs the corners left, tangent all the way round — rounded to
// radius 4, filled: the corners' own radius, so round each corner an eighth of a ball.
unit mm
use std
use std.solids
use std.solids (cut)
use std.rounds (round)

in std.top {
  block_sq := step(u: 40mm, v: -30mm) (bs := line) -> step(v: 30mm) (be := line) ->
              step(u: -40mm) (bn := line) -> step(v: -30mm) (bw := line) -> close
  a := point hint((-25, -15))
  b := point hint((25, -15))
  c := point hint((25, 15))
  d := point hint((-25, 15))
  outline := horizontal (south := line(a, b)) -> round(r: 4mm) vertical (east := line(b, c)) ->
             round(r: 4mm) horizontal (north := line(c, d)) -> round(r: 4mm)
             vertical (west := line(d, a)) -> round(r: 4mm) close
  fix((-25, -15)) a
  a distance(50mm) b
  b distance(30mm) c
}

part := solid(std.solids.Prism(block_sq, from: 0mm, to: 16mm))
pocket := solid(std.solids.Prism(outline, from: 6mm, to: 17mm))
pocket cut part
part.pocket.far round(r: 4mm) part.pocket
