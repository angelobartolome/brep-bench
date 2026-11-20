// A hyperboloid of one sheet: in the front plane (x, z), the branch x > 0 of the hyperbola
// x²/16 - z²/36 = 1 traced from z = -6 to z = 6, its waist 4 out from the axis; with the lines in
// to the z axis at each end and the axis between them, turned a whole turn about the z axis.
unit mm
use std
use std.solids

// A hyperbola as its foci make it: every point `2a` farther from `far` than from `near`. `q` goes
// round the circle of radius `2a` about `far`, at bearing `u` from the line of the foci, and `p`
// is the point of the ray through it as far from `q` as from `near`.
component Hyperbola(near: point, far: point, a: Length, u: Angle) {
  q := point
  p := point
  foci := line(far, near)
  ray := line(far, q)
  far distance(2 * a) q
  foci angle(u) ray
  p coincident ray
  line(p, q) equal line(p, near)
}

in std.front {
  o := point
  fix((0, 0)) o
  // the box of the asymptotes, the waist 4 out and 6 up: a focus is as far out as its corner
  corner := point
  fix((4, 6)) corner
  near := point hint((7.2, 0))
  far := point hint((-7.2, 0))
  horizontal (span := line(far, near))
  o midpoint span
  line(o, near) equal line(o, corner)

  low := point hint((5.66, -6))
  high := point hint((5.66, 6))
  low coincident k hint(t: -25)
  high coincident k hint(t: 25)
  fix(y == -6mm) low
  fix(y == 6mm) high
  foot := point hint((0, -6))
  head := point hint((0, 6))
  horizontal (top := line(high, head))
  construction centerline spindle := line(head, foot)
  vertical spindle
  horizontal (bottom := line(foot, low))
  fix(x == 0mm) foot
}
k := trace(Hyperbola(near, far, a: 4mm).p over u in (-30deg, 30deg))

part := solid(std.solids.Revolution(face(k from low to high, top, spindle, bottom), about: std.z))
