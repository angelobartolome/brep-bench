+++
title = "A spline prism with a dent, its top rim chamfered"
tags = ["chamfer", "spline", "ruled", "curved-edge", "concave"]

[volume]
value = 1265.3106524626564
exact = "6A − (d²L/2 − 2πd³/6), d = 1, A = 6463/30, the spline's area, and L = 56.673090177080546, its length ∫|C′(t)|dt"
tex = '6A - \left(\tfrac{d^2 L}{2} - \tfrac{2\pi d^3}{6}\right),\quad d = 1,\ A = \tfrac{6463}{30},\ L = 56.673090177080546'

[area]
value = 736.405589840441
exact = "2A + 6L − 2dL + πd² + √2·d(L − πd), d = 1, A = 6463/30, L = 56.673090177080546"
tex = '2A + 6L - 2dL + \pi d^2 + \sqrt{2}\,d\,(L - \pi d),\quad d = 1,\ A = \tfrac{6463}{30},\ L = 56.673090177080546'
+++

The prism of `fillet/spline-boss-rim`: from z = 0 to z = 6 on the periodic uniform cubic B-spline
whose eight control points, in order round it, are (−10, −5), (0, −7), (10, −5), (12, 4), (4, 7),
(0, 3), (−4, 7), (−12, 4), a bean dented at the top. (The curve is eight cubic segments, the i-th
the uniform B-spline segment on the control points i to i + 3, counted round the eight.)

Chamfer the spline where the prism's side meets its top, at z = 6, by 1 on both faces: 1 in from
the rim across the top, square to it, and 1 down the side.

In each plane square to the rim the chamfer's section is the straight line from the point 1 down
the side to the point 1 in across the top, at 45° to both. So the chamfer is the ruled surface
between the spline's inner parallel curve at distance 1 on the top and the spline at z = 5 on the
side, each ruling square to the rim: a surface of constant slope 45°, convex round the bean's ends
and concave through the dent. The spline's convex radii of curvature are all above 3.87, so the
parallel curve is smooth.

**As a set:**

$$\{\, (x, y) \in S,\ 0 \le z \le 6 \,\} \cap \{\, z \le 5 + s \,\}$$

where S is the region the spline bounds and s the point's distance, in its horizontal plane, from
the spline.
