+++
title = "A pocket's floor rounded as tight as its corners"
tags = ["fillet", "round", "concave", "pocket", "tangent-chain", "sphere", "degenerate"]
origin = "a geomsolver regression, bmander/geomsolver#962"

[volume]
value = 62443.87038006424
exact = "XYZ − Sh + (1 − π/4)r²L + 4·πr³/12 (Pappus about each corner's axis, the fill's centroid at ρ − d = 2r/(12 − 3π), d = r(10 − 3π)/(12 − 3π)), S = ab − (4 − π)ρ², L = 2(a − 2ρ) + 2(b − 2ρ), X = 80, Y = 60, Z = 16, a = 50, b = 30, h = 10, ρ = r = 4: 64488 − 1952π/3"
tex = 'XYZ - Sh + \left(1 - \tfrac{\pi}{4}\right)r^2 L + \tfrac{\pi r^3}{3},\quad S = ab - (4 - \pi)\rho^2,\ L = 2(a - 2\rho) + 2(b - 2\rho):\quad 64488 - \tfrac{1952\pi}{3}'

[area]
value = 15341.309649148734
exact = "2XY − S + 2(X + Y)Z + (L + 2πρ)(h − r) + (a − 2r)(b − 2r) + (πr/2)L + 2πr² (four eighths of a ball): 14336 + 320π"
tex = '2XY - S + 2(X + Y)Z + (L + 2\pi\rho)(h - r) + (a - 2r)(b - 2r) + \tfrac{\pi r}{2}L + 2\pi r^2:\quad 14336 + 320\pi'
+++

A block 80 by 60 by 16, from (−40, −30, 0) to (40, 30, 16), with a pocket cut down into its top:
the rectangle from (−25, −15) to (25, 15) with its four corners rounded to radius 4, cut 10 deep,
so that its floor is the plane z = 6 and its walls are four planes and four quarter cylinders of
radius 4, about the upright lines x = ±21, y = ±11.

Round the rim of the pocket's floor to radius 4, filling. The rim is eight edges, four straight and
four quarter circles, each running on tangent into the next, and the round runs round all eight
unbroken: along the straight edges a concave quarter cylinder of radius 4, and round each corner
what would be a quarter of a torus's tube about the corner's axis. The round's radius is the
corners' own, so that torus degenerates: its tube's centre circle has radius 0, and round each
corner the round is an eighth of the ball of radius 4 about the point (±21, ±11, 10), tangent to
the straight rounds either side. The floor that is left is the rectangle from (−21, −11) to
(21, 11), its corners sharp, and the walls stand from z = 10 to the top. The block's top rim and
the pocket's top rim are left sharp.

**As a set:**

$$[-40, 40] \times [-30, 30] \times [0, 16]\ \setminus\ \bigl\{\, d\bigl((x, y),\ T\bigr)^2 + \max(10 - z,\ 0)^2 \le 16 \,\bigr\},\quad T = [-21, 21] \times [-11, 11]$$
