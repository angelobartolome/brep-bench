+++
title = "A cylinder's rim rounded unequally, by a quarter ellipse"
tags = ["fillet", "asymmetric", "ellipse", "cylinder", "curved-edge", "revolution"]

[volume]
value = 766.0759734642396
exact = "250π − 2π[5ab(1 − π/4) − a²b(5/6 − π/4)], a = 1 across the top, b = 3 down the side: the section's area ab(1 − π/4) and its moment about the side a²b(5/6 − π/4)"
tex = '250\pi - 2\pi\Bigl[5ab\left(1 - \tfrac{\pi}{4}\right) - a^2 b\left(\tfrac{5}{6} - \tfrac{\pi}{4}\right)\Bigr],\quad a = 1,\ b = 3'

[area]
value = 448.13774725142827
exact = "25π + 16π + 70π + 2π∫₀^{π/2}(4 + cos θ)√(sin²θ + 9cos²θ)dθ = 111π + 99.42096270296122"
tex = '111\pi + 2\pi\int_0^{\pi/2}(4 + \cos\theta)\sqrt{\sin^2\theta + 9\cos^2\theta}\,d\theta = 111\pi + 99.42096270296122'
+++

A cylinder of radius 5 on the z axis, from z = 0 to z = 10. Round the circle where its side meets
its top unequally: 1 in across the top and 3 down the side, with a section that is a quarter
ellipse.

In each half plane through the z axis the round's section is the quarter of the ellipse centered at
distance 4 from the axis and height 7, with semi-axis 1 across (along the radius) and 3 up, that
runs from the top at distance 4 from the axis, where it is tangent to the top, to the side at
height 7, where it is tangent to the side: the points (ρ, z) = (4 + cos θ, 7 + 3 sin θ),
0 ≤ θ ≤ 90°, where ρ is the distance from the axis. The round is that quarter ellipse turned round
the z axis, and the corner outside it is removed. The top is left the disk of radius 4 and the side
ends at z = 7.

(An asymmetric round: the quarter ellipse is what a round of 1 on one face and 3 on the other
makes square to a straight edge between faces at 90°. Here the edge is a circle.)

**As a set:**

$$\{\, \rho \le 5,\ 0 \le z \le 10 \,\} \setminus \bigl\{\, \rho > 4,\ z > 7,\ (\rho - 4)^2 + \bigl(\tfrac{z - 7}{3}\bigr)^2 > 1 \,\bigr\}$$
