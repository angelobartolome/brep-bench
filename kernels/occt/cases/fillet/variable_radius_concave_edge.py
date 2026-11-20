"""An inside edge rounded with a radius growing along it. An L-shaped block 10 deep: the L through (0,
0), (20, 0), (20, 4), (6, 4), (6, 12), (0, 12), as (x, z), extruded from y = 0 to y = 10. It is a
foot from the origin to (20, 10, 4) with an upright from the origin to (6, 10, 12) standing on its
end, and its one inside edge is the line from (6, 0, 4) to (6, 10, 4), where the foot's top z = 4
meets the upright's face x = 6.

Round that inside edge with a radius that grows linearly along it: r = 1 + y/5, so 1 at the face y
= 0 and 3 at the face y = 10. The round fills the inside corner, adding material.

The round's sections square to the edge are circular: in each plane y = c its section is the
quarter circle of radius r(c) centered at (6 + r, c, 4 + r), tangent to the foot's top at (6 + r,
c, 4) and to the upright's face at (6, c, 4 + r), and everything between that arc and the corner is
added. (So the round is part of an oblique circular cone with its apex at (6, −5, 4), where the
radius would be nought.) The faces y = 0 and y = 10 each gain the region between the corner and the
arc, of area (1 − π/4)r²: 1 − π/4 and 9(1 − π/4)."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from blend import edges_where
from rims import rounded_variably

L_SHAPE = [(0, 0), (20, 0), (20, 4), (6, 4), (6, 12), (0, 12)]  # (x, z)
DEPTH = 10
RADIUS_FRONT, RADIUS_BACK = 1, 3  # at y = 0 and y = 10
TOLERANCE = 1e-7


def build():
    polygon = BRepBuilderAPI_MakePolygon()
    for x, z in L_SHAPE:
        polygon.Add(gp_Pnt(x, 0, z))
    polygon.Close()
    face = BRepBuilderAPI_MakeFace(polygon.Wire(), True).Face()
    block = BRepPrimAPI_MakePrism(face, gp_Vec(0, DEPTH, 0)).Shape()
    [edge] = edges_where(block, lambda p: abs(p.X() - 6) < TOLERANCE and abs(p.Z() - 4) < TOLERANCE)
    return rounded_variably(block, edge, (lambda p: p.Y() < DEPTH / 2, RADIUS_FRONT),
                            (None, RADIUS_BACK), "the inside edge with a growing radius")
