"""A cube edge chamfered with one distance growing along it. A cube of side 10 between the origin and
(10, 10, 10). Chamfer the edge where its faces x = 10 and y = 10 meet, the line from (10, 10, 0) to
(10, 10, 10), with a distance on the face x = 10 that grows linearly along the edge, d₁ = 1 + z/5,
from 1 at the bottom to 3 at the top, and a constant distance d₂ = 2 on the face y = 10.

The chamfer's sections square to the edge are straight: at each height z its section by the
horizontal plane is the line from (10, 10 − d₁, z) on the face x = 10 to (10 − d₂, 10, z) on the
face y = 10. Its two edges on those faces are the straight lines from (10, 9, 0) to (10, 7, 10) and
from (8, 10, 0) to (8, 10, 10), which are skew, so the chamfer is not a plane: it is the hyperbolic
paraboloid ruled by the horizontal lines between them. It cuts the bottom face with the line from
(10, 9, 0) to (8, 10, 0) and the top with the line from (10, 7, 10) to (8, 10, 10)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakePolygon
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused

SIDE = 10


def d1(z):
    """The distance on the face x = 10, growing along the edge."""
    return 1 + z / 5


D2 = 2  # the distance on the face y = 10


def corner(z):
    """The triangle in the plane at height z cut off by the chamfer, run on past both faces: its
    chamfer side the line from (10, 10 − d₁) to (10 − d₂, 10) extended half its length beyond
    each end, closed round the corner outside the cube."""
    a = (SIDE, SIDE - d1(z))
    b = (SIDE - D2, SIDE)
    a_out = (a[0] + (a[0] - b[0]) / 2, a[1] + (a[1] - b[1]) / 2)
    b_out = (b[0] + (b[0] - a[0]) / 2, b[1] + (b[1] - a[1]) / 2)
    far = (SIDE + 3, SIDE + 3)
    polygon = BRepBuilderAPI_MakePolygon()
    for x, y in (a_out, far, b_out):
        polygon.Add(gp_Pnt(x, y, z))
    polygon.Close()
    return polygon.Wire()


def build():
    # Workaround: BRepFilletAPI_MakeChamfer has no chamfer whose distance varies along its edge.
    # The chamfer is the surface ruled by the horizontal lines between (10, 10 − d₁, z) and
    # (10 − d₂, 10, z), and both run linearly in z, so it is the ruled loft between its lines at
    # two heights: the corner beyond it is the ruled loft of the triangles above, from below the
    # cube to above it (every line between two such sections lies on the same hyperbolic
    # paraboloid), cut from the cube.
    loft = BRepOffsetAPI_ThruSections(True, True)
    loft.AddWire(corner(-1))
    loft.AddWire(corner(SIDE + 1))
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft the corner")
    cut = BRepAlgoAPI_Cut(BRepPrimAPI_MakeBox(SIDE, SIDE, SIDE).Shape(), loft.Shape())
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not cut the corner away")
    return cut.Shape()
