"""An elliptic boss with its root rounded. A plate from (−20, −15, 0) to (20, 15, 4), and an elliptic
boss on it: the elliptic cylinder on x²/10² + y²/6² = 1 (semi-axes 10 along x and 6 along y)
standing on the plate's top, from z = 4 to z = 10, joined to the plate into one solid. Round the
boss's root, the ellipse where its side meets the plate's top at z = 4, to radius 2. The round
fills the inside corner, adding material; the boss's top rim stays sharp.

The round is a pipe, not a torus: the surface a ball of radius 2 sweeps as it rolls round the boss
in the corner, touching the boss's side and the plate's top, its center running round the ellipse's
outer parallel curve at distance 2, at height 6. In each plane square to the root its section is
the quarter circle of radius 2 tangent to the side 2 above the plate and to the plate 2 out from
the boss, and everything between that arc and the corner is added. The plate's top is left
everything outside the ellipse's outer parallel curve 2 out (not an ellipse), and the boss's side
runs down to z = 6.

It is the concave counterpart of `fillet/elliptic-cylinder-rim`."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakePrism
from OCP.GeomAbs import GeomAbs_CurveType
from OCP.gp import gp_Ax2, gp_Dir, gp_Elips, gp_Pnt

from bench import Refused
from rims import edges_at_height, prism, rounded

MAJOR, MINOR = 10, 6
PLATE, TOP = 4, 10
RADIUS = 2


def build():
    plate = BRepPrimAPI_MakeBox(gp_Pnt(-20, -15, 0), gp_Pnt(20, 15, PLATE)).Shape()
    # the boss from inside the plate up to its top, so the two overlap rather than touch
    ellipse = gp_Elips(gp_Ax2(gp_Pnt(0, 0, 1), gp_Dir(0, 0, 1), gp_Dir(1, 0, 0)), MAJOR, MINOR)
    boss = prism([BRepBuilderAPI_MakeEdge(ellipse).Edge()], TOP - 1)
    fuse = BRepAlgoAPI_Fuse(plate, boss)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the boss to the plate")
    joined = fuse.Shape()
    # the plate's top's edges but its four straight ones: the boss's root
    root = [e for e in edges_at_height(joined, PLATE)
            if BRepAdaptor_Curve(e).GetType() != GeomAbs_CurveType.GeomAbs_Line]
    return rounded(joined, RADIUS, root, "the boss's elliptic root")
