"""An eye across a bar. The bar, the box from (−40, −8, −5) to (40, 8, 5), joined with the eye,
the cylinder of radius 14 about the y axis from y = −8 to y = 8, its ends flush with the bar's
sides. The four straight edges where the eye's side meets the bar's top and bottom, lines along y
at x = ±√171, z = ±5, rounded to radius 3, filling, each round running the bar's whole width and
ending flush with its sides."""
from math import sqrt

from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

EYE = 14
HALF = 5          # the bar's half thickness
ROUND = 3
TOLERANCE = 1e-7


def on_junction(edge):
    """Whether `edge` is a line along y where the eye's side meets the bar's top or bottom."""
    curve = BRepAdaptor_Curve(edge)
    a, b = curve.Value(curve.FirstParameter()), curve.Value(curve.LastParameter())
    return (abs(a.X() - b.X()) < TOLERANCE and abs(a.Z() - b.Z()) < TOLERANCE
            and abs(abs(a.Z()) - HALF) < TOLERANCE
            and abs(abs(a.X()) - sqrt(EYE ** 2 - HALF ** 2)) < TOLERANCE)


def build():
    bar = BRepPrimAPI_MakeBox(gp_Pnt(-40, -8, -HALF), gp_Pnt(40, 8, HALF)).Shape()
    eye = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, -8, 0), gp_Dir(0, 1, 0)), EYE, 16).Shape()
    fuse = BRepAlgoAPI_Fuse(bar, eye)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the eye to the bar")
    joined = fuse.Shape()
    fillet = BRepFilletAPI_MakeFillet(joined)
    found = []
    edges = TopExp_Explorer(joined, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        if on_junction(edge) and not any(edge.IsSame(e) for e in found):
            found.append(edge)
            fillet.Add(ROUND, edge)
        edges.Next()
    assert len(found) == 4, f"{len(found)} junction edges"
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round where the eye meets the bar")
    return fillet.Shape()
