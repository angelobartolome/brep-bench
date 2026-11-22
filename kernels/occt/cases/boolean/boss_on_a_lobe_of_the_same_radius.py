"""A boss on a lobe of the same radius. A dog-bone plate, its outline in the plane z = 0 extruded
from z = 0 to z = 10: two round lobes of radius 10 about (±30, 0), joined by a waist whose sides run
along y = ±4, each side running into each lobe through a concave blend of radius 6 tangent to both.
The boss is the solid cylinder of radius 10 about the vertical line through (30, 0), from z = 0 to
z = 30, joined to the plate: where the lobe's outer arc runs, its side is the lobe's side."""
from math import sqrt

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakePrism
from OCP.GC import GC_MakeArcOfCircle
from OCP.gp import gp, gp_Ax2, gp_Circ, gp_Pnt, gp_Vec

from bench import Refused

LOBE_X, LOBE_RADIUS = 30, 10
WAIST = 4
BLEND = 6
THICKNESS, BOSS_TOP = 10, 30


def point(p):
    return gp_Pnt(p[0], p[1], 0)


def line(a, b):
    return BRepBuilderAPI_MakeEdge(point(a), point(b)).Edge()


def arc(centre, radius, a, b):
    """The arc of the circle about `centre` running counter-clockwise from `a` to `b`."""
    circle = gp_Circ(gp_Ax2(point(centre), gp.DZ_s()), radius)
    curve = GC_MakeArcOfCircle(circle, point(a), point(b), True).Value()
    return BRepBuilderAPI_MakeEdge(curve).Edge()


def build():
    # A blend's centre is BLEND off the waist's side and LOBE_RADIUS + BLEND from the lobe's
    # centre. It touches the side square to the side from its centre, and the lobe on the line
    # joining the two centres.
    reach, lift = LOBE_RADIUS + BLEND, WAIST + BLEND
    bx = LOBE_X - sqrt(reach ** 2 - lift ** 2)
    k = LOBE_RADIUS / reach
    tx, ty = LOBE_X + k * (bx - LOBE_X), k * lift   # where the upper right blend touches its lobe

    # The outline, walked counter-clockwise from the lower side. The blends are concave, so each
    # turns clockwise along the walk: made counter-clockwise from its far end to its near one.
    edges = [
        line((-bx, -WAIST), (bx, -WAIST)),
        arc((bx, -lift), BLEND, (tx, -ty), (bx, -WAIST)),
        arc((LOBE_X, 0), LOBE_RADIUS, (tx, -ty), (tx, ty)),
        arc((bx, lift), BLEND, (bx, WAIST), (tx, ty)),
        line((bx, WAIST), (-bx, WAIST)),
        arc((-bx, lift), BLEND, (-tx, ty), (-bx, WAIST)),
        arc((-LOBE_X, 0), LOBE_RADIUS, (-tx, ty), (-tx, -ty)),
        arc((-bx, -lift), BLEND, (-bx, -WAIST), (-tx, -ty)),
    ]
    wire = BRepBuilderAPI_MakeWire()
    for edge in edges:
        wire.Add(edge)
    if not wire.IsDone():
        raise Refused("BRepBuilderAPI_MakeWire could not close the outline")
    outline = BRepBuilderAPI_MakeFace(wire.Wire()).Face()
    plate = BRepPrimAPI_MakePrism(outline, gp_Vec(0, 0, THICKNESS)).Shape()
    boss = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(LOBE_X, 0, 0), gp.DZ_s()), LOBE_RADIUS,
                                    BOSS_TOP).Shape()
    fuse = BRepAlgoAPI_Fuse(plate, boss)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the boss to the plate")
    return fuse.Shape()
