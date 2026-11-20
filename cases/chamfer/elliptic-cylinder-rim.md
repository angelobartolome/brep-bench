+++
title = "An elliptic cylinder with its top rim chamfered"
tags = ["chamfer", "ellipse", "ruled", "curved-edge"]

[volume]
value = 1414.234058679081
exact = "480π − (d²L/2 − 2πd³/6), d = 2, L = 40E(e²), e² = 0.64, the ellipse's perimeter: 480π − 2L + 8π/3, L = 51.053997726796254"
tex = '480\pi - \left(\tfrac{d^2 L}{2} - \tfrac{2\pi d^3}{6}\right) = 480\pi - 2L + \tfrac{8\pi}{3},\quad d = 2,\ L = 40\,E(0.64) = 51.053997726796254'

[area]
value = 720.4044601968868
exact = "120π + 8L − 2dL + πd² + √2·d(L − πd), d = 2, L = 40E(0.64) = 51.053997726796254: the top loses dL − πd², the side dL, and the chamfer is ∫ over its slant of length d√2 of L − 2πs, s from 0 to d"
tex = '120\pi + 8L - 2dL + \pi d^2 + \sqrt{2}\,d\,(L - \pi d),\quad d = 2,\ L = 40\,E(0.64) = 51.053997726796254'
+++

An elliptic cylinder from z = 0 to z = 8 on the ellipse x²/10² + y²/6² = 1 (semi-axes 10 along x
and 6 along y). Chamfer the ellipse where its side meets its top, at z = 8, by 2 on both faces: 2
in from the rim across the top, square to it, and 2 down the side.

In each plane square to the rim the chamfer's section is the straight line from the point 2 down
the side to the point 2 in across the top, at 45° to both. So the chamfer is the ruled surface
between the ellipse's inner parallel curve at distance 2 on the top and the ellipse at z = 6 on the
side, each ruling square to the rim: a surface of constant slope 45°, developable, and not a cone.
The ellipse's least radius of curvature, 3.6, is more than 2, so the parallel curve is smooth.

**As a set:**

$$\{\, x^2/10^2 + y^2/6^2 \le 1,\ 0 \le z \le 8 \,\} \cap \{\, z \le 6 + s \,\}$$

where s is the point's distance, in its horizontal plane, from the ellipse.
