"""A pocket's floor rounded as tight as its corners. The block from (−40, −30, 0) to (40, 30, 16)
less a pocket: the rectangle from (−25, −15) to (25, 15) with its corners rounded to radius 4, cut
down from the top to the floor z = 6. The rim of the pocket's floor, four straight edges and four
quarter circles tangent all the way round, rounded to radius 4, filling: the radius of the corners,
so round each corner the round is an eighth of a ball about the corner's axis."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Pnt

from bench import Refused

FLOOR = 6
CORNER = 4
ROUND = 4
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
    block = BRepPrimAPI_MakeBox(gp_Pnt(-40, -30, 0), gp_Pnt(40, 30, 16)).Shape()
    # the pocket, a box running out past the top, its four upright edges rounded
    cutter = BRepPrimAPI_MakeBox(gp_Pnt(-25, -15, FLOOR), gp_Pnt(25, 15, 17)).Shape()
    corners = BRepFilletAPI_MakeFillet(cutter)
    for edge in edges(cutter):
        a, b = ends(edge)
        if abs(a.X() - b.X()) < TOLERANCE and abs(a.Y() - b.Y()) < TOLERANCE:
            corners.Add(CORNER, edge)
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
