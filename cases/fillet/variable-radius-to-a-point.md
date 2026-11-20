+++
title = "A cube edge rounded with a radius shrinking to nothing"
tags = ["fillet", "variable-radius", "vanishing-radius", "cone", "singular"]

[volume]
value = 997.1386421786326
exact = "1000 − (1 − π/4)∫₀¹⁰(2 − z/5)²dz = 1000 − (40/3)(1 − π/4)"
tex = '1000 - \left(1 - \tfrac{\pi}{4}\right)\int_0^{10}\left(2 - \tfrac{z}{5}\right)^2 dz = 1000 - \tfrac{40}{3}\left(1 - \tfrac{\pi}{4}\right)'

[area]
value = 594.8778376869467
exact = "600 − 2∫₀¹⁰ r dz − (1 − π/4)·2² + ∫₀¹⁰ r dz·I, r = 2 − z/5, r′ = −1/5: 580 − 4(1 − π/4) + 10·I, I = ∫₀^{π/2}√(1 + (1 − sin θ − cos θ)²/25)dθ = 1.573624503335686 (the round's area separates as ∫ r dz times I, since r′ is constant)"
tex = '\begin{array}{l} 580 - 4\left(1 - \tfrac{\pi}{4}\right) + 10I \\ I = \displaystyle\int_0^{\pi/2}\sqrt{1 + \tfrac{(1 - \sin\theta - \cos\theta)^2}{25}}\,d\theta = 1.573624503335686 \end{array}'
+++

A cube of side 10 between the origin and (10, 10, 10). Round the edge where its faces x = 10 and
y = 10 meet, the line from (10, 10, 0) to (10, 10, 10), with a radius that shrinks linearly along
it to nothing: r = 2 − z/5, so 2 at the bottom face z = 0 and 0 at the top face z = 10.

The round's sections square to the edge are circular: at each height z the round's section by the
horizontal plane is the quarter circle of radius r(z) centered at (10 − r, 10 − r, z), tangent to
the face x = 10 at (10, 10 − r, z) and to the face y = 10 at (10 − r, 10, z). Since the radius
falls to nought at the top, the round is part of an oblique circular cone whose apex is the cube's
corner (10, 10, 10): its lines run from that corner down to the quarter circle of radius 2 it cuts
from the bottom face. It meets each of the faces x = 10 and y = 10 along a straight line, from
(10, 8, 0) or (8, 10, 0) up to the corner, so each of those faces loses a triangle of legs 2 and
10. The top face is the whole square: its corner (10, 10, 10) is the cone's apex, where the round
comes to a point.

It is `fillet/variable-radius-edge` with a radius that vanishes at one end, as a variable round
set to nought at a vertex does.

**As a set:**

$$[0, 10]^3 \setminus \bigl\{\, x > 10 - r,\ y > 10 - r,\ (x - 10 + r)^2 + (y - 10 + r)^2 > r^2 \,\bigr\},\quad r = 2 - z/5$$
