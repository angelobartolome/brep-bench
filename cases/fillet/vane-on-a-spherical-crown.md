+++
title = "A vane on a spherical crown with its foot rounded"
tags = ["fillet", "concave", "sphere", "tangent-chain", "union"]
origin = "a geomsolver refusal, bmander/geomsolver#962"
settled = false
+++

A disc with a crowned top and a thin vane standing on it, the loop where the vane meets the crown
rounded.

The disc is a solid of revolution about the z axis: flat beneath, in the plane z = 0, radius
100, its rim the cylinder ρ = 100 from z = 0 up to z = 3, and its top a spherical cap, the sphere
of radius 800 through the rim's top circle, centred on the z axis below, at
c = (0, 0, 3 − √(800² − 100²)) = (0, 0, 3 − 100√63). The crown rises from 3 at the rim to
803 − 100√63 ≈ 9.2746 on the axis.

The vane is an obround prism standing upright: its plan is every point within 2 of the segment
from (22, 0) to (88, 0), two straight sides at y = ±2 joined by half circles of radius 2 about
the segment's ends, so it runs from x = 20 to x = 90, 4 wide; and it rises from z = 1, inside the
disc, to z = 31. The solid is the union of the disc and the vane.

Round the loop where the vane's sides meet the crown to radius 2: the concave round a ball of
radius 2 sweeps as it rolls round the vane's foot touching both the vane and the sphere. The foot
is one tangent chain, two straight runs along the flat sides and two half circles round the ends,
so the round goes all the way round unbroken and has no corner. Every other edge stays sharp.

The crown slopes under the vane, more steeply toward the rim, so the round's sections lean: at
each seam where a flat side runs on into a round end, the section through the seam's point on the
foot is not the one in which the ball's contact with the vane crosses the seam.

The round has no closed form, so the case is unsettled until two kernels agree.

**As a set:**

$$(K \oplus B_2) \ominus B_2,\quad K = \{\,\rho \le 100,\ z \ge 0,\ \|p - c\| \le 800\,\} \cup
\{\,d((x, y), s) \le 2,\ 1 \le z \le 31\,\}$$

where s is the segment from (22, 0) to (88, 0): K grown by 2 and shrunk by 2 again, which fills
the concave loop at the vane's foot with the rolling ball's round and leaves every convex edge
sharp.
