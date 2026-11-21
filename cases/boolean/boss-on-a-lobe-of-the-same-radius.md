+++
title = "A boss on a lobe of the same radius"
tags = ["boolean", "union", "analytic", "cylinder", "coincident", "tangent"]
origin = "a geomsolver failure, bmander/geomsolver#961"

[volume]
value = 15870.828736779318
exact = "At + πR²(H − t), A = 4wb + 4ρ(w + ρ)cos β − ρ²(π − 2β) − 2ρ²sin β cos β + 2πR² − 2R²β + 2R²sin β cos β the outline, b = m − (R + ρ)cos β, sin β = (w + ρ)/(R + ρ); = 4800 + 80√39 + 3640π − 1280β, β = arcsin(5/8), m = 30, R = 10, w = 4, ρ = 6, t = 10, H = 30"
tex = 'At + \pi R^2(H - t),\quad A = 4wb + 4\rho(w + \rho)\cos\beta - \rho^2(\pi - 2\beta) - 2\rho^2\sin\beta\cos\beta + 2\pi R^2 - 2R^2\beta + 2R^2\sin\beta\cos\beta,\ b = m - (R + \rho)\cos\beta,\ \sin\beta = \tfrac{w + \rho}{R + \rho}'

[area]
value = 5076.1099062709845
exact = "2A + Pt + 2πR(H − t), P = 4(b + ρ(π/2 − β) + R(π − β)) the outline's perimeter, A, b and β as above; = 2160 − 64√39 + 1248π − 896β"
tex = '2A + Pt + 2\pi R(H - t),\quad P = 4\left(b + \rho\left(\tfrac{\pi}{2} - \beta\right) + R(\pi - \beta)\right)'
+++

A dog-bone plate with a round boss standing on one of its lobes, the boss exactly as wide as the
lobe and on the same axis.

The plate's outline, in the plane z = 0, is two round lobes joined by a waist. The lobes are the
circles of radius 10 about (−30, 0) and (30, 0). The waist's sides are the lines y = 4 and y = −4.
Each side runs into each lobe through a concave blend of radius 6, tangent to the side and to the
lobe: its centre is 6 off the side, at y = ±10, and 16 from the lobe's centre, so at
x = ±(30 − 2√39) ≈ ±17.51. So the outline is, in turn, a straight side, a blend, the outer arc of
a lobe, a blend, and so on round, eight pieces each running on tangent into the next. The plate
is that outline extruded from z = 0 to z = 10.

The boss is the solid cylinder of radius 10 about the vertical line through (30, 0), from z = 0
to z = 30. The solid is the union of the plate and the boss. Where the lobe's outer arc runs, the
boss's side and the plate's side are the same cylinder from z = 0 to the plate's top: the solid's
side there is one smooth surface from z = 0 to z = 30, and the plate's top meets the boss in the
lobe's inner arc, between the two blends.

**As a set:**

$$\Bigl(\bigl((S_0 \oplus B_6) \ominus B_6\bigr) \times [0, 10]\Bigr) \cup
\{\,(x-30)^2 + y^2 \le 10^2,\ 0 \le z \le 30\,\}$$

where $S_0 = B_{10}((-30, 0)) \cup B_{10}((30, 0)) \cup [-30, 30]\times[-4, 4]$ is the outline
with its blends left sharp and $B_6$ the disk of radius 6, all in the plane: closing $S_0$ by
$B_6$ rounds its four concave corners to radius 6.
