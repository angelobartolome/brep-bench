"""An elliptic cylinder with its top rim chamfered. An elliptic cylinder from z = 0 to z = 8 on the
ellipse x²/10² + y²/6² = 1 (semi-axes 10 along x and 6 along y). Chamfer the ellipse where its side
meets its top, at z = 8, by 2 on both faces: 2 in from the rim across the top, square to it, and 2
down the side.

In each plane square to the rim the chamfer's section is the straight line from the point 2 down
the side to the point 2 in across the top, at 45° to both. So the chamfer is the ruled surface
between the ellipse's inner parallel curve at distance 2 on the top and the ellipse at z = 6 on the
side, each ruling square to the rim: a surface of constant slope 45°, developable, and not a cone.
The ellipse's least radius of curvature, 3.6, is more than 2, so the parallel curve is smooth."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge
from OCP.gp import gp_Ax2, gp_Dir, gp_Elips, gp_Pnt

from rims import chamfered, edges_at_height, prism

MAJOR, MINOR, HEIGHT = 10, 6, 8
DISTANCE = 2


def build():
    ellipse = gp_Elips(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1), gp_Dir(1, 0, 0)), MAJOR, MINOR)
    cylinder = prism([BRepBuilderAPI_MakeEdge(ellipse).Edge()], HEIGHT)
    return chamfered(cylinder, DISTANCE, edges_at_height(cylinder, HEIGHT), "the elliptic rim")
