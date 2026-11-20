+++
title = "A plate with the rim of its elliptic hole chamfered"
tags = ["chamfer", "ellipse", "ruled", "curved-edge", "hole"]

[volume]
value = 2158.2254590595167
exact = "2400 − 72π − (d²L/2 + 2πd³/6), d = 1, L = 24E(e²), e² = 0.75, the ellipse's perimeter, L = 29.065344661643028 (round a hole each section's area counts at the rate its points' parallel curve runs, L + 2πs at s out from the hole)"
tex = '2400 - 72\pi - \left(\tfrac{d^2 L}{2} + \tfrac{2\pi d^3}{6}\right),\quad d = 1,\ L = 24\,E(0.75) = 29.065344661643028'

[area]
value = 1587.439248694166
exact = "1600 − 36π + 4L − 2dL − πd² + √2·d(L + πd), d = 1, L = 24E(0.75) = 29.065344661643028: the top loses dL + πd², the hole's wall dL, and the chamfer is ∫ over its slant of length d√2 of L + 2πs, s from 0 to d"
tex = '1600 - 36\pi + 4L - 2dL - \pi d^2 + \sqrt{2}\,d\,(L + \pi d),\quad d = 1,\ L = 24\,E(0.75) = 29.065344661643028'
+++

The plate of `fillet/elliptic-hole-rim`: from (−15, −10, 0) to (15, 10, 4), with a hole right
through it on the ellipse x²/6² + y²/3² = 1 (semi-axes 6 along x and 3 along y). Chamfer the
hole's rim on the top face, the ellipse where the hole's wall meets the top at z = 4, by 1 on both
faces: 1 out from the rim across the top, square to it, and 1 down the wall. The rim on the bottom
face stays sharp.

In each plane square to the rim the chamfer's section is the straight line from the point 1 down
the wall to the point 1 out across the top, at 45° to both. So the chamfer is the ruled surface
between the ellipse at z = 3 on the wall and the ellipse's outer parallel curve at distance 1 on
the top, each ruling square to the rim: a countersink of constant slope 45°, developable, and not a
cone. The hole's mouth on the top is the region inside that parallel curve (not an ellipse).

**As a set:**

$$\bigl([-15, 15] \times [-10, 10] \times [0, 4]\bigr) \setminus \bigl(\{\, x^2/6^2 + y^2/3^2 < 1 \,\} \cup \{\, z > 3 + s \,\}\bigr)$$

where s is the point's distance, in its horizontal plane, from the ellipse, taken outside it.
