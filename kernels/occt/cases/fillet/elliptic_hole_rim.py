"""A plate with the rim of its elliptic hole rounded. A plate from (−15, −10, 0) to (15, 10, 4) with a
hole right through it on the ellipse x²/6² + y²/3² = 1 (semi-axes 6 along x and 3 along y). Round
the hole's rim on the top face, the ellipse where the hole's wall meets the top at z = 4, to radius
1. The rim on the bottom face stays sharp.

The round is a pipe, not a torus: the surface a ball of radius 1 sweeps as it rolls round the rim
touching the top and the hole's wall, its center running round the ellipse's outer parallel curve
at distance 1, at height 3. In each plane square to the rim its section is the quarter circle of
radius 1 tangent to the wall 1 below the top and to the top 1 out from the rim. The hole's mouth on
the top is the region inside that parallel curve (not an ellipse)."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.GeomAbs import GeomAbs_CurveType
from OCP.gp import gp_Ax2, gp_Dir, gp_Elips, gp_Pnt

from bench import Refused
from rims import edges_at_height, prism, rounded

MAJOR, MINOR, THICKNESS = 6, 3, 4
RADIUS = 1


def build():
    plate = BRepPrimAPI_MakeBox(gp_Pnt(-15, -10, 0), gp_Pnt(15, 10, THICKNESS)).Shape()
    # the hole's cutter runs past both faces
    ellipse = gp_Elips(gp_Ax2(gp_Pnt(0, 0, -1), gp_Dir(0, 0, 1), gp_Dir(1, 0, 0)), MAJOR, MINOR)
    cutter = prism([BRepBuilderAPI_MakeEdge(ellipse).Edge()], THICKNESS + 2)
    cut = BRepAlgoAPI_Cut(plate, cutter)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not cut the hole")
    drilled = cut.Shape()
    # the top's edges but the plate's four straight ones: the hole's rim
    rim = [e for e in edges_at_height(drilled, THICKNESS)
           if BRepAdaptor_Curve(e).GetType() != GeomAbs_CurveType.GeomAbs_Line]
    return rounded(drilled, RADIUS, rim, "the hole's elliptic rim")
