"""A ball on an arm, turned half a turn as its axis slides. A ball of radius 1 centered at (5, 0, 0),
at the end of an arm of length 5 from the z axis. Sweep it by a rigid motion that turns it about
the z axis while sliding it along x, both at a steady rate: at each instant t from 0 to 1 the ball
is turned right-handed about the z axis by 180°·t and then slid by (25t, 0, 0). The solid is
everything the ball passes through.

It is not a screw: the slide is square to the turn's axis, so the motion is a rolling one, the ball
carried on a wheel of radius 25/π ≈ 7.96 rolling along a line, and its center runs along a curtate
trochoid, c(t) = (5 cos πt + 25t, 5 sin πt, 0), from (5, 0, 0) to (20, 0, 0) by way of (12.5, 5,
0). The path is tightest at its top, t = ½, where its radius of curvature is 1.75, more than the
ball's, so the solid is the tube of radius 1 about that path, with a half ball at each end and no
fold."""
from bench import Refused


def build():
    # The ball's sweep is the tube about its center's path with a ball at each end, but the path
    # is a curtate trochoid, (5 cos πt + 25t, 5 sin πt, 0), and OCCT has no such curve: only a
    # curve fitted to points computed here, which is geometry made outside the kernel.
    raise Refused("OCCT has no sweep of a solid, and no curve for the trochoid the ball's center "
                  "runs along to sweep its circle on")
