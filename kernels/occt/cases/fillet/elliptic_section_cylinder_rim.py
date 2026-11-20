"""A cylinder's rim rounded unequally, by a quarter ellipse. A cylinder of radius 5 on the z axis,
from z = 0 to z = 10. Round the circle where its side meets its top unequally: 1 in across the top
and 3 down the side, with a section that is a quarter ellipse.

In each half plane through the z axis the round's section is the quarter of the ellipse centered at
distance 4 from the axis and height 7, with semi-axis 1 across (along the radius) and 3 up, that
runs from the top at distance 4 from the axis, where it is tangent to the top, to the side at
height 7, where it is tangent to the side: the points (ρ, z) = (4 + cos θ, 7 + 3 sin θ), 0 ≤ θ ≤
90°, where ρ is the distance from the axis. The round is that quarter ellipse turned round the z
axis, and the corner outside it is removed. The top is left the disk of radius 4 and the side ends
at z = 7.

(An asymmetric round: the quarter ellipse is what a round of 1 on one face and 3 on the other makes
square to a straight edge between faces at 90°. Here the edge is a circle.)"""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge
from OCP.gp import gp_Ax2, gp_Dir, gp_Elips, gp_Pnt

from rims import segment, turned

RADIUS, HEIGHT = 5, 10
ACROSS, DOWN = 1, 3


def build():
    # Workaround: OCCT's fillets have a circular section and it has no asymmetric (elliptic)
    # round. The rounded cylinder is a solid of revolution, so it is its half section turned round
    # the z axis: the quarter ellipse centred at (4, 7) as (x, z), semi-axes 1 along x and 3 along
    # z, closed by the axis, the bottom, the side and the top.
    centre = gp_Pnt(RADIUS - ACROSS, 0, HEIGHT - DOWN)
    # The major axis along z and the frame's normal −y, so its other axis is −y × z = −x: the
    # point at parameter t is centre + 3 cos t·z − sin t·x, the top (4, 10) at t = 0 and the
    # side (5, 7) at t = −π/2.
    frame = gp_Ax2(centre, gp_Dir(0, -1, 0), gp_Dir(0, 0, 1))
    ellipse = gp_Elips(frame, DOWN, ACROSS)
    quarter = BRepBuilderAPI_MakeEdge(ellipse, -math.pi / 2, 0).Edge()
    return turned([
        quarter,
        segment((RADIUS - ACROSS, 0, HEIGHT), (0, 0, HEIGHT)),
        segment((0, 0, HEIGHT), (0, 0, 0)),
        segment((0, 0, 0), (RADIUS, 0, 0)),
        segment((RADIUS, 0, 0), (RADIUS, 0, HEIGHT - DOWN)),
    ])
