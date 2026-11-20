"""A cube edge rounded with a radius shrinking to nothing. A cube of side 10 between the origin and
(10, 10, 10). Round the edge where its faces x = 10 and y = 10 meet, the line from (10, 10, 0) to
(10, 10, 10), with a radius that shrinks linearly along it to nothing: r = 2 − z/5, so 2 at the
bottom face z = 0 and 0 at the top face z = 10.

The round's sections square to the edge are circular: at each height z the round's section by the
horizontal plane is the quarter circle of radius r(z) centered at (10 − r, 10 − r, z), tangent to
the face x = 10 at (10, 10 − r, z) and to the face y = 10 at (10 − r, 10, z). Since the radius
falls to nought at the top, the round is part of an oblique circular cone whose apex is the cube's
corner (10, 10, 10): its lines run from that corner down to the quarter circle of radius 2 it cuts
from the bottom face. It meets each of the faces x = 10 and y = 10 along a straight line, from (10,
8, 0) or (8, 10, 0) up to the corner, so each of those faces loses a triangle of legs 2 and 10. The
top face is the whole square: its corner (10, 10, 10) is the cone's apex, where the round comes to
a point.

It is `fillet/variable-radius-edge` with a radius that vanishes at one end, as a variable round set
to nought at a vertex does."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from blend import edges_where
from rims import rounded_variably

SIDE = 10
RADIUS_BOTTOM, RADIUS_TOP = 2, 0  # at z = 0 and z = 10
TOLERANCE = 1e-7


def build():
    cube = BRepPrimAPI_MakeBox(SIDE, SIDE, SIDE).Shape()
    [edge] = edges_where(cube, lambda p: abs(p.X() - SIDE) < TOLERANCE and abs(p.Y() - SIDE) < TOLERANCE)
    return rounded_variably(cube, edge, (lambda p: p.Z() < SIDE / 2, RADIUS_BOTTOM),
                            (None, RADIUS_TOP), "the edge with a radius shrinking to nought")
