"""A countersunk hole undercutting a counterbore's wall. A disc, the solid cylinder of radius 100
about the z axis from z = −20 to z = 20, counterbored to radius 60 from z = 0 up through its top. A
hole of radius 3.5 about the vertical line through (59.9, 0), 0.1 inside the counterbore's wall,
from its flat bottom z = −15 up to z = 0, flush with the floor; and its countersink, the 90° cone
about the same axis whose circle on the floor plane has radius 4, its point 4 below, cut away.
Both reach under the wall, where the countersink's circle leaves a ceiling facing down."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

AXIS = (59.9, 0)
HOLE_RADIUS, HOLE_DEPTH = 3.5, 15
SINK_RADIUS = 4


def cut(body, tool, what):
    operation = BRepAlgoAPI_Cut(body, tool)
    if not operation.IsDone():
        raise Refused(f"BRepAlgoAPI_Cut could not cut {what}")
    return operation.Shape()


def build():
    disc = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -20), gp.DZ_s()), 100, 40).Shape()
    # The counterbore's cutter runs 1 past the disc's top.
    counterbore = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 60, 21).Shape()
    body = cut(disc, counterbore, "the counterbore")
    # The hole's cutter ends flush with the floor, as the case says.
    hole = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(*AXIS, -HOLE_DEPTH), gp.DZ_s()), HOLE_RADIUS,
                                    HOLE_DEPTH).Shape()
    body = cut(body, hole, "the hole")
    # The countersink: the 90° cone from its point, SINK_RADIUS below the floor, to its circle of
    # radius SINK_RADIUS on the floor plane.
    sink = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(*AXIS, -SINK_RADIUS), gp.DZ_s()), 0, SINK_RADIUS,
                                SINK_RADIUS).Shape()
    return cut(body, sink, "the countersink")
