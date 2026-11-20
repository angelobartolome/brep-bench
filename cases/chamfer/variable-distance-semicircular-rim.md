+++
title = "A half cylinder's curved rim chamfered by a distance growing round it"
tags = ["chamfer", "variable-distance", "cylinder", "ruled", "curved-edge", "helix"]

[volume]
value = 219.25698728178764
exact = "75π − ∫_{−π/2}^{π/2}(5d²/2 − d³/6)dφ, d = 3/2 + φ/π: 75π − 35π/6 + 5π/8 (each section's area d²/2 by Pappus, its moment about the side d³/6)"
tex = '75\pi - \int_{-\pi/2}^{\pi/2}\left(\tfrac{5d^2}{2} - \tfrac{d^3}{6}\right)d\varphi = 75\pi - \tfrac{35\pi}{6} + \tfrac{5\pi}{8},\quad d = \tfrac{3}{2} + \tfrac{\varphi}{\pi}'

[area]
value = 215.00765053698686
exact = "60 − 5/2 + ½∫(5 − d)²dφ + ∫5(6 − d)dφ + 25π/2 + A_chamfer, d = 3/2 + φ/π, φ from −π/2 to π/2; A_chamfer = ∫∫|∂S/∂φ × ∂S/∂s| ds dφ over S(φ, s) = ((5 − d(1 − s))cos φ, (5 − d(1 − s))sin φ, 6 − ds), 0 ≤ s ≤ 1: A_chamfer = 28.178752964207046"
tex = '\begin{array}{l} 60 - \tfrac{5}{2} + \tfrac12\displaystyle\int(5 - d)^2 d\varphi + \int 5(6 - d)\,d\varphi + \tfrac{25\pi}{2} + A_\text{chamfer} \\ A_\text{chamfer} = \displaystyle\iint\Bigl|\tfrac{\partial S}{\partial\varphi} \times \tfrac{\partial S}{\partial s}\Bigr|\,ds\,d\varphi = 28.178752964207046 \end{array}'
+++

The half cylinder of `fillet/variable-radius-semicircular-rim`: the half of the cylinder of radius
5 on the z axis, from z = 0 to z = 6, on the side x ≥ 0, its flat face in the plane x = 0.

Chamfer the half circle where its curved side meets its top, the edge from (0, −5, 6) round
through (5, 0, 6) to (0, 5, 6), by the same distance d on both faces, d growing linearly with the
angle φ round the z axis (from +x toward +y): d = 3/2 + φ/π, so 1 where the edge starts at
φ = −90°, 1.5 at (5, 0, 6), and 2 where it ends at φ = 90°.

In the half plane through the z axis at angle φ the chamfer's section is the straight line from the
point d(φ) in across the top, at distance 5 − d from the axis, to the point d(φ) down the side, at
height 6 − d. The chamfer meets the top along an arc of a spiral of Archimedes, ρ = 5 − d(φ), and
the side along a helix, z = 6 − d(φ), and is the ruled surface between them, each ruling in its
half plane: not a cone. Its ends are its sections in the plane x = 0, right triangles of legs 1 at
y = −5 and 2 at y = 5, cut from the flat face.

**As a set:**

$$\{\, x \ge 0,\ \rho \le 5,\ 0 \le z \le 6,\ \rho + z \le 11 - d \,\},\quad d = \tfrac{3}{2} + \tfrac{\varphi}{\pi},\ -\tfrac{\pi}{2} \le \varphi \le \tfrac{\pi}{2}$$
