+++
title = "An elliptic boss with its root rounded"
tags = ["fillet", "ellipse", "pipe", "curved-edge", "concave"]

[volume]
value = 5977.207966448124
exact = "4800 + 360π + (1 − π/4)r²L + 2π(5/6 − π/4)r³, r = 2, L = 40E(e²), e² = 0.64, the ellipse's perimeter, L = 51.053997726796254 (round the boss each section's area counts at the rate its points' parallel curve runs, L + 2πs at s out from the boss)"
tex = '4800 + 360\pi + \left(1 - \tfrac{\pi}{4}\right)r^2 L + 2\pi\left(\tfrac{5}{6} - \tfrac{\pi}{4}\right)r^3,\quad r = 2,\ L = 40\,E(0.64) = 51.053997726796254'

[area]
value = 3224.2781654097657
exact = "2960 + 6L + (π/2 − 2)rL + (π − 3)πr², r = 2, L = 40E(0.64) = 51.053997726796254: the plate's top loses rL + πr² more, the boss's side rL, and the round is ∫₀^{π/2}(L + 2πr(1 − cos θ))r dθ"
tex = '2960 + 6L + \left(\tfrac{\pi}{2} - 2\right)rL + (\pi - 3)\pi r^2,\quad r = 2,\ L = 40\,E(0.64) = 51.053997726796254'
+++

A plate from (−20, −15, 0) to (20, 15, 4), and an elliptic boss on it: the elliptic cylinder on
x²/10² + y²/6² = 1 (semi-axes 10 along x and 6 along y) standing on the plate's top, from z = 4
to z = 10, joined to the plate into one solid. Round the boss's root, the ellipse where its side
meets the plate's top at z = 4, to radius 2. The round fills the inside corner, adding material;
the boss's top rim stays sharp.

The round is a pipe, not a torus: the surface a ball of radius 2 sweeps as it rolls round the
boss in the corner, touching the boss's side and the plate's top, its center running round the
ellipse's outer parallel curve at distance 2, at height 6. In each plane square to the root its
section is the quarter circle of radius 2 tangent to the side 2 above the plate and to the plate 2
out from the boss, and everything between that arc and the corner is added. The plate's top is
left everything outside the ellipse's outer parallel curve 2 out (not an ellipse), and the boss's
side runs down to z = 6.

It is the concave counterpart of `fillet/elliptic-cylinder-rim`.

**As a set:**

$$P \cup E \cup \bigl\{\, 0 < s < 2,\ 4 \le z < 6,\ (2 - s)^2 + (6 - z)^2 > 4 \,\bigr\}$$

where P is the plate, E the boss, and s the point's distance, in its horizontal plane, from the
ellipse, taken outside it.
