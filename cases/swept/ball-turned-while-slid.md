+++
title = "A ball on an arm, turned half a turn as its axis slides"
tags = ["swept", "solid-sweep", "turn", "slide", "sphere", "canal", "trochoid"]

[volume]
value = 61.32537597321975
exact = "πr²L + 4πr³/3, r = 1, L = ∫₀¹|c′(t)|dt = ∫₀¹√(625 − 250π sin πt + 25π²)dt = 18.187140112880417, the length of the center's path c(t) = (5 cos πt + 25t, 5 sin πt, 0) (a tube's volume is its section's area times its center line's length wherever the line curves no tighter than its radius, and the ends are two half balls)"
tex = '\pi r^2 L + \tfrac{4}{3}\pi r^3,\quad r = 1,\ L = \int_0^1\sqrt{625 - 250\pi\sin\pi t + 25\pi^2}\,dt = 18.187140112880417'

[area]
value = 126.8395421512259
exact = "2πrL + 4πr², r = 1, L = 18.187140112880417"
tex = '2\pi r L + 4\pi r^2,\quad r = 1,\ L = 18.187140112880417'
+++

A ball of radius 1 centered at (5, 0, 0), at the end of an arm of length 5 from the z axis. Sweep
it by a rigid motion that turns it about the z axis while sliding it along x, both at a steady
rate: at each instant t from 0 to 1 the ball is turned right-handed about the z axis by 180°·t and
then slid by (25t, 0, 0). The solid is everything the ball passes through.

It is not a screw: the slide is square to the turn's axis, so the motion is a rolling one, the ball
carried on a wheel of radius 25/π ≈ 7.96 rolling along a line, and its center runs along a curtate
trochoid, c(t) = (5 cos πt + 25t, 5 sin πt, 0), from (5, 0, 0) to (20, 0, 0) by way of
(12.5, 5, 0). The path is tightest at its top, t = ½, where its radius of curvature is 1.75, more
than the ball's, so the solid is the tube of radius 1 about that path, with a half ball at each end
and no fold.

**As a set:**

$$\bigcup_{0 \le t \le 1} B_1\bigl(c(t)\bigr),\quad c(t) = (5\cos\pi t + 25t,\ 5\sin\pi t,\ 0)$$
