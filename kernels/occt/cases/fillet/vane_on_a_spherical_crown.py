"""A vane on a spherical crown. A disc turned about the z axis: flat beneath at z = 0, radius 100,
its rim 3 high, its top the sphere of radius 800 through the rim's top circle, centred on the z
axis below. On it stands an obround vane, the points within 2 of the segment from (22, 0) to
(88, 0), from z = 1 up to z = 31, in union with it. Round the loop where the vane's sides meet the
crown to radius 2."""
from math import pi, sqrt

from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism, BRepPrimAPI_MakeRevol
from OCP.GC import GC_MakeArcOfCircle
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp, gp_Ax1, gp_Ax2, gp_Circ, gp_Dir, gp_Pnt, gp_Vec

from bench import Refused

RADIUS = 100
RIM = 3
SPHERE = 800
CENTRE = RIM - sqrt(SPHERE ** 2 - RADIUS ** 2)  # the sphere's centre on the z axis
HALF = 2                                       # the vane's half width
ENDS = (22, 88)                                # its segment's ends on the x axis
BOTTOM, TOP = 1, 31
ROUND = 2
TOLERANCE = 1e-6


def wire(*edges):
    made = BRepBuilderAPI_MakeWire()
    for edge in edges:
        made.Add(edge)
    return made.Wire()


def edge(a, b):
    return BRepBuilderAPI_MakeEdge(gp_Pnt(*a), gp_Pnt(*b)).Edge()


def arc(a, m, b):
    return BRepBuilderAPI_MakeEdge(GC_MakeArcOfCircle(gp_Pnt(*a), gp_Pnt(*m), gp_Pnt(*b)).Value()).Edge()


def build():
    # the half section in the xz plane, turned whole about z
    peak = CENTRE + SPHERE
    x, z = RADIUS / 2, CENTRE + sqrt(SPHERE ** 2 - (RADIUS / 2) ** 2)
    section = wire(edge((0, 0, 0), (RADIUS, 0, 0)), edge((RADIUS, 0, 0), (RADIUS, 0, RIM)),
                   arc((RADIUS, 0, RIM), (x, 0, z), (0, 0, peak)), edge((0, 0, peak), (0, 0, 0)))
    disc = BRepPrimAPI_MakeRevol(BRepBuilderAPI_MakeFace(section, True).Face(),
                                 gp_Ax1(gp_Pnt(0, 0, 0), gp.DZ_s()), 2 * pi).Shape()
    # the obround at z = 1, extruded up to z = 31
    a, b = ENDS
    foot = wire(edge((a, -HALF, BOTTOM), (b, -HALF, BOTTOM)),
                arc((b, -HALF, BOTTOM), (b + HALF, 0, BOTTOM), (b, HALF, BOTTOM)),
                edge((b, HALF, BOTTOM), (a, HALF, BOTTOM)),
                arc((a, HALF, BOTTOM), (a - HALF, 0, BOTTOM), (a, -HALF, BOTTOM)))
    vane = BRepPrimAPI_MakePrism(BRepBuilderAPI_MakeFace(foot, True).Face(),
                                 gp_Vec(0, 0, TOP - BOTTOM)).Shape()
    fuse = BRepAlgoAPI_Fuse(disc, vane)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the vane")
    body = fuse.Shape()
    # the loop where the vane meets the crown: edges on the sphere, 2 from the vane's segment
    fillet = BRepFilletAPI_MakeFillet(body)
    found = TopExp_Explorer(body, TopAbs_EDGE)
    picked = []
    while found.More():
        e = TopoDS.Edge(found.Current())
        curve = BRepAdaptor_Curve(e)
        p = curve.Value(0.5 * (curve.FirstParameter() + curve.LastParameter()))
        on_sphere = abs(gp_Pnt(0, 0, CENTRE).Distance(p) - SPHERE) < TOLERANCE
        along = min(max(p.X(), a), b)
        on_vane = abs(sqrt((p.X() - along) ** 2 + p.Y() ** 2) - HALF) < TOLERANCE
        if on_sphere and on_vane and not any(e.IsSame(q) for q in picked):
            picked.append(e)
            fillet.Add(ROUND, e)
        found.Next()
    assert fillet.NbContours() == 1, f"{fillet.NbContours()} chains round the vane's foot"
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the vane's foot")
    return fillet.Shape()
