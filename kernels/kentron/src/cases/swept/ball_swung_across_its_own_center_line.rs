//! The solid a ball fills as it is swung about an upright line through it: the ball of radius 2
//! about (1, 0, 0), turned rigidly about the z axis by 120°.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) cuts a blank, and turns a tool clear past
//! it, so the solid is built as elementary geometry gives it: what a ball passes through is every
//! point within its radius of its centre's path, so it is the ball where it starts and where it
//! ends and the discs of radius 2 square to the path at each of its points. Square to the arc
//! about the z axis is the plane through the axis, so those discs are the ball's section by the
//! plane y = 0 turned 120°. That disc reaches across the axis, and kentron revolves a face
//! lying to one side of its axis: so it is turned in its two parts either side of the z axis,
//! each its own solid of revolution.
use crate::bench::{frame, p};
use crate::shared::parts::xz_plane;
use kentron::boolean;
use kentron::math::{Axis, Direction, Transform};
use kentron::modify::{self, TransformOptions};
use kentron::primitives::Sphere;
use kentron::sketch::{self, Outcome, Step};
use kentron::sweep;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    use Step::{Arc, Heading, Start};
    let turn = 120f64.to_radians();
    let chord = 3f64.sqrt();
    // The disc's part on the +x side, from (0, −√3) round through (3, 0) to (0, √3), and its part
    // on the −x side, from (0, √3) round through (−1, 0) to (0, −√3), each closed along the axis.
    let near = [Start(0.0, -chord), Heading(-30.0), Arc { radius: 2.0, angle: 240.0 }];
    let far = [Start(0.0, chord), Heading(210.0), Arc { radius: 2.0, angle: 120.0 }];
    let mut body = Sphere::new(2.0).with_frame(frame(p(1.0, 0.0, 0.0), Direction::Z)).build(m)?;
    let end = modify::transform(m, body, &Transform::rotation(Axis::Z, turn), TransformOptions::default())?;
    body = boolean::union(m, body, end)?;
    for part in [&near[..], &far[..]] {
        let part = sketch::profile(m, xz_plane(0.0), part, Outcome::Face)?;
        let turned = sweep::revolve(m, part, Axis::Z, turn)?.shape;
        body = boolean::union(m, body, turned)?;
    }
    Ok(body)
}
