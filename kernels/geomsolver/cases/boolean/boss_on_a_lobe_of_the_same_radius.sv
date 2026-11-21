// A dog-bone plate and a boss on one of its lobes. The outline in the top plane: two lobes of
// radius 10 about (±30, 0), each drawn as its outer arc between the points where the waist's sides
// y = ±4 meet its circle, and each of those four corners rounded concave to radius 6 — a blend
// tangent to the side and to the lobe. The plate is its prism from z = 0 to z = 10. The boss, the
// cylinder of radius 10 about (30, 0) from z = 0 to z = 30, is joined to it: on the lobe's outer
// arc the two sides are one cylinder up to the plate's top.
unit mm
use std
use std.solids
use std.solids (union)

in std.top {
  l := point
  r := point
  fix((-30, 0)) l
  fix((30, 0)) r
  a := point hint((-20.83, -4))
  b := point hint((20.83, -4))
  c := point hint((20.83, 4))
  d := point hint((-20.83, 4))
  outline := horizontal (south := line(a, b)) -> round(r: 6mm)
             (east := arc(center: r, start: b, end: c)) -> round(r: 6mm)
             horizontal (north := line(c, d)) -> round(r: 6mm)
             (west := arc(center: l, start: d, end: a)) -> round(r: 6mm) close
  radius(10mm) east
  radius(10mm) west
  l distance(-4mm, along: v) a
  l distance(4mm, along: v) d

  lobe := circle(center: r) hint(r: 10)
  radius(10mm) lobe
}

part := solid(std.solids.Prism(outline, from: 0mm, to: 10mm))
boss := solid(std.solids.Prism(face(lobe), from: 0mm, to: 30mm))
boss union part
