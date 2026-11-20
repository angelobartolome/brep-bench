+++
title = "A hexagonal prism's edge rounded with a radius growing along it"
tags = ["fillet", "variable-radius", "obtuse", "cone"]

[volume]
value = 931.5448315358009
exact = "540√3 − (1/√3 − π/6)∫₀¹⁰(1 + 3z/10)²dz = 540√3 − 70(1/√3 − π/6)"
tex = '540\sqrt{3} - \left(\tfrac{1}{\sqrt{3}} - \tfrac{\pi}{6}\right)\int_0^{10}\left(1 + \tfrac{3z}{10}\right)^2 dz = 540\sqrt{3} - 70\left(\tfrac{1}{\sqrt{3}} - \tfrac{\pi}{6}\right)'

[area]
value = 543.4750692845354
exact = "108√3 + 360 − (2/√3)∫₀¹⁰ r dz − (1/√3 − π/6)(1² + 4²) + ∫₀¹⁰ r dz·I, r = 1 + 3z/10, r′ = 3/10: 108√3 + 360 − 50/√3 − 17(1/√3 − π/6) + 25·I, I = ∫_{−π/6}^{π/6}√(1 + (9/100)(1 − (2/√3)cos θ)²)dθ = 1.0477948367052181"
tex = '\begin{array}{l} 108\sqrt{3} + 360 - \tfrac{50}{\sqrt{3}} - 17\left(\tfrac{1}{\sqrt{3}} - \tfrac{\pi}{6}\right) + 25I \\ I = \displaystyle\int_{-\pi/6}^{\pi/6}\sqrt{1 + \tfrac{9}{100}\left(1 - \tfrac{2}{\sqrt{3}}\cos\theta\right)^2}\,d\theta = 1.0477948367052181 \end{array}'
+++

A regular hexagonal prism from z = 0 to z = 10 on the hexagon of side 6 centered on the z axis
whose corners are at (6 cos 60k°, 6 sin 60k°), k = 0, …, 5. Its sides meet at 120° inside it.
Round its edge from (6, 0, 0) to (6, 0, 10) with a radius that grows linearly along it:
r = 1 + 3z/10, so 1 at the bottom face z = 0 and 4 at the top face z = 10.

The round's sections square to the edge are circular: at each height z its section by the
horizontal plane is the arc of radius r(z) centered at (6 − 2r/√3, 0, z), on the plane y = 0 that
bisects the corner, turning through 60° between where it touches the two sides, each r/√3 from the
edge, at (6 − r/2, ±r√3/6, z). It cuts the bottom face with the arc of radius 1 and the top face
with the arc of radius 4, and the region between each arc and its corner, of area
(1/√3 − π/6)r², is gone from them. (So the round is part of an oblique circular cone with its apex
at (6, 0, −10/3), where the radius would be nought.)

An obtuse edge takes a variable round's radius back only a little: at the top the ball of radius 4
touches the sides 2.31 from the edge, and its arc passes 0.62 inside the edge.
