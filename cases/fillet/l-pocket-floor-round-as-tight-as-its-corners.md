+++
title = "An L-shaped pocket's floor rounded as tight as its corners"
tags = ["fillet", "round", "concave", "pocket", "tangent-chain", "sphere", "torus", "degenerate"]
origin = "a geomsolver refusal, bmander/geomsolver#994"

[volume]
value = 10655.271690853984
exact = "XYZ − Sh + (1 − π/4)r²L + 5·πr³/12 + (π/2)(ρ + d)(1 − π/4)r² (Pappus: each convex corner's fill about its axis at ρ₁ − d, ρ₁ = r, the re-entrant corner's at ρ + d), d = r(10 − 3π)/(12 − 3π), S = 432 − 5(1 − π/4)ρ₁² + (1 − π/4)ρ², L = 96 − 10ρ₁ − 2ρ, X = Y = 40, Z = 8, h = 5, ρ₁ = r = 1, ρ = 3: 10700 − 38π/3 − π²/2"
tex = 'XYZ - Sh + \left(1 - \tfrac{\pi}{4}\right)r^2 L + \tfrac{5\pi r^3}{12} + \tfrac{\pi}{2}(\rho + d)\left(1 - \tfrac{\pi}{4}\right)r^2,\quad d = \frac{r(10 - 3\pi)}{12 - 3\pi}:\quad 10700 - \tfrac{38\pi}{3} - \tfrac{\pi^2}{2}'

[area]
value = 4902.657200348528
exact = "2XY − S + 4XZ + (L + (π/2)(5ρ₁ + ρ))(h − r) + F + (πr/2)L + 5πr²/2 + (π/2)(ρ + r − 2r/π)(πr/2) (Pappus), F = 340 + (1 − π/4)(ρ + r)² the floor: 4720 + 55π + π²"
tex = '2XY - S + 4XZ + \bigl(L + \tfrac{\pi}{2}(5\rho_1 + \rho)\bigr)(h - r) + F + \tfrac{\pi r}{2}L + \tfrac{5\pi r^2}{2} + \tfrac{\pi}{2}\left(\rho + r - \tfrac{2r}{\pi}\right)\tfrac{\pi r}{2}:\quad 4720 + 55\pi + \pi^2'
+++

A block 40 by 40 by 8, from (−20, −20, 0) to (20, 20, 8), with an L-shaped pocket cut down into its
top. The pocket's outline is the polygon through (−12, −12), (12, −12), (12, 0), (0, 0), (0, 12)
and (−12, 12), three squares of side 12, with its corners rounded: the five convex corners to
radius 1, and the re-entrant corner at the origin to radius 3, an arc about (3, 3). It is cut 5
deep, so that its floor is the plane z = 3. Its walls are six planes, five quarter cylinders of
radius 1 with the cavity inside them, and one quarter cylinder of radius 3 about the upright line
x = y = 3 that stands out into the cavity, the block's material inside it.

Round the rim of the pocket's floor to radius 1, filling. The rim is twelve edges, six straight
and six arcs, each running on tangent into the next, and the round runs round all twelve unbroken:
along the straight edges a concave quarter cylinder of radius 1. At the five convex corners the
round's radius is the corners' own, so round each the round is an eighth of the ball of radius 1
about the point 1 in from the corner's two walls at z = 4, such as (11, −11, 4). At the re-entrant
corner the round wraps the outside of the wall of radius 3: a quarter of the tube of radius 1
about the circle of radius 4 about (3, 3) in the plane z = 4, the part of it facing that wall and
the floor. The floor that is left is the outline moved in by 1: the L through (−11, −11),
(11, −11), (11, −1), (−1, −1), (−1, 11) and (−11, 11), its convex corners sharp and its re-entrant
corner rounded to radius 4 about (3, 3). The block's top rim and the pocket's top rim are left
sharp.

**As a set:**

$$[-20, 20]^2 \times [0, 8]\ \setminus\ \bigl\{\, d\bigl((x, y),\ T\bigr)^2 + \max(4 - z,\ 0)^2 \le 1 \,\bigr\}$$

where T, the floor that is left, is $\mathrm{poly}((-11, -11), (11, -11), (11, -1), (-1, -1), (-1, 11), (-11, 11)) \cup \bigl([-1, 3]^2 \setminus B_4((3, 3))\bigr)$.
