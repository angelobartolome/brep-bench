"""A plate with the rim of its elliptic hole chamfered. The plate of `fillet/elliptic-hole-rim`: from
(−15, −10, 0) to (15, 10, 4), with a hole right through it on the ellipse x²/6² + y²/3² = 1 (semi-
axes 6 along x and 3 along y). Chamfer the hole's rim on the top face, the ellipse where the hole's
wall meets the top at z = 4, by 1 on both faces: 1 out from the rim across the top, square to it,
and 1 down the wall. The rim on the bottom face stays sharp.

In each plane square to the rim the chamfer's section is the straight line from the point 1 down
the wall to the point 1 out across the top, at 45° to both. So the chamfer is the ruled surface
between the ellipse at z = 3 on the wall and the ellipse's outer parallel curve at distance 1 on
the top, each ruling square to the rim: a countersink of constant slope 45°, developable, and not a
cone. The hole's mouth on the top is the region inside that parallel curve (not an ellipse)."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.GeomAbs import GeomAbs_CurveType
from OCP.gp import gp_Ax2, gp_Dir, gp_Elips, gp_Pnt

from bench import Refused
from rims import chamfered, edges_at_height, prism

MAJOR, MINOR, THICKNESS = 6, 3, 4
DISTANCE = 1


def build():
    plate = BRepPrimAPI_MakeBox(gp_Pnt(-15, -10, 0), gp_Pnt(15, 10, THICKNESS)).Shape()
    # the hole's cutter runs past both faces
    ellipse = gp_Elips(gp_Ax2(gp_Pnt(0, 0, -1), gp_Dir(0, 0, 1), gp_Dir(1, 0, 0)), MAJOR, MINOR)
    cut = BRepAlgoAPI_Cut(plate, prism([BRepBuilderAPI_MakeEdge(ellipse).Edge()], THICKNESS + 2))
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not cut the hole")
    drilled = cut.Shape()
    # the top's edges but the plate's four straight ones: the hole's rim
    rim = [e for e in edges_at_height(drilled, THICKNESS)
           if BRepAdaptor_Curve(e).GetType() != GeomAbs_CurveType.GeomAbs_Line]
    return chamfered(drilled, DISTANCE, rim, "the hole's elliptic rim")
