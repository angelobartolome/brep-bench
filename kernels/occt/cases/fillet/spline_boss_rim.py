"""A spline prism with a dent, its top rim rounded. A prism from z = 0 to z = 6 on a closed spline:
the periodic uniform cubic B-spline whose eight control points, in order round it, are (−10, −5),
(0, −7), (10, −5), (12, 4), (4, 7), (0, 3), (−4, 7), (−12, 4). It is a smooth bean 21.6 across,
from x = −10.79 to 10.79, dented at the top where the control point (0, 3) pulls it in: there the
curve turns the other way, its radius of curvature 2 at the dent's middle, (0, 4.33). Its tightest
convex turn has radius of curvature 3.87, at (±10.33, 3).

(Periodic and uniform: the curve is eight cubic segments, the i-th the uniform B-spline segment on
the control points i to i + 3, counted round the eight, so it is curvature continuous everywhere.
As a B-spline that is not periodic it is the same curve on eleven control points, the eight with
the first three repeated after them, and the knots 0, 1, …, 14, run from the knot 3 to the knot
11.)

Round the spline where the prism's side meets its top, at z = 6, to radius 1.

The round is the surface a ball of radius 1 sweeps as it rolls round the inside of the rim touching
the top and the side, its center on the spline's inner parallel curve at distance 1, at height 5: a
pipe that is convex round the bean's ends and saddle-shaped through the dent. In each plane square
to the rim its section is the quarter circle of radius 1 tangent to the side 1 below the top and to
the top 1 in from the rim. The radius is less than every convex radius of curvature of the spline,
so the parallel curve is smooth."""
from rims import edges_at_height, rounded, spline_prism

HEIGHT = 6
RADIUS = 1


def build():
    bean = spline_prism(HEIGHT)
    return rounded(bean, RADIUS, edges_at_height(bean, HEIGHT), "the spline's rim")
