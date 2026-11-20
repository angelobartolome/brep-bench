+++
title = "A plate with the rim of its elliptic hole rounded"
tags = ["fillet", "ellipse", "pipe", "curved-edge", "hole"]

[volume]
value = 2167.266667040222
exact = "2400 − 72π − [(1 − π/4)r²L + 2π(5/6 − π/4)r³], r = 1, L = 24E(e²), e² = 0.75, the ellipse's perimeter, L = 29.065344661643028 (round a hole each section's area counts at the rate its points' parallel curve runs, L + 2πs at s out from the hole)"
tex = '2400 - 72\pi - \Bigl[\left(1 - \tfrac{\pi}{4}\right)r^2 L + 2\pi\left(\tfrac{5}{6} - \tfrac{\pi}{4}\right)r^3\Bigr],\quad r = 1,\ L = 24\,E(0.75) = 29.065344661643028'

[area]
value = 1591.13391686591
exact = "1600 − 36π + 4L + (π/2 − 2)rL + (π − 3)πr², r = 1, L = 24E(0.75) = 29.065344661643028: the top loses rL + πr², the hole's wall rL, and the round is ∫₀^{π/2}(L + 2πr(1 − cos θ))r dθ"
tex = '1600 - 36\pi + 4L + \left(\tfrac{\pi}{2} - 2\right)rL + (\pi - 3)\pi r^2,\quad r = 1,\ L = 24\,E(0.75) = 29.065344661643028'
+++

A plate from (−15, −10, 0) to (15, 10, 4) with a hole right through it on the ellipse
x²/6² + y²/3² = 1 (semi-axes 6 along x and 3 along y). Round the hole's rim on the top face, the
ellipse where the hole's wall meets the top at z = 4, to radius 1. The rim on the bottom face
stays sharp.

The round is a pipe, not a torus: the surface a ball of radius 1 sweeps as it rolls round the rim
touching the top and the hole's wall, its center running round the ellipse's outer parallel curve
at distance 1, at height 3. In each plane square to the rim its section is the quarter circle of
radius 1 tangent to the wall 1 below the top and to the top 1 out from the rim. The hole's mouth
on the top is the region inside that parallel curve (not an ellipse).

**As a set:**

$$\bigl([-15, 15] \times [-10, 10] \times [0, 4]\bigr) \setminus \bigl(\{\, x^2/6^2 + y^2/3^2 < 1 \,\} \cup \{\, s < 1,\ z > 3,\ (1 - s)^2 + (z - 3)^2 > 1 \,\}\bigr)$$

where s is the point's distance, in its horizontal plane, from the ellipse.
