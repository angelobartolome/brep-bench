"""A half cylinder's curved rim rounded with a radius growing round it. A half cylinder: the half of
the cylinder of radius 5 on the z axis, from z = 0 to z = 6, on the side x ≥ 0. Its flat face is
the rectangle in the plane x = 0 from (0, −5, 0) to (0, 5, 6), and its top is the half disk of
radius 5 at z = 6.

Round the half circle where its curved side meets its top, the edge from (0, −5, 6) round through
(5, 0, 6) to (0, 5, 6), with a radius that grows linearly with the angle φ round the z axis (from
+x toward +y): r = 3/2 + φ/π, so 1 where the edge starts at φ = −90°, 1.5 at (5, 0, 6), and 2 where
it ends at φ = 90°.

The round's sections square to the edge are circular: in the half plane through the z axis at angle
φ its section is the quarter circle of radius r(φ) whose center is r in from the side and r down
from the top, at distance 5 − r from the axis and height 6 − r, tangent to the top 5 − r from the
axis and to the side at height 6 − r. The round's two ends are its sections in the plane x = 0, of
radius 1 at y = −5 and 2 at y = 5, so it ends square on the flat face, which loses the region
between each section and its corner, of area (1 − π/4)r². The top is left the region inside the
curve ρ = 5 − r(φ), an arc of a spiral of Archimedes, and the side the region below the helix z = 6
− r(φ)."""
from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.GeomAbs import GeomAbs_CurveType

from rims import edges_at_height, half_cylinder, rounded_variably

RADIUS, HEIGHT = 5, 6
RADIUS_START, RADIUS_END = 1, 2  # at y = −5 and y = 5


def build():
    solid = half_cylinder(RADIUS, HEIGHT)
    # The top's two edges: its diameter (a line) and the half circle.
    [rim] = [e for e in edges_at_height(solid, HEIGHT)
             if BRepAdaptor_Curve(e).GetType() == GeomAbs_CurveType.GeomAbs_Circle]
    # OCCT's law of two radii runs linearly in the spine's parameter, its angle round the circle.
    return rounded_variably(solid, rim, (lambda p: p.Y() < 0, RADIUS_START), (None, RADIUS_END),
                            "the half circle with a radius growing round it")
