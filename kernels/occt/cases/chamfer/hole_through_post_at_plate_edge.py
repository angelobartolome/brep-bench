"""A hole through a post standing across a plate's edge, both rims chamfered. The plate is the box
from (−50, −50, 0) to (50, 50, 20); the post the solid cylinder of radius 15 about the vertical line
through (45, 0), from z = 0 to z = 40, 5 inside the plate's side x = 50, so a segment of it
overhangs, its foot flush with the plate's bottom; the two are one solid. A hole of radius 6 is
drilled right through along the post's axis, its circle on the bottom face crossing the line
x = 50. Both rims of the hole, on the bottom face and on the post's top, are chamfered 1.5 along
each face, at 45°."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepFilletAPI import BRepFilletAPI_MakeChamfer
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.GeomAbs import GeomAbs_CurveType
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

AXIS = (45, 0)
POST_RADIUS, POST_TOP = 15, 40
HOLE_RADIUS = 6
CHAMFER = 1.5


def on_rim(edge):
    """Whether `edge` is a piece of a circle of the hole's radius about the post's axis, at the
    bottom face or the post's top."""
    curve = BRepAdaptor_Curve(edge)
    if curve.GetType() != GeomAbs_CurveType.GeomAbs_Circle:
        return False
    circle = curve.Circle()
    centre = circle.Location()
    return (abs(circle.Radius() - HOLE_RADIUS) < 1e-7
            and abs(centre.X() - AXIS[0]) < 1e-7 and abs(centre.Y() - AXIS[1]) < 1e-7
            and min(abs(centre.Z()), abs(centre.Z() - POST_TOP)) < 1e-7)


def build():
    plate = BRepPrimAPI_MakeBox(gp_Pnt(-50, -50, 0), 100, 100, 20).Shape()
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(*AXIS, 0), gp.DZ_s()), POST_RADIUS,
                                    POST_TOP).Shape()
    fuse = BRepAlgoAPI_Fuse(plate, post)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the post to the plate")
    # The plate's bottom and the post's overhanging foot are one plane: merge them into one face.
    fuse.SimplifyResult()
    body = fuse.Shape()
    # The drill runs 1 past both ends.
    drill = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(*AXIS, -1), gp.DZ_s()), HOLE_RADIUS,
                                     POST_TOP + 2).Shape()
    cut = BRepAlgoAPI_Cut(body, drill)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not drill the post")
    body = cut.Shape()
    chamfer = BRepFilletAPI_MakeChamfer(body)
    found = []
    edges = TopExp_Explorer(body, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        if on_rim(edge) and not any(edge.IsSame(e) for e in found):
            found.append(edge)
            chamfer.Add(CHAMFER, edge)
        edges.Next()
    chamfer.Build()
    if not chamfer.IsDone():
        raise Refused("BRepFilletAPI_MakeChamfer could not chamfer the hole's rims")
    return chamfer.Shape()
