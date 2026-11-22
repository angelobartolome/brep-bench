+++
title = "A plate's bottom rim round a blended lug"
tags = ["fillet", "round", "planar", "cylinder", "torus", "concave", "union", "tangent-chain"]
origin = "a geomsolver refusal, bmander/geomsolver#996"

[volume]
value = 52471.10372229897
exact = "AH − (1 − π/4)r²(P − 2πd) (Pappus), d = r(10 − 3π)/(12 − 3π), A = 3720 + 100π + 17φ, P = 150 + 20π + 34φ, φ = arctan(15/8), H = 13, r = 2"
tex = 'AH - \left(1 - \tfrac{\pi}{4}\right)r^2\,(P - 2\pi d),\quad d = \frac{r(10 - 3\pi)}{12 - 3\pi},\ A = 3720 + 100\pi + 17\varphi,\ P = 150 + 20\pi + 34\varphi,\ \varphi = \arctan\tfrac{15}{8},\ H = 13,\ r = 2'

[area]
value = 11133.591048216753
exact = "A + (A − Pr + πr²) + P(H − r) + (πr/2)(P − 2π(r − 2r/π)) (Pappus)"
tex = 'A + (A - Pr + \pi r^2) + P(H - r) + \tfrac{\pi r}{2}\Bigl(P - 2\pi\bigl(r - \tfrac{2r}{\pi}\bigr)\Bigr)'
+++

A plate with a round lug on one long side, the lug blended into the side, and the plate's bottom
rim rounded all the way round, over the lug and its blends.

The plate runs from (−40, −25, 0) to (40, 25, 13), its four upright edges rounded to radius 10.
The lug is the solid cylinder of radius 9 about the upright line through (0, 25), the midpoint of
the plate's north side, from z = 0 to z = 13, joined to the plate: half of it stands out past the
north side, and its side meets that side in two concave upright edges, at (±9, 25, z).

Round those two edges to radius 8. Each blend is a piece of the cylinder of radius 8 about the
upright through (±15, 33): it touches the north side along x = ±15 and the lug's side where the
line from the lug's axis to its own crosses it, and sweeps an angle φ = arctan(15/8) ≈ 61.93°.
The lug's side keeps an arc of 2φ ≈ 123.86° between them. The plate's outline is now one tangent
chain of twelve pieces: five straight sides (the north side in two pieces, from x = ±30 to ±15),
four corners, two blends and the lug's arc.

Then round the rim of the bottom of the plate so blended to radius 2: every edge where its bottom
face meets a side, all twelve, as one round that never ends. Along the straight pieces it is a
quarter of a cylinder of radius 2; round each arc a quarter of the section of a torus, whose tube
runs at radius R − 2 about a convex arc's axis (8 about a corner's, 7 about the lug's) and R + 2
about a concave one's (10 about a blend's). The rim round runs over faces an earlier round made,
turning one way round each blend and the other round the lug.

**As a set:** with D the region the outline bounds (the plate's rounded rectangle, the lug's disc
and the two blends between them) and $D \ominus B_2$ the points of it at least 2 from the
outline,

$$\bigl\{\,d\bigl((x,y),\ D \ominus B_2\bigr)^2 + \max(2 - z,\ 0)^2 \le 2^2,\ 0 \le z \le 13\,\bigr\}$$
