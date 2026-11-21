"""A block's top rim, round a lobe and into a reverse arc. A block 80 by 40, from (0, 0, 0) to
(80, 40, 10), its four upright corners rounded to radius 5, with a lobe on its south side: the
south side runs east along y = 0 to x = 30, bends down through a quarter of the circle of radius 5
about (30, −5) (concave) to (35, −5), round the lobe, half the circle of radius 10 about (45, −5)
(convex), to (55, −5), runs straight north to (55, 0), bends through a quarter of the circle of
radius 5 about (60, 0) (concave) to (60, 5), and runs on east along y = 5 to the corner. The
outline is one tangent chain of thirteen pieces; the block is the prism over it from z = 0 to
z = 10. Round the rim of its top face to radius 2, all of it, as one run."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused
from rims import edges_at_height, prism

HEIGHT = 10
ROUND = 2
# The outline, counter-clockwise from the south-west corner: a straight piece is ("line", from, to),
# an arc ("arc", centre, radius, from angle, to angle), anticlockwise where the angle grows.
OUTLINE = [
    ("line", (5, 0), (30, 0)),
    ("arc", (30, -5), 5, 90, 0),            # concave, into the lobe
    ("arc", (45, -5), 10, 180, 360),        # the lobe
    ("line", (55, -5), (55, 0)),            # the short straight piece
    ("arc", (60, 0), 5, 180, 90),           # concave, the reverse arc
    ("line", (60, 5), (75, 5)),
    ("arc", (75, 10), 5, -90, 0),
    ("line", (80, 10), (80, 35)),
    ("arc", (75, 35), 5, 0, 90),
    ("line", (75, 40), (5, 40)),
    ("arc", (5, 35), 5, 90, 180),
    ("line", (0, 35), (0, 5)),
    ("arc", (5, 5), 5, 180, 270),
]


def edge(piece):
    if piece[0] == "line":
        _, (ax, ay), (bx, by) = piece
        return BRepBuilderAPI_MakeEdge(gp_Pnt(ax, ay, 0), gp_Pnt(bx, by, 0)).Edge()
    _, (cx, cy), radius, a, b = piece
    circle = gp_Circ(gp_Ax2(gp_Pnt(cx, cy, 0), gp_Dir(0, 0, 1)), radius)
    return BRepBuilderAPI_MakeEdge(circle, math.radians(min(a, b)), math.radians(max(a, b))).Edge()


def build():
    block = prism([edge(piece) for piece in OUTLINE], HEIGHT)
    rim = BRepFilletAPI_MakeFillet(block)
    for top in edges_at_height(block, HEIGHT):
        rim.Add(ROUND, top)
    assert rim.NbContours() == 1, f"{rim.NbContours()} chains round the top"
    rim.Build()
    if not rim.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the top rim")
    return rim.Shape()
