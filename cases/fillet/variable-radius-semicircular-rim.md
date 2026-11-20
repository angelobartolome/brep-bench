+++
title = "A half cylinder's curved rim rounded with a radius growing round it"
tags = ["fillet", "variable-radius", "cylinder", "curved-edge"]

[volume]
value = 228.31860381364436
exact = "75π − ∫_{−π/2}^{π/2}[5(1 − π/4)r² − (5/6 − π/4)r³]dφ, r = 3/2 + φ/π: 75π − (35π/3)(1 − π/4) + (15π/4)(5/6 − π/4) (each section's area (1 − π/4)r² by Pappus, its moment about the side (5/6 − π/4)r³)"
tex = '75\pi - \int_{-\pi/2}^{\pi/2}\Bigl[5\left(1 - \tfrac{\pi}{4}\right)r^2 - \left(\tfrac{5}{6} - \tfrac{\pi}{4}\right)r^3\Bigr]d\varphi = 75\pi - \tfrac{35\pi}{3}\left(1 - \tfrac{\pi}{4}\right) + \tfrac{15\pi}{4}\left(\tfrac{5}{6} - \tfrac{\pi}{4}\right),\quad r = \tfrac{3}{2} + \tfrac{\varphi}{\pi}'

[area]
value = 221.09030860032684
exact = "60 − 5(1 − π/4) + ½∫(5 − r)²dφ + ∫5(6 − r)dφ + 25π/2 + A_round, r = 3/2 + φ/π, φ from −π/2 to π/2; A_round = ∫∫|∂S/∂φ × ∂S/∂θ| dθ dφ over S(φ, θ) = ((5 − r + r cos θ)cos φ, (5 − r + r cos θ)sin φ, 6 − r + r sin θ), 0 ≤ θ ≤ π/2: A_round = 32.834420210559784"
tex = '\begin{array}{l} 60 - 5\left(1 - \tfrac{\pi}{4}\right) + \tfrac12\displaystyle\int(5 - r)^2 d\varphi + \int 5(6 - r)\,d\varphi + \tfrac{25\pi}{2} + A_\text{round} \\ A_\text{round} = \displaystyle\iint\Bigl|\tfrac{\partial S}{\partial\varphi} \times \tfrac{\partial S}{\partial\theta}\Bigr|\,d\theta\,d\varphi = 32.834420210559784 \end{array}'
+++

A half cylinder: the half of the cylinder of radius 5 on the z axis, from z = 0 to z = 6, on the
side x ≥ 0. Its flat face is the rectangle in the plane x = 0 from (0, −5, 0) to (0, 5, 6), and
its top is the half disk of radius 5 at z = 6.

Round the half circle where its curved side meets its top, the edge from (0, −5, 6) round through
(5, 0, 6) to (0, 5, 6), with a radius that grows linearly with the angle φ round the z axis (from
+x toward +y): r = 3/2 + φ/π, so 1 where the edge starts at φ = −90°, 1.5 at (5, 0, 6), and 2
where it ends at φ = 90°.

The round's sections square to the edge are circular: in the half plane through the z axis at
angle φ its section is the quarter circle of radius r(φ) whose center is r in from the side and r
down from the top, at distance 5 − r from the axis and height 6 − r, tangent to the top 5 − r from
the axis and to the side at height 6 − r. The round's two ends are its sections in the plane
x = 0, of radius 1 at y = −5 and 2 at y = 5, so it ends square on the flat face, which loses the
region between each section and its corner, of area (1 − π/4)r². The top is left the region
inside the curve ρ = 5 − r(φ), an arc of a spiral of Archimedes, and the side the region below the
helix z = 6 − r(φ).
