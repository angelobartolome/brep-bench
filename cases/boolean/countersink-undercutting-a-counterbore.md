+++
title = "A countersunk hole undercutting a counterbore's wall"
tags = ["boolean", "cut", "analytic", "cylinder", "cone", "coincident"]
origin = "a geomsolver refusal, bmander/geomsolver#961"

[volume]
value = 1029862.2429340893
exact = "πR²·2Z − πB²Z − πr²(D − k) − πk(s² + sr + r²)/3, k = s − r the sink's depth, R = 100, Z = 20, B = 60, r = 3.5, s = 4, D = 15"
tex = '2\pi R^2 Z - \pi B^2 Z - \pi r^2 (D - k) - \tfrac{\pi k}{3}\left(s^2 + sr + r^2\right),\quad k = s - r'

[area]
value = 95877.54522417982
exact = "2πR² + 4πRZ + 2πBZ + πs² − 2Λ + 2πr(D − k) + πr² + π(s + r)k√2, Λ = s²·arccos((e² + s² − B²)/(2es)) + B²·arccos((e² + B² − s²)/(2eB)) − ½√((B + s − e)(e + s − B)(e − s + B)(e + s + B)) the sink's circle within the counterbore's, e = 59.9"
tex = '2\pi R^2 + 4\pi R Z + 2\pi B Z + \pi s^2 - 2\Lambda + 2\pi r(D - k) + \pi r^2 + \pi(s + r)k\sqrt{2},\quad \Lambda = s^2\arccos\frac{e^2 + s^2 - B^2}{2es} + B^2\arccos\frac{e^2 + B^2 - s^2}{2eB} - \tfrac{1}{2}\sqrt{(B + s - e)(e + s - B)(e - s + B)(e + s + B)}'
+++

A thick disc, counterbored, with a countersunk hole drilled from the counterbore's floor so close
to its wall that the hole and its countersink reach under the wall.

The disc is the solid cylinder of radius 100 about the z axis from z = −20 to z = 20. The
counterbore removes the cylinder of radius 60 about the z axis from z = 0 up through the disc's
top, leaving the floor z = 0 inside radius 60 and the wall ρ = 60 from the floor up to the top.

The hole is the cylinder of radius 3.5 about the vertical line through (59.9, 0), from its flat
bottom z = −15 up to z = 0: its top is flush with the floor. Its axis is 0.1 inside the wall, so
nearly half the hole lies beyond ρ = 60, under the wall's material.

The countersink is the cone of 90° included angle about the same axis whose circle on the floor
plane z = 0 has radius 4: its point is 4 below, at (59.9, 0, −4), and it meets the hole's side
0.5 below the floor. Cut away, it widens the hole's mouth to the circle of radius 4 on z = 0 and
removes nothing above that plane.

The solid is the disc less the counterbore, the hole and the countersink. The circle of radius 4
crosses the wall's foot: the part of its disk inside ρ = 60 opens on the floor, and the part
beyond it, a lens, is a ceiling facing down, under the wall's material. Floor, wall, ceiling and
countersink meet at the two points where the circle crosses ρ = 60.

**As a set:**

$$\{\,\rho \le 100,\ -20 \le z \le 20\,\} \setminus \{\,\rho \le 60,\ z \ge 0\,\}
\setminus \{\,\rho' \le \max(3.5,\ 4 + z),\ -15 \le z \le 0\,\},\quad
\rho' = \sqrt{(x-59.9)^2 + y^2}$$
