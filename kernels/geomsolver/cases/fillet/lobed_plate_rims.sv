// A plate whose outline is six convex lobes and six concave arcs, alternating, each running on
// tangent into the next: the lobes of radius 8 about points 40 from the centre, at every sixty
// degrees from +x; the concave arcs of radius 30, each touching the two lobes beside it from
// outside, its centre on the ray halfway between theirs. One lobe and the hollow after it, turned
// six times about the centre, in the top plane (x, y); the plate is the prism over it from z = 0 to
// z = 12. Both rims, the top's (`near`) and the bottom's (`far`), each a tangent chain of twelve
// arcs all the way round, rounded to radius 1.
unit mm
use std
use std.solids
use std.rounds (round)

in std.top {
  o := point
  fix((0, 0)) o
  ring 6 about o {
    c := point hint((40, 0))
    m := point hint((57.98, 33.48))
    p := point hint((43.79, -7.05))
    q := point hint((43.79, 7.05))
    o distance(40mm) c
    // the lobe, and the hollow after it, which runs on into the next copy's lobe
    (lobe := arc(center: c, start: p, end: q)) ->
      (hollow := arc(center: m, start: q, end: next.p, sense: cw))
    radius(8mm) lobe
    radius(30mm) hollow
    // each joint tangent: the point where two arcs meet lies on the line through their centres
    construction into := line(c, m)
    q coincident into
    construction out := line(m, next.c)
    next.p coincident out
  }
  o level(v) c[0]
  outline := face(lobe[0], hollow[0], lobe[1], hollow[1], lobe[2], hollow[2], lobe[3], hollow[3],
    lobe[4], hollow[4], lobe[5], hollow[5])
}

part := solid(std.solids.Prism(outline, from: 0mm, to: 12mm))
part.near round(r: 1mm) part
part.far round(r: 1mm) part
