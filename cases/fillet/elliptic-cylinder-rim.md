+++
title = "An elliptic cylinder with its top rim rounded"
tags = ["fillet", "ellipse", "pipe", "curved-edge"]

[volume]
value = 1466.5488314543154
exact = "480π − [(1 − π/4)r²L − 2π(5/6 − π/4)r³], r = 2, L = 40E(e²), e² = 0.64, the ellipse's perimeter: 480π − 4(1 − π/4)L + 16π(5/6 − π/4), L = 51.053997726796254 (along a closed curve each section's area counts at the rate its points' parallel curve runs, L − 2πs at s in from the side)"
tex = '480\pi - \Bigl[\left(1 - \tfrac{\pi}{4}\right)r^2 L - 2\pi\left(\tfrac{5}{6} - \tfrac{\pi}{4}\right)r^3\Bigr],\quad r = 2,\ L = 40\,E(0.64) = 51.053997726796254'

[area]
value = 739.8186677715735
exact = "120π + 8L + (π/2 − 2)rL + (3 − π)πr², r = 2, L = 40E(0.64) = 51.053997726796254: the top loses rL − πr², the side rL, and the round is ∫₀^{π/2}(L − 2πr(1 − cos θ))r dθ"
tex = '120\pi + 8L + \left(\tfrac{\pi}{2} - 2\right)rL + (3 - \pi)\pi r^2,\quad r = 2,\ L = 40\,E(0.64) = 51.053997726796254'
+++

An elliptic cylinder from z = 0 to z = 8 on the ellipse x²/10² + y²/6² = 1 (semi-axes 10 along x
and 6 along y). Round the ellipse where its side meets its top, at z = 8, to radius 2.

The round is a pipe, not a torus: the surface a ball of radius 2 sweeps as it rolls round the
inside of the rim touching the top and the side, its center running round the ellipse's inner
parallel curve at distance 2, at height 6. In each plane square to the rim its section is the
quarter circle of radius 2 tangent to the side 2 below the top and to the top 2 in from the rim.
The ellipse's least radius of curvature, 6²/10 = 3.6 at the ends of its major axis, is more than
the round's, so the parallel curve is smooth and the round has no fold.

The top is left the region inside the ellipse's parallel curve 2 in (not an ellipse), the side
everything below z = 6.

**As a set:**

$$\{\, x^2/10^2 + y^2/6^2 \le 1,\ 0 \le z \le 8 \,\} \setminus \bigl\{\, s < 2,\ z > 6,\ (2 - s)^2 + (z - 6)^2 > 4 \,\bigr\}$$

where s is the point's distance, in its horizontal plane, from the ellipse.
