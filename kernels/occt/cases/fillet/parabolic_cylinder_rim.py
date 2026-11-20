"""A cylinder's rim blended by a parabolic conic. A cylinder of radius 5 on the z axis, from z = 0 to
z = 10. Blend the circle where its side meets its top with a conic blend of rho 0.5, a parabola,
tangent to the top and to the side each 2 from the edge.

In each half plane through the z axis the blend's section is the parabola tangent to the top 2 in
from the rim, at distance 3 from the axis, and to the side 2 below the rim, at height 8: the
quadratic Bézier curve whose control points are those two points and the edge's point between them,
(ρ, z) = (3, 10), (5, 10), (5, 8), where ρ is the distance from the axis. (A conic blend's rho is
where the conic crosses the line from the edge's point to the middle of the chord between the two
tangent points, as a fraction of the way from the chord: 0.5 makes it a parabola, as here; a
circular round, which this is not, has rho √2 − 1 ≈ 0.414.) The blend is that parabola turned round
the z axis, and the corner outside it is removed.

It is the blend of `fillet/conic-edge`, run round a circular edge instead of a straight one. The
top is left the disk of radius 3 and the side ends at z = 8."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge
from OCP.Geom import Geom_BezierCurve
from OCP.collections import Array1_gp_Pnt
from OCP.gp import gp_Pnt

from rims import segment, turned

RADIUS, HEIGHT, SETBACK = 5, 10, 2


def build():
    # Workaround: OCCT's fillets have a circular section and it has no conic blend. The blended
    # cylinder is a solid of revolution, so it is its half section turned round the z axis: the
    # parabola, the quadratic Bézier curve on (3, 10), (5, 10), (5, 8) as (x, z), closed by the
    # axis, the bottom, the side and the top.
    poles = Array1_gp_Pnt(1, 3)
    for i, (x, z) in enumerate([(RADIUS - SETBACK, HEIGHT), (RADIUS, HEIGHT),
                                (RADIUS, HEIGHT - SETBACK)]):
        poles.SetValue(i + 1, gp_Pnt(x, 0, z))
    parabola = BRepBuilderAPI_MakeEdge(Geom_BezierCurve(poles)).Edge()
    return turned([
        parabola,
        segment((RADIUS, 0, HEIGHT - SETBACK), (RADIUS, 0, 0)),
        segment((RADIUS, 0, 0), (0, 0, 0)),
        segment((0, 0, 0), (0, 0, HEIGHT)),
        segment((0, 0, HEIGHT), (RADIUS - SETBACK, 0, HEIGHT)),
    ])
