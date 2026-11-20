"""A hexagonal prism's edge rounded with a radius growing along it. A regular hexagonal prism from z =
0 to z = 10 on the hexagon of side 6 centered on the z axis whose corners are at (6 cos 60k°, 6 sin
60k°), k = 0, …, 5. Its sides meet at 120° inside it. Round its edge from (6, 0, 0) to (6, 0, 10)
with a radius that grows linearly along it: r = 1 + 3z/10, so 1 at the bottom face z = 0 and 4 at
the top face z = 10.

The round's sections square to the edge are circular: at each height z its section by the
horizontal plane is the arc of radius r(z) centered at (6 − 2r/√3, 0, z), on the plane y = 0 that
bisects the corner, turning through 60° between where it touches the two sides, each r/√3 from the
edge, at (6 − r/2, ±r√3/6, z). It cuts the bottom face with the arc of radius 1 and the top face
with the arc of radius 4, and the region between each arc and its corner, of area (1/√3 − π/6)r²,
is gone from them. (So the round is part of an oblique circular cone with its apex at (6, 0,
−10/3), where the radius would be nought.)

An obtuse edge takes a variable round's radius back only a little: at the top the ball of radius 4
touches the sides 2.31 from the edge, and its arc passes 0.62 inside the edge."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from blend import edges_where
from rims import rounded_variably

SIDE, HEIGHT = 6, 10
RADIUS_BOTTOM, RADIUS_TOP = 1, 4  # at z = 0 and z = 10
TOLERANCE = 1e-7


def build():
    polygon = BRepBuilderAPI_MakePolygon()
    for k in range(6):
        a = math.radians(60 * k)
        polygon.Add(gp_Pnt(SIDE * math.cos(a), SIDE * math.sin(a), 0))
    polygon.Close()
    face = BRepBuilderAPI_MakeFace(polygon.Wire(), True).Face()
    prism = BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, HEIGHT)).Shape()
    [edge] = edges_where(prism, lambda p: abs(p.X() - SIDE) < TOLERANCE and abs(p.Y()) < TOLERANCE)
    return rounded_variably(prism, edge, (lambda p: p.Z() < HEIGHT / 2, RADIUS_BOTTOM),
                            (None, RADIUS_TOP), "the edge with a growing radius")
