+++
title = "A ring pushed into a block just past the lean where its inner outline folds"
tags = ["swept", "solid-sweep", "slide", "torus", "leaning", "fold", "cusp", "swallowtail", "stopped", "boolean", "cut"]
origin = "geomsolver #880 T1e"
settled = false
+++

A block, the box from (−8, −8, −8) to (8, 8, 0), with a hole bored into its top by a ring pushed
straight down into it at a lean, stopping wholly inside it.

The ring is a torus centered at (0, 0, 4), its axis leaning 56° from upright toward +x, along the
direction (sin 56°, 0, cos 56°): the circle through its tube's centers has radius 3, and its tube
has radius 1. As placed it is wholly above the block.

It slides without turning straight down by 8, so that its center ends at (0, 0, −4), the whole ring
under the block's top. Cut from the block everything the ring passes through.

The ring leans just past tan² β = (R − r)/r (here 54.7°), where the curve along which its normal is
level, seen from above, first cusps. Here its inner outline turns back twice, at cusps about 0.02
apart, and crosses itself between them: a swallowtail far narrower than the ring, the wall swept
down from it crossing itself along the vertical line over the crossing. Of that wall only what
bounds the hole is kept.
