"""A half cylinder's curved rim chamfered by a distance growing round it. The half cylinder of
`fillet/variable-radius-semicircular-rim`: the half of the cylinder of radius 5 on the z axis, from
z = 0 to z = 6, on the side x ≥ 0, its flat face in the plane x = 0.

Chamfer the half circle where its curved side meets its top, the edge from (0, −5, 6) round through
(5, 0, 6) to (0, 5, 6), by the same distance d on both faces, d growing linearly with the angle φ
round the z axis (from +x toward +y): d = 3/2 + φ/π, so 1 where the edge starts at φ = −90°, 1.5 at
(5, 0, 6), and 2 where it ends at φ = 90°.

In the half plane through the z axis at angle φ the chamfer's section is the straight line from the
point d(φ) in across the top, at distance 5 − d from the axis, to the point d(φ) down the side, at
height 6 − d. The chamfer meets the top along an arc of a spiral of Archimedes, ρ = 5 − d(φ), and
the side along a helix, z = 6 − d(φ), and is the ruled surface between them, each ruling in its
half plane: not a cone. Its ends are its sections in the plane x = 0, right triangles of legs 1 at
y = −5 and 2 at y = 5, cut from the flat face."""
from bench import Refused


def build():
    raise Refused("BRepFilletAPI_MakeChamfer has no chamfer whose distance varies along its edge, "
                  "and the chamfer here, ruled between a spiral of Archimedes and a helix, is "
                  "no surface OCCT can make exactly")
