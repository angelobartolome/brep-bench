+++
title = "A hole through a post standing across a plate's edge, both rims chamfered"
tags = ["chamfer", "boolean", "union", "cylinder", "planar", "coincident"]
origin = "a geomsolver refusal, bmander/geomsolver#994"

[volume]
value = 213646.48575052764
exact = "L²T + πR²(H − T) + S·T − πr²H − 2π(r + c/3)c² (Pappus), S = R²·arccos(e/R) − e√(R² − e²) the post's foot beyond the plate, L = 100, T = 20, R = 15, e = 5, H = 40, r = 6, c = 1.5"
tex = 'L^2T + \pi R^2(H - T) + ST - \pi r^2 H - 2\pi\left(r + \tfrac{c}{3}\right)c^2,\quad S = R^2\arccos\tfrac{e}{R} - e\sqrt{R^2 - e^2}'

[area]
value = 31691.730922835573
exact = "2L² + 2S + 4LT − 2T√(R² − e²) + 2πR(H − T) + 2RT·arccos(e/R) − 2π(r + c)² + 2πr(H − 2c) + 2·2π(r + c/2)·c√2 (Pappus), S as above"
tex = '2L^2 + 2S + 4LT - 2T\sqrt{R^2 - e^2} + 2\pi R(H - T) + 2RT\arccos\tfrac{e}{R} - 2\pi(r + c)^2 + 2\pi r(H - 2c) + 2\cdot 2\pi\left(r + \tfrac{c}{2}\right)c\sqrt{2}'
+++

A square plate with a round post standing across its edge, drilled through the post, both rims of
the hole chamfered.

The plate is the box from (−50, −50, 0) to (50, 50, 20). The post is the solid cylinder of radius
15 about the vertical line through (45, 0), from z = 0 to z = 40: its axis is 5 inside the plate's
side x = 50, so most of it rises through the plate and a segment 10 wide overhangs it, the
overhang's foot flush with the plate's bottom. The plate and the post are one solid, their union.
Its bottom face z = 0 is one plane: the plate's square and the overhanging part of the post's
disk.

A hole of radius 6 is drilled right through along the post's axis, from the bottom face z = 0 to
the post's top z = 40. It lies within the post everywhere (it reaches x = 51, the post x = 60). On
the bottom face its circle crosses the line x = 50 along which the plate's bottom and the post's
overhanging foot meet in the one plane.

Both rims of the hole, where it meets the bottom face and where it meets the post's top, are
chamfered 1.5 along each face, at 45°. Each chamfer is the cone through the circle of radius 7.5
on its flat face and the circle 1.5 along the hole's wall from it; it removes a ring whose section
is the right triangle with two legs of 1.5. The lower chamfer runs all the way round, across the
line x = 50, unbroken: on the bottom face there is no edge there.

**As a set:**

$$\Bigl(\bigl([-50,50]^2\times[0,20]\bigr) \cup \{\,\rho' \le 15,\ 0 \le z \le 40\,\}\Bigr)
\setminus \{\,\rho' \le 6 + \max(0,\ 1.5 - z,\ z - 38.5)\,\},\quad \rho' = \sqrt{(x-45)^2 + y^2}$$
