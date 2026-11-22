+++
title = "A block's top rim round a lobe and into a reverse arc"
tags = ["fillet", "round", "planar", "cylinder", "torus", "concave", "tangent-chain"]
origin = "a geomsolver refusal, bmander/geomsolver#962"

[volume]
value = 33002.55648871564
exact = "AH − (1 − π/4)r²(P − 2πd) (Pappus), d = r(10 − 3π)/(12 − 3π), A = 3125 + 125π/2, P = 170 + 25π, H = 10, r = 2; = 30570 + 2125π/3 + 21π²"
tex = 'AH - \left(1 - \tfrac{\pi}{4}\right)r^2\,(P - 2\pi d),\quad d = \frac{r(10 - 3\pi)}{12 - 3\pi},\ A = 3125 + \tfrac{125\pi}{2},\ P = 170 + 25\pi,\ H = 10,\ r = 2'

[area]
value = 8912.969535113412
exact = "A + (A − Pr + πr²) + P(H − r) + (πr/2)(P − 2π(r − 2r/π)) (Pappus); = 7270 + 457π + 21π²"
tex = 'A + (A - Pr + \pi r^2) + P(H - r) + \tfrac{\pi r}{2}\Bigl(P - 2\pi\bigl(r - \tfrac{2r}{\pi}\bigr)\Bigr)'
+++

A block from z = 0 to z = 10 whose plan, 80 along x and 40 along y with its corners rounded, has a
lobe hanging from its south side, and whose top rim is rounded to radius 2.

The plan is the region D bounded by one closed chain of thirteen pieces, each running on tangent
into the next. Anticlockwise from the south-west:

| piece | from | to | what it is |
|---|---|---|---|
| south side | (5, 0) | (30, 0) | straight |
| dip | (30, 0) | (35, −5) | a quarter of the circle of radius 5 about (30, −5), concave |
| lobe | (35, −5) | (55, −5) | the lower half of the circle of radius 10 about (45, −5), convex |
| rise | (55, −5) | (55, 0) | straight, 5 long |
| reverse arc | (55, 0) | (60, 5) | a quarter of the circle of radius 5 about (60, 0), concave |
| step | (60, 5) | (75, 5) | straight |
| corner | (75, 5) | (80, 10) | a quarter of the circle of radius 5 about (75, 10) |
| east side | (80, 10) | (80, 35) | straight |
| corner | (80, 35) | (75, 40) | a quarter of the circle of radius 5 about (75, 35) |
| north side | (75, 40) | (5, 40) | straight |
| corner | (5, 40) | (0, 35) | a quarter of the circle of radius 5 about (5, 35) |
| west side | (0, 35) | (0, 5) | straight |
| corner | (0, 5) | (5, 0) | a quarter of the circle of radius 5 about (5, 5) |

So the south side drops 5 into the lobe, which bulges 15 below it, and climbs out of the lobe by
the straight rise and the reverse arc onto the step, 5 above where it began. The block is the
prism over D from z = 0 to z = 10. Its sides are planes and cylinders, and it has no upright edge.

Round the rim of its top face to radius 2: all thirteen edges where the top meets the sides, one
tangent chain, as one round that never ends. Along the straight pieces the round is a quarter of a
cylinder of radius 2; round each arc it is a quarter of the section of a torus, whose tube runs at
radius R − 2 about a convex arc's axis and R + 2 about a concave one's. What makes it hard is
where the lobe runs out: the round turns about the lobe's axis, runs straight for 5 along the
rise, and turns the other way about the reverse arc's, two tangent seams of opposite turn one
short straight piece apart.

**As a set:** with D the plan above and $D \ominus B_2$ the points of it at least 2 from its
outline,

$$\bigl\{\,d\bigl((x,y),\ D \ominus B_2\bigr)^2 + \max(z - 8,\ 0)^2 \le 2^2,\ 0 \le z \le 10\,\bigr\}$$
