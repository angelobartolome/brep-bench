//! What the cnc implementations share: the solid a mill passes through as it slides, its axis
//! upright, round a closed level path.
//!
//! kentron has no sweep of a solid (`sweep::pipe` refuses one), so the cut is built as
//! elementary geometry gives it (SPEC.md, "A swept solid"). Each level of an upright mill is a
//! disc about its axis, of the mill's radius at that height; slid along a level path it passes
//! through every point within that radius of the path. So the mill passes through the solid the
//! mill's section through its axis (its outline either side of the axis), stood square to the
//! path, sweeps along it: the pipe of that section round the path, wherever the path turns more
//! gently than the mill is wide.
use kentron::math::{Direction, Frame, Point, Vector};
use kentron::sketch::{self, Outcome, Step};
use kentron::sweep;
use kentron::topology::{Model, Shape};

/// The closed level path the pen `steps` draw on the plane z = `z`.
pub fn level_path(m: &mut Model, z: f64, steps: &[Step]) -> kentron::Result<Shape> {
    let plane = Frame::new(Point::new(0.0, 0.0, z), Direction::Z, Direction::X);
    sketch::profile(m, plane, steps, Outcome::ClosedWire)
}

/// What an upright mill passes through as it slides once round `path`: `section`, its outline
/// either side of its axis drawn by the pen in the plane square to the path where the path
/// starts (x across the path to its right, y up, the origin the tip), swept round `path`.
pub fn milled_round(m: &mut Model, path: Shape, start: Point, heading: Direction, section: &[Step]) -> kentron::Result<Shape> {
    // x to the right of the heading, y up: the plane's normal is x × y, against the heading.
    let right = Direction::new(heading.y(), -heading.x(), 0.0);
    let plane = Frame::new(start, -heading, right);
    let face = sketch::profile(m, plane, section, Outcome::Face)?;
    Ok(sweep::pipe(m, face, path)?.shape)
}

/// What an upright flat end mill of `radius`, `length` long from its tip, passes through as it
/// slides level from `from` to `to`, its tip at height `z`: each level of it is a disc, which
/// slid along a line passes through the stadium about the line, the rectangle as wide as the
/// disc and a half disc at each end. So the stadium's prism from `z` up `length`.
pub fn slid_flat_mill(m: &mut Model, from: (f64, f64), to: (f64, f64), z: f64, radius: f64, length: f64) -> kentron::Result<Shape> {
    let (dx, dy) = (to.0 - from.0, to.1 - from.1);
    let run = dx.hypot(dy);
    // in a plane at the start with x along the slide: from the start's right, along, round the
    // end and back, round the start
    let plane = Frame::new(Point::new(from.0, from.1, z), Direction::Z, Direction::new(dx, dy, 0.0));
    let steps = [
        Step::Start(0.0, -radius),
        Step::Forward(run),
        Step::Arc { radius, angle: 180.0 },
        Step::Forward(run),
        Step::Arc { radius, angle: 180.0 },
    ];
    let face = sketch::profile(m, plane, &steps, Outcome::Face)?;
    Ok(sweep::extrude(m, face, Vector::new(0.0, 0.0, length))?.shape)
}
