"""A spline prism with a dent, its top rim chamfered. The prism of `fillet/spline-boss-rim`: from z =
0 to z = 6 on the periodic uniform cubic B-spline whose eight control points, in order round it,
are (−10, −5), (0, −7), (10, −5), (12, 4), (4, 7), (0, 3), (−4, 7), (−12, 4), a bean dented at the
top. (The curve is eight cubic segments, the i-th the uniform B-spline segment on the control
points i to i + 3, counted round the eight.)

Chamfer the spline where the prism's side meets its top, at z = 6, by 1 on both faces: 1 in from
the rim across the top, square to it, and 1 down the side.

In each plane square to the rim the chamfer's section is the straight line from the point 1 down
the side to the point 1 in across the top, at 45° to both. So the chamfer is the ruled surface
between the spline's inner parallel curve at distance 1 on the top and the spline at z = 5 on the
side, each ruling square to the rim: a surface of constant slope 45°, convex round the bean's ends
and concave through the dent. The spline's convex radii of curvature are all above 3.87, so the
parallel curve is smooth."""
from rims import chamfered, edges_at_height, spline_prism

HEIGHT = 6
DISTANCE = 1


def build():
    bean = spline_prism(HEIGHT)
    return chamfered(bean, DISTANCE, edges_at_height(bean, HEIGHT), "the spline's rim")
