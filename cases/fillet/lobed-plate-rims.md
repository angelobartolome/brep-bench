+++
title = "A lobed plate with both rims rounded"
tags = ["fillet", "round", "planar", "cylinder", "torus", "concave", "tangent-chain"]
origin = "a geomsolver refusal, bmander/geomsolver#962"

[volume]
value = 65332.05185747239
exact = "AH − 2(1 − π/4)r²(P − 2πd) (Pappus), d = r(10 − 3π)/(12 − 3π), A = 120D − 5016α + 900π, P = 456α − 60π, D = 20√3 + 6√29, α = arccos((3√87 − 10)/38), H = 12, r = 1"
tex = 'AH - 2\left(1 - \tfrac{\pi}{4}\right)r^2\,(P - 2\pi d),\quad d = \frac{r(10 - 3\pi)}{12 - 3\pi},\ A = 120D - 5016\alpha + 900\pi,\ P = 456\alpha - 60\pi,\ D = 20\sqrt{3} + 6\sqrt{29},\ \alpha = \arccos\frac{3\sqrt{87} - 10}{38},\ H = 12,\ r = 1'

[area]
value = 14285.36218203638
exact = "2(A − Pr + πr²) + P(H − 2r) + πr(P − 2π(r − 2r/π)) (Pappus)"
tex = '2(A - Pr + \pi r^2) + P(H - 2r) + \pi r\Bigl(P - 2\pi\bigl(r - \tfrac{2r}{\pi}\bigr)\Bigr)'
+++

A plate 12 thick, from z = 0 to z = 12, whose outline is six round lobes and six hollows between
them, every one running on tangent into the next, with both its rims rounded to radius 1.

The lobes are arcs of the six circles of radius 8 about the points 40 from the z axis at 0°, 60°,
120°, 180°, 240° and 300° from +x. Each hollow is an arc of the circle of radius 30 that touches
the two lobes' circles beside it from outside: its centre lies on the ray halfway between theirs
(at 30°, 90°, …, 330°), 38 from each of their centres, so at D = 20√3 + 6√29 ≈ 66.952 from the
axis (the farther of the two such points). The outline runs round each lobe's outer side from the
point where it touches the hollow before it to the point where it touches the hollow after it,
an arc of 2α ≈ 123.51° with α = arccos((3√87 − 10)/38), and round each hollow's inner side, an
arc of 2α − 60° ≈ 63.51°: twelve arcs in one tangent loop, convex and concave by turns. The plate
is the prism over the region it bounds.

Round both rims to radius 1: the twelve edges where the top meets the side, and the twelve where
the bottom does. Each rim is one tangent chain, rounded as one run that never ends; round each lobe
the round is a quarter of the section of a torus whose tube runs at radius 7 about the lobe's
axis, round each hollow one whose tube runs at radius 31 about the hollow's. Every joint of the
outline is a seam where the round turns from one way to the other, twelve times round each rim.

**As a set:** with L the region the outline bounds and $L \ominus B_1$ the points of it at least
1 from the outline,

$$\bigl\{\,d\bigl((x,y),\ L \ominus B_1\bigr)^2 + \max(|z - 6| - 5,\ 0)^2 \le 1\,\bigr\}$$
