"""An L-shaped pocket's floor rounded as tight as its corners. The block from (−20, −20, 0) to
(20, 20, 8) less a pocket: the L through (−12, −12), (12, −12), (12, 0), (0, 0), (0, 12) and
(−12, 12), its five convex corners rounded to radius 1 and its re-entrant corner at the origin to
radius 3, cut down from the top to the floor z = 3. The rim of the pocket's floor, six straight
edges and six arcs tangent all the way round, rounded to radius 1, filling: round the five convex
corners an eighth of a ball, round the re-entrant one a quarter of a torus's tube about the round
of radius 3."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakePrism
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused

ELL = [(-12, -12), (12, -12), (12, 0), (0, 0), (0, 12), (-12, 12)]
INSIDE = (0, 0)   # the re-entrant corner
FLOOR = 3
CORNER = 1
INSIDE_CORNER = 3
ROUND = 1
TOLERANCE = 1e-7


def edges(shape):
    found = TopExp_Explorer(shape, TopAbs_EDGE)
    while found.More():
        yield TopoDS.Edge(found.Current())
        found.Next()


def ends(edge):
    curve = BRepAdaptor_Curve(edge)
    return curve.Value(curve.FirstParameter()), curve.Value(curve.LastParameter())


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(-20, -20, 0), gp_Pnt(20, 20, 8)).Shape()
    # the pocket, the L's prism running out past the top, its six upright edges rounded
    polygon = BRepBuilderAPI_MakePolygon()
    for x, y in ELL:
        polygon.Add(gp_Pnt(x, y, FLOOR))
    polygon.Close()
    section = BRepBuilderAPI_MakeFace(polygon.Wire()).Face()
    cutter = BRepPrimAPI_MakePrism(section, gp_Vec(0, 0, 9 - FLOOR)).Shape()
    corners = BRepFilletAPI_MakeFillet(cutter)
    for edge in edges(cutter):
        a, b = ends(edge)
        if abs(a.X() - b.X()) < TOLERANCE and abs(a.Y() - b.Y()) < TOLERANCE:
            inside = abs(a.X() - INSIDE[0]) < TOLERANCE and abs(a.Y() - INSIDE[1]) < TOLERANCE
            corners.Add(INSIDE_CORNER if inside else CORNER, edge)
    corners.Build()
    if not corners.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the pocket's corners")
    cut = BRepAlgoAPI_Cut(block, corners.Shape())
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not cut the pocket")
    pocketed = cut.Shape()
    # the floor's rim: every edge with both ends on the floor; one tangent chain
    rim = BRepFilletAPI_MakeFillet(pocketed)
    for edge in edges(pocketed):
        a, b = ends(edge)
        if abs(a.Z() - FLOOR) < TOLERANCE and abs(b.Z() - FLOOR) < TOLERANCE:
            rim.Add(ROUND, edge)
    assert rim.NbContours() == 1, f"{rim.NbContours()} chains round the floor"
    rim.Build()
    if not rim.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the pocket's floor")
    return rim.Shape()
