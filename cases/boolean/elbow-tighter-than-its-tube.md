+++
title = "An elbow tighter than its tube"
tags = ["boolean", "union", "analytic", "torus", "degenerate", "tangent"]
origin = "a geomsolver refusal, bmander/geomsolver#996"

[volume]
value = 136053.4394407064
exact = "2πr²L + (π/2)(2h³/3 + R(πr² − r²β + Rh)) − (4h³/3 − 2R(r²β − Rh)), h = √(r² − R²) = 2√19, β = arccos(R/r), r = 20, R = 18, L = 40"
tex = '2\pi r^2 L + \tfrac{\pi}{2}\Bigl(\tfrac{2}{3}h^3 + R\bigl(\pi r^2 - r^2\beta + Rh\bigr)\Bigr) - \Bigl(\tfrac{4}{3}h^3 - 2R\bigl(r^2\beta - Rh\bigr)\Bigr),\quad h = \sqrt{r^2 - R^2},\ \beta = \arccos\tfrac{R}{r}'

[area]
value = 16109.13906605814
exact = "2πr² + 4πrL − 4r(h − Rβ) + πr(R(π − β) + h)"
tex = '2\pi r^2 + 4\pi r L - 4r\,(h - R\beta) + \pi r\bigl(R(\pi - \beta) + h\bigr)'
+++

The outside of a pipe elbow whose bend is tighter than the pipe is thick: a round tube of
radius r = 20 turned a quarter turn about a bend axis only R = 18 from its centre-line, with a
straight leg of length 40 at each end.

The bend axis is the y axis. The turn is every point that the disk D, of radius 20 about
(0, 0, 18) in the plane x = 0, passes through as it turns 90° about the y axis, right-handed, so
that +z goes to +x: its centre runs round the circle of radius 18 from (0, 0, 18) to (18, 0, 0),
where the disk lies in the plane z = 0. The disk reaches 2 past the axis, so the turned region
crosses it.

The legs run straight back from the turn's two ends, each a solid cylinder of radius 20 and
length 40: the first about the line y = 0, z = 18, from x = −40 to x = 0; the second about the
line x = 18, y = 0, from z = −40 to z = 0. The solid is the union of the turn and the two legs.

The legs overlap near the inner corner, in the small region between x = −2 and 0 and between
z = −2 and 0. The turn's outer surface is a piece of a spindle torus, its tube wider than its
centre circle, whose two apexes lie on the bend axis at (0, ±2√19, 0); both legs' sides pass
through those two points as well.

**As a set:**

$$\{\,y^2+(z-18)^2\le 20^2,\ -40\le x\le 0\,\}\ \cup\ \{\,(x-18)^2+y^2\le 20^2,\ -40\le z\le 0\,\}\ \cup
\bigcup_{0\le\theta\le 90^\circ} R_y(\theta)\,\{\,x=0,\ y^2+(z-18)^2\le 20^2\,\}$$
