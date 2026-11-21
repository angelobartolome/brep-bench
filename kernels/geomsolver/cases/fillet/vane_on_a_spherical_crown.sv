// A vane on a spherical crown. The disc: its half section in the front plane (x, z) turned whole
// about z — flat beneath at z = 0 out to the rim at x = 100, the rim 3 high, the top the arc of
// radius 800 about a centre on the axis from the rim's top to the axis. The vane: the obround in
// the top plane, within 2 of the segment from (22, 0) to (88, 0) (two straight sides and the half
// circles about its ends, one tangent chain), from z = 1 up to z = 31, in union with the disc.
// The loop where the vane's sides meet the crown is rounded to radius 2.
unit mm
use std
use std.solids
use std.solids (union)
use std.rounds (round)

in std.front {
  o := point
  fix((0, 0)) o
  q := point
  fix((100, 0)) q
  t := point
  fix((100, 3)) t
  c := point hint((0, -790.7))
  fix(x == 0mm) c
  peak := point hint((0, 9.27))
  fix(x == 0mm) peak
  bottom := line(o, q)
  rim := line(q, t)
  crown := arc(center: c, start: t, end: peak)
  radius(800mm) crown
  spine := line(peak, o)
  section := face(bottom, rim, crown, spine)
}
disc := solid(std.solids.Revolution(section, about: std.z))

in std.top {
  k0 := point
  fix((22, 0)) k0
  k1 := point
  fix((88, 0)) k1
  a := point
  fix((22, -2)) a
  b := point
  fix((88, -2)) b
  d := point hint((88, 2))
  fix(x == 88mm) d
  e := point hint((22, 2))
  fix(x == 22mm) e
  south := line(a, b)
  east := arc(center: k1, start: b, end: d)
  north := line(d, e)
  west := arc(center: k0, start: e, end: a)
  foot := face(south, east, north, west)
}
vane := solid(std.solids.Prism(foot, from: 1mm, to: 31mm))

part := solid(disc)
vane union part
part.vane round(r: 2mm) part.disc
