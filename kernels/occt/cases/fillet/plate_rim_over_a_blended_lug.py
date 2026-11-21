"""A plate's bottom rim, round a lug's blends. A plate from (−40, −25, 0) to (40, 25, 13), its four
upright edges rounded to radius 10, and a round lug, the cylinder of radius 9 about the upright
through (0, 25), the midpoint of the plate's north side, from z = 0 to z = 13, joined to it. Round
the two concave upright edges where the lug's side meets the plate's north side, at (±9, 25), to
radius 8: the outline is then one tangent chain. Then round the rim of the bottom of the blended
plate, every edge in z = 0, to radius 2, as one run."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.ShapeUpgrade import ShapeUpgrade_UnifySameDomain
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused
from blend import edges_where, rounded
from rims import edges_at_height

HALF_LENGTH, HALF_WIDTH, THICKNESS = 40, 25, 13
CORNER = 10
LUG = 9
BLEND = 8
RIM = 2
TOLERANCE = 1e-7


def uprights(solid, places):
    """The upright edges of `solid` standing over the points `places`, (x, y) each, by their
    middles, halfway up."""
    return edges_where(solid, lambda p: abs(p.Z() - THICKNESS / 2) < TOLERANCE and any(
        abs(p.X() - x) < TOLERANCE and abs(p.Y() - y) < TOLERANCE for x, y in places))


def build():
    plate = BRepPrimAPI_MakeBox(gp_Pnt(-HALF_LENGTH, -HALF_WIDTH, 0),
                                gp_Pnt(HALF_LENGTH, HALF_WIDTH, THICKNESS)).Shape()
    corners = [(x, y) for x in (-HALF_LENGTH, HALF_LENGTH) for y in (-HALF_WIDTH, HALF_WIDTH)]
    plate = rounded(plate, CORNER, uprights(plate, corners), "the plate's corners")
    lug = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, HALF_WIDTH, 0), gp_Dir(0, 0, 1)),
                                   LUG, THICKNESS).Shape()
    fuse = BRepAlgoAPI_Fuse(plate, lug)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the lug")
    # one top and one bottom face, not the plate's and the lug's side by side
    unify = ShapeUpgrade_UnifySameDomain(fuse.Shape(), True, True, False)
    unify.Build()
    part = unify.Shape()
    # the lug's side meets the north side in two uprights, at (±9, 25)
    roots = uprights(part, [(-LUG, HALF_WIDTH), (LUG, HALF_WIDTH)])
    assert len(roots) == 2, f"{len(roots)} uprights where the lug meets the plate"
    blended = rounded(part, BLEND, roots, "the uprights where the lug meets the plate")
    rim = BRepFilletAPI_MakeFillet(blended)
    for edge in edges_at_height(blended, 0):
        rim.Add(RIM, edge)
    assert rim.NbContours() == 1, f"{rim.NbContours()} chains round the bottom"
    rim.Build()
    if not rim.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the bottom rim")
    return rim.Shape()
