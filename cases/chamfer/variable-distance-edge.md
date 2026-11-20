+++
title = "A cube edge chamfered with one distance growing along it"
tags = ["chamfer", "variable-distance", "asymmetric", "ruled", "hyperbolic-paraboloid"]

[volume]
value = 980.0
exact = "1000 − ∫₀¹⁰ d₁d₂/2 dz, d₁ = 1 + z/5, d₂ = 2: 1000 − 20"
tex = '1000 - \int_0^{10}\tfrac{d_1 d_2}{2}\,dz = 1000 - 20,\quad d_1 = 1 + \tfrac{z}{5},\ d_2 = 2'

[area]
value = 584.6818133753055
exact = "600 − ∫d₁dz − ∫d₂dz − (1·2 + 3·2)/2 + ∫₀¹⁰∫₀¹√(d₁² + d₂² + d₂²d₁′²(1 − s)²)ds dz = 556 + 28.681813375305474 (the chamfer the ruled surface (1 − s)(10, 10 − d₁, z) + s(10 − d₂, 10, z))"
tex = '556 + \int_0^{10}\!\!\int_0^1\sqrt{d_1^2 + d_2^2 + d_2^2 d_1^{\prime 2}(1 - s)^2}\,ds\,dz = 556 + 28.681813375305474'
+++

A cube of side 10 between the origin and (10, 10, 10). Chamfer the edge where its faces x = 10 and
y = 10 meet, the line from (10, 10, 0) to (10, 10, 10), with a distance on the face x = 10 that
grows linearly along the edge, d₁ = 1 + z/5, from 1 at the bottom to 3 at the top, and a constant
distance d₂ = 2 on the face y = 10.

The chamfer's sections square to the edge are straight: at each height z its section by the
horizontal plane is the line from (10, 10 − d₁, z) on the face x = 10 to (10 − d₂, 10, z) on the
face y = 10. Its two edges on those faces are the straight lines from (10, 9, 0) to (10, 7, 10) and
from (8, 10, 0) to (8, 10, 10), which are skew, so the chamfer is not a plane: it is the hyperbolic
paraboloid ruled by the horizontal lines between them. It cuts the bottom face with the line from
(10, 9, 0) to (8, 10, 0) and the top with the line from (10, 7, 10) to (8, 10, 10).

**As a set:**

$$[0, 10]^3 \cap \Bigl\{\, \frac{10 - y}{d_1} + \frac{10 - x}{d_2} \ge 1 \,\Bigr\},\quad d_1 = 1 + z/5,\ d_2 = 2$$
