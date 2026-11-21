// A plate 80 by 50, from (−40, −25) to (40, 25) in the top plane, its corners rounded to radius
// 10, and the prism over it from z = 0 to z = 13; a round lug, the cylinder of radius 9 about the
// upright through (0, 25), the midpoint of the plate's north side, over the same depth, in union
// with it. The two concave upright edges where the lug's side meets the plate's north side are
// rounded to radius 8, which makes the outline one tangent chain; then, on the plate so blended,
// its bottom rim, every edge where a bottom face meets a side, rounded to radius 2 as one run.
unit mm
use std
use std.solids
use std.solids (union)
use std.rounds (round)

in std.top {
  a := point hint((-40, -25))
  b := point hint((40, -25))
  c := point hint((40, 25))
  d := point hint((-40, 25))
  outline := horizontal (south := line(a, b)) -> round(r: 10mm) vertical (east := line(b, c)) ->
             round(r: 10mm) horizontal (north := line(c, d)) -> round(r: 10mm)
             vertical (west := line(d, a)) -> round(r: 10mm) close
  fix((-40, -25)) a
  a distance(80mm) b
  b distance(50mm) c
  m := point
  m midpoint north
  lug_k := circle(center: m) hint(r: 9)
  radius(9mm) lug_k
}

plate := solid(std.solids.Prism(outline, from: 0mm, to: 13mm))
lug := solid(std.solids.Prism(face(lug_k), from: 0mm, to: 13mm))
body := solid(plate)
lug union body
// where the lug's side meets the plate's: the two upright edges, both concave
blend := body.lug.lug_k round(r: 8mm) body.plate

// the blended body; its bottom is three faces, the plate's, the lug's and the blend's, and its
// rim every edge where one of them meets an upright face
part := solid(body)
repeat f in part.body {
  repeat g in part.body {
    when f perpendicular std.z and g parallel std.z and f != part.body.plate.near
      and f != part.body.lug.near and f != part.body.blend.filled.near {
      f round(r: 2mm) g
    }
  }
}
