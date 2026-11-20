+++
title = "An inside edge rounded with a radius growing along it"
tags = ["fillet", "variable-radius", "concave", "cone"]

[volume]
value = 1289.299412919444
exact = "1280 + (1 − π/4)∫₀¹⁰(1 + y/5)²dy = 1280 + (130/3)(1 − π/4)"
tex = '1280 + \left(1 - \tfrac{\pi}{4}\right)\int_0^{10}\left(1 + \tfrac{y}{5}\right)^2 dy = 1280 + \tfrac{130}{3}\left(1 - \tfrac{\pi}{4}\right)'

[area]
value = 889.6185084327392
exact = "896 − 2∫₀¹⁰ r dy + (1 − π/4)(1² + 3²) + ∫₀¹⁰ r dy·I, r = 1 + y/5, r′ = 1/5: 856 + 10(1 − π/4) + 20·I, I = ∫₀^{π/2}√(1 + (1 − sin θ − cos θ)²/25)dθ = 1.573624503335686"
tex = '\begin{array}{l} 856 + 10\left(1 - \tfrac{\pi}{4}\right) + 20I \\ I = \displaystyle\int_0^{\pi/2}\sqrt{1 + \tfrac{(1 - \sin\theta - \cos\theta)^2}{25}}\,d\theta = 1.573624503335686 \end{array}'
+++

An L-shaped block 10 deep: the L through (0, 0), (20, 0), (20, 4), (6, 4), (6, 12), (0, 12), as
(x, z), extruded from y = 0 to y = 10. It is a foot from the origin to (20, 10, 4) with an
upright from the origin to (6, 10, 12) standing on its end, and its one inside edge is the line
from (6, 0, 4) to (6, 10, 4), where the foot's top z = 4 meets the upright's face x = 6.

Round that inside edge with a radius that grows linearly along it: r = 1 + y/5, so 1 at the face
y = 0 and 3 at the face y = 10. The round fills the inside corner, adding material.

The round's sections square to the edge are circular: in each plane y = c its section is the
quarter circle of radius r(c) centered at (6 + r, c, 4 + r), tangent to the foot's top at
(6 + r, c, 4) and to the upright's face at (6, c, 4 + r), and everything between that arc and the
corner is added. (So the round is part of an oblique circular cone with its apex at (6, −5, 4),
where the radius would be nought.) The faces y = 0 and y = 10 each gain the region between the
corner and the arc, of area (1 − π/4)r²: 1 − π/4 and 9(1 − π/4).

**As a set:**

$$K \cup \bigl\{\, 6 \le x < 6 + r,\ 4 \le z < 4 + r,\ (x - 6 - r)^2 + (z - 4 - r)^2 > r^2,\ 0 \le y \le 10 \,\bigr\},\quad r = 1 + y/5$$

where K is the L-shaped block.
