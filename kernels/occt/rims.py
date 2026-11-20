"""What the OCCT implementations of the rim cases share: a prism on a closed curve, the edges of
its top, a half cylinder, a variable round of one edge, and a solid turned from a profile."""
import math

from OCP.BRep import BRep_Tool
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepFilletAPI import BRepFilletAPI_MakeChamfer, BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakePrism, BRepPrimAPI_MakeRevol
from OCP.Geom import Geom_BSplineCurve
from OCP.collections import Array1_gp_Pnt, Array1_int, Array1_double
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp, TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Ax1, gp_Ax2, gp_Dir, gp_Pnt, gp_Vec

from bench import Refused

TOLERANCE = 1e-7


def prism(edges, height):
    """The prism from z = 0 to `height` on the region the closed loop of `edges` (in z = 0)
    bounds."""
    wire = BRepBuilderAPI_MakeWire()
    for edge in edges:
        wire.Add(edge)
    face = BRepBuilderAPI_MakeFace(wire.Wire(), True).Face()
    return BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, height)).Shape()


def edges_at_height(solid, z):
    """The edges of `solid` lying wholly in the plane at height `z`, each once."""
    found = []
    edges = TopExp_Explorer(solid, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        curve = BRepAdaptor_Curve(edge)
        a, b = curve.FirstParameter(), curve.LastParameter()
        if (all(abs(curve.Value(a + (b - a) * k / 8).Z() - z) < TOLERANCE for k in range(9))
                and not any(edge.IsSame(e) for e in found)):
            found.append(edge)
        edges.Next()
    return found


def rounded(solid, radius, edges, what):
    """`solid` with `edges` rounded to `radius`; Refused, naming `what`, if OCCT declines."""
    fillet = BRepFilletAPI_MakeFillet(solid)
    for edge in edges:
        fillet.Add(radius, edge)
    fillet.Build()
    if not fillet.IsDone():
        raise Refused(f"BRepFilletAPI_MakeFillet could not round {what}")
    return fillet.Shape()


def chamfered(solid, distance, edges, what):
    """`solid` with `edges` chamfered by `distance` on both faces; Refused, naming `what`."""
    chamfer = BRepFilletAPI_MakeChamfer(solid)
    for edge in edges:
        chamfer.Add(distance, edge)
    chamfer.Build()
    if not chamfer.IsDone():
        raise Refused(f"BRepFilletAPI_MakeChamfer could not chamfer {what}")
    return chamfer.Shape()


def rounded_variably(solid, edge, at_first, at_last, what):
    """`solid` with `edge` rounded by a radius running linearly from `at_first` at the end where
    `at_first` is a test of a gp_Pnt that holds to `at_last` at the other (a pair (test, radius)
    each)."""
    first = BRep_Tool.Pnt_s(TopExp.FirstVertex_s(edge))
    (test, r_a), (_, r_b) = at_first, at_last
    fillet = BRepFilletAPI_MakeFillet(solid)
    # A linear law from the spine's first vertex to its last: give each end its radius.
    if test(first):
        fillet.Add(r_a, r_b, edge)
    else:
        fillet.Add(r_b, r_a, edge)
    fillet.Build()
    if not fillet.IsDone():
        raise Refused(f"BRepFilletAPI_MakeFillet could not round {what}")
    return fillet.Shape()


def half_cylinder(radius, height):
    """The half of the cylinder of `radius` on the z axis from z = 0 to `height` on the side
    x ≥ 0: a cylinder of a half turn from −y round through +x to +y."""
    frame = gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1), gp_Dir(0, -1, 0))
    return BRepPrimAPI_MakeCylinder(frame, radius, height, math.pi).Shape()


def turned(edges):
    """The solid the closed loop of `edges`, in the half plane y = 0, x ≥ 0, sweeps turned once
    round the z axis."""
    wire = BRepBuilderAPI_MakeWire()
    for edge in edges:
        wire.Add(edge)
    face = BRepBuilderAPI_MakeFace(wire.Wire(), True).Face()
    return BRepPrimAPI_MakeRevol(face, gp_Ax1(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1))).Shape()


BEAN = [(-10, -5), (0, -7), (10, -5), (12, 4), (4, 7), (0, 3), (-4, 7), (-12, 4)]


def spline_prism(height):
    """The prism from z = 0 to `height` on the bean of the spline rim cases: the periodic uniform
    cubic B-spline on the eight control points BEAN."""
    poles = Array1_gp_Pnt(1, len(BEAN))
    for i, (x, y) in enumerate(BEAN):
        poles.SetValue(i + 1, gp_Pnt(x, y, 0))
    # periodic: one knot more than the poles, each simple, evenly spaced
    knots = Array1_double(1, len(BEAN) + 1)
    multiplicities = Array1_int(1, len(BEAN) + 1)
    for i in range(len(BEAN) + 1):
        knots.SetValue(i + 1, float(i))
        multiplicities.SetValue(i + 1, 1)
    curve = Geom_BSplineCurve(poles, knots, multiplicities, 3, True)
    return prism([BRepBuilderAPI_MakeEdge(curve).Edge()], height)


def segment(a, b):
    """The straight edge from the point `a` to `b`, each (x, y, z)."""
    return BRepBuilderAPI_MakeEdge(gp_Pnt(*a), gp_Pnt(*b)).Edge()
