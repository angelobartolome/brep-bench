// A block 80 by 40 in plan, its corners rounded to radius 5, with a lobe on its south side: the
// south side runs east along y = 0 to x = 30, bends down through a quarter of the circle of radius
// 5 about (30, −5) (concave) into the lobe, half the circle of radius 10 about (45, −5) (convex),
// runs straight north 5 from (55, −5) to (55, 0), bends through a quarter of the circle of radius 5
// about (60, 0) (concave, the reverse arc) to (60, 5), and runs on east along y = 5. The outline is
// one tangent chain; the block is the prism over it from z = 0 to z = 10. The rim of its top
// (`near`), every edge where it meets another face, tangent all the way round, rounded to
// radius 2: the lobe and the reverse arc are two tangent seams, convex then concave, one straight
// piece apart.
unit mm
use std
use std.solids
use std.rounds (round)

in std.top {
  sw := point hint((0, 0))
  p1 := point hint((30, 0))
  p2 := point hint((35, -5))
  p3 := point hint((55, -5))
  p4 := point hint((55, 0))
  p5 := point hint((60, 5))
  se := point hint((80, 5))
  ne := point hint((80, 40))
  nw := point hint((0, 40))
  o1 := point hint((30, -5))
  o2 := point hint((45, -5))
  outline := horizontal (south := line(sw, p1)) -> tangent
             (dip := arc(center: o1, start: p1, end: p2, sense: cw)) ->
             (lobe := arc(center: o2, start: p2, end: p3)) -> tangent
             vertical (rise := line(p3, p4)) -> tangent
             (reverse := arc(center: hint((60, 0)), start: p4, end: p5, sense: cw)) -> tangent
             horizontal (step := line(p5, se)) -> round(r: 5mm)
             vertical (east := line(se, ne)) -> round(r: 5mm)
             horizontal (north := line(ne, nw)) -> round(r: 5mm)
             vertical (west := line(nw, sw)) -> round(r: 5mm) close
  fix((0, 0)) sw
  sw distance(30mm) p1
  radius(5mm) dip
  radius(10mm) lobe
  construction centres := line(o1, o2)
  p2 coincident centres                 // the dip runs on tangent into the lobe
  p3 distance(5mm) p4
  radius(5mm) reverse
  sw distance(80mm, along: u) se
  sw distance(5mm, along: v) se
  sw distance(40mm, along: v) nw
}

part := solid(std.solids.Prism(outline, from: 0mm, to: 10mm))
part.near round(r: 2mm) part
