"""An elliptic cylinder with its top rim rounded. An elliptic cylinder from z = 0 to z = 8 on the
ellipse x²/10² + y²/6² = 1 (semi-axes 10 along x and 6 along y). Round the ellipse where its side
meets its top, at z = 8, to radius 2.

The round is a pipe, not a torus: the surface a ball of radius 2 sweeps as it rolls round the
inside of the rim touching the top and the side, its center running round the ellipse's inner
parallel curve at distance 2, at height 6. In each plane square to the rim its section is the
quarter circle of radius 2 tangent to the side 2 below the top and to the top 2 in from the rim.
The ellipse's least radius of curvature, 6²/10 = 3.6 at the ends of its major axis, is more than
the round's, so the parallel curve is smooth and the round has no fold.

The top is left the region inside the ellipse's parallel curve 2 in (not an ellipse), the side
everything below z = 6."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge
from OCP.gp import gp_Ax2, gp_Dir, gp_Elips, gp_Pnt

from rims import edges_at_height, prism, rounded

MAJOR, MINOR, HEIGHT = 10, 6, 8
RADIUS = 2


def build():
    ellipse = gp_Elips(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1), gp_Dir(1, 0, 0)), MAJOR, MINOR)
    cylinder = prism([BRepBuilderAPI_MakeEdge(ellipse).Edge()], HEIGHT)
    return rounded(cylinder, RADIUS, edges_at_height(cylinder, HEIGHT), "the elliptic rim")
