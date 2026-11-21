+++
title = "An eye across a bar, its roots rounded flush with the bar's sides"
tags = ["fillet", "round", "concave", "cylinder", "planar", "union", "flush"]
origin = "a geomsolver failure, bmander/geomsolver#962"

[volume]
value = 18307.74057923835
exact = "wA, A = 4ℓh + 2c(r − h) + 2(R² − r²)φ the section, c = √((R + r)² − (h + r)²) = 15, φ = atan(c/(h + r)) = atan(15/8), ℓ = 40, h = 5, w = 16, R = 14, r = 3: 11840 + 5984 atan(15/8)"
tex = 'wA,\quad A = 4\ell h + 2c(r - h) + 2(R^2 - r^2)\varphi,\quad c = \sqrt{(R + r)^2 - (h + r)^2},\ \varphi = \arctan\frac{c}{h + r}:\quad 11840 + 5984\arctan\tfrac{15}{8}'

[area]
value = 5384.420404993585
exact = "2A + wP, P = 4h + 4(ℓ − c) + 4(R + r)φ the section's perimeter: 3400 + 1836 atan(15/8)"
tex = '2A + wP,\quad P = 4h + 4(\ell - c) + 4(R + r)\varphi:\quad 3400 + 1836\arctan\tfrac{15}{8}'
+++

A bar with an eye across its middle. The bar is the box from (−40, −8, −5) to (40, 8, 5). The eye
is the cylinder of radius 14 about the y axis, from y = −8 to y = 8: its ends lie flush with the
bar's sides, in the planes y = ±8, and its side stands 9 proud of the bar's top and bottom. Join
them.

The eye's side meets the bar's top and bottom in four straight concave edges along y, at
x = ±√171, z = ±5. Round those four to radius 3, filling. Each round is part of a cylinder of
radius 3 about a line along y through (±15, ±8), tangent to the bar's top or bottom along x = ±15
and to the eye's side where the plane through the y axis and the round's axis meets it. Each runs
the bar's whole width, from y = −8 to y = 8, and ends flush with the bar's sides, in the same
planes as the eye's ends: the solid is its section in the xz plane, the rectangle and the disk with
the four rounds, extruded 16 along y. In the closed forms φ is the angle at the eye's axis between
z and the line to a round's axis, so that the arc of the eye left standing above the bar spans 2φ.

**As a set:**

$$\Bigl(\bigl([-40, 40] \times [-5, 5]\bigr) \cup B_{14} \cup Q\Bigr) \times [-8, 8]$$

read in the xz plane and extruded along y, where Q is the four rounds' fill,
$Q = \{\, 5 \le |z| \le 8|x|/15,\ |x| \le 15,\ (|x| - 15)^2 + (|z| - 8)^2 \ge 9 \,\}$.
