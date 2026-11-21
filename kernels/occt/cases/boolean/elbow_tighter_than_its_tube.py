"""An elbow tighter than its tube. A round tube of radius 20 turned a quarter turn about the y
axis, its centre-line on the circle of radius 18 about it from (0, 0, 18) to (18, 0, 0), with a
straight leg of length 40 at each end: one along −x from the disk in the plane x = 0, the other
along −z from the disk in the plane z = 0. The turn is every point its end disk passes through as
it turns; the disk crosses the axis, which lies 18 from its centre."""
from math import pi, sqrt

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeRevol
from OCP.GC import GC_MakeArcOfCircle
from OCP.gp import gp, gp_Ax1, gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused

TUBE = 20
BEND = 18
LEG = 40


def build():
    # The disk at the turn's start, in the plane x = 0 about (0, 0, 18). Its part beyond the axis
    # (z < 0) turns into the leg along −x, so the turn is built from the disk cut at the axis:
    # the long arc from (0, h, 0) round through (0, 0, 38) to (0, −h, 0), closed along the axis.
    h = sqrt(TUBE ** 2 - BEND ** 2)
    circle = gp_Circ(gp_Ax2(gp_Pnt(0, 0, BEND), gp.DX_s()), TUBE)
    top, bottom = gp_Pnt(0, h, 0), gp_Pnt(0, -h, 0)
    arc = GC_MakeArcOfCircle(circle, top, bottom, True).Value()
    if arc.Value(0.5 * (arc.FirstParameter() + arc.LastParameter())).Z() < BEND:
        arc = GC_MakeArcOfCircle(circle, bottom, top, True).Value()
    wire = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(arc).Edge(),
                                   BRepBuilderAPI_MakeEdge(bottom, top).Edge()).Wire()
    disk = BRepBuilderAPI_MakeFace(wire, True).Face()
    # a quarter turn about +y takes +z to +x
    turn = BRepPrimAPI_MakeRevol(disk, gp_Ax1(gp_Pnt(0, 0, 0), gp.DY_s()), pi / 2).Shape()
    # the legs, each running 40 back from its end of the turn
    leg_x = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(-LEG, 0, BEND), gp.DX_s()), TUBE, LEG).Shape()
    leg_z = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(BEND, 0, -LEG), gp.DZ_s()), TUBE, LEG).Shape()
    body = turn
    for leg in (leg_x, leg_z):
        fuse = BRepAlgoAPI_Fuse(body, leg)
        if not fuse.IsDone():
            raise Refused("BRepAlgoAPI_Fuse declined a leg")
        body = fuse.Shape()
    return body
