+++
title = "A cylinder's rim blended by a parabolic conic"
tags = ["fillet", "conic", "cylinder", "curved-edge", "revolution"]

[volume]
value = 766.129728455431
exact = "250π − 2π(5A − M), A = d²/6 = 2/3, the region between the corner and the parabola, M = 4/15, its moment about the side: 250π − 92π/15"
tex = '250\pi - 2\pi\left(5A - M\right),\quad A = \tfrac{d^2}{6} = \tfrac{2}{3},\ M = \tfrac{4}{15}:\quad 250\pi - \tfrac{92\pi}{15}'

[area]
value = 446.19935825865923
exact = "25π + 9π + 80π + 2π∫₀¹ρ(t)|B′(t)|dt, B(t) = (ρ, z) = (5 − 2(1 − t)², 10 − 2t²), the blend a surface of revolution: 114π + 88.05779574942282"
tex = '114\pi + 2\pi\int_0^1 \rho(t)\,\lvert B^\prime(t)\rvert\,dt = 114\pi + 88.05779574942282,\quad B(t) = \bigl(5 - 2(1 - t)^2,\ 10 - 2t^2\bigr)'
+++

A cylinder of radius 5 on the z axis, from z = 0 to z = 10. Blend the circle where its side meets
its top with a conic blend of rho 0.5, a parabola, tangent to the top and to the side each 2 from
the edge.

In each half plane through the z axis the blend's section is the parabola tangent to the top 2 in
from the rim, at distance 3 from the axis, and to the side 2 below the rim, at height 8: the
quadratic Bézier curve whose control points are those two points and the edge's point between
them, (ρ, z) = (3, 10), (5, 10), (5, 8), where ρ is the distance from the axis. (A conic blend's
rho is where the conic crosses the line from the edge's point to the middle of the chord between
the two tangent points, as a fraction of the way from the chord: 0.5 makes it a parabola, as here;
a circular round, which this is not, has rho √2 − 1 ≈ 0.414.) The blend is that parabola turned
round the z axis, and the corner outside it is removed.

It is the blend of `fillet/conic-edge`, run round a circular edge instead of a straight one. The
top is left the disk of radius 3 and the side ends at z = 8.

**As a set:**

$$\{\, \rho \le 5,\ 0 \le z \le 10 \,\} \setminus \bigl\{\, \rho > 3,\ z > 8,\ \sqrt{5 - \rho} + \sqrt{10 - z} < \sqrt{2} \,\bigr\}$$

(the parabola, in u = 5 − ρ and v = 10 − z, is √u + √v = √2).
