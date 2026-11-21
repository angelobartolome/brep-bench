"""A lobed plate with both rims rounded. The plate's outline is six convex lobes and six concave
arcs, alternating, each running on tangent into the next: the lobes are arcs of the circles of
radius 8 about the points 40 from the z axis at 0°, 60°, …, 300°; the concave arcs are arcs of the
circles of radius 30 about the points 20√3 + 6√29 from the axis at 30°, 90°, …, 330°, each the
circle that touches the two lobes' circles beside it from outside (its centre 38 from theirs). The
plate is the prism over that outline from z = 0 to z = 12. Round both rims, top and bottom, each a
tangent chain of twelve arcs, to radius 1."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from rims import edges_at_height, prism, rounded

LOBES, PITCH, LOBE = 6, 40, 8
HOLLOW = 30
THICKNESS = 12
ROUND = 1
# the hollow's centre: on the ray halfway between two lobes, LOBE + HOLLOW from each lobe's centre
REACH = 20 * math.sqrt(3) + 6 * math.sqrt(29)


def arc(cx, cy, radius, a, b):
    """The arc of the circle of `radius` about (cx, cy) in z = 0 from angle `a` anticlockwise to
    `b`, in radians."""
    circle = gp_Circ(gp_Ax2(gp_Pnt(cx, cy, 0), gp_Dir(0, 0, 1)), radius)
    return BRepBuilderAPI_MakeEdge(circle, a, b).Edge()


def build():
    turn = 2 * math.pi / LOBES
    # the lobe's half sweep: the bearing, from a lobe's centre, of the next hollow's centre
    half = math.atan2(REACH * math.sin(turn / 2), REACH * math.cos(turn / 2) - PITCH)
    edges = []
    for k in range(LOBES):
        t = k * turn
        edges.append(arc(PITCH * math.cos(t), PITCH * math.sin(t), LOBE, t - half, t + half))
        # the hollow from the next lobe's tangent point round to this one's, anticlockwise
        h = t + turn / 2
        edges.append(arc(REACH * math.cos(h), REACH * math.sin(h), HOLLOW,
                         t + turn - half + math.pi, t + half + math.pi))
    plate = prism(edges, THICKNESS)
    rims = edges_at_height(plate, THICKNESS) + edges_at_height(plate, 0)
    assert len(rims) == 4 * LOBES, f"{len(rims)} edges in the two rims"
    return rounded(plate, ROUND, rims, "the two rims")
