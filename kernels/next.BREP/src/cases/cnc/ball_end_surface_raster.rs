//! A plate, the box from (0, 0, −20) to (100, 80, 0), whose top a ball-end mill of radius 3 (a half
//! ball below a cylinder 30 above its tip) finishes in 41 raster passes over the surface
//! f(x, y) = −5 + 4 sin(πx/50) sin(πy/40): pass j along y = 2j through the 23 points
//! (−5 + 5i, 2j, f), zigzag, joined off the plate, after a plunge at (−5, 0) and before a rise at
//! (−5, 80).
//!
//! The mill stays upright, so nothing of it reaches lower than its ball, and the ball's centre runs
//! each straight move 3 above the tip: what is left is the plate below the lower envelope of the
//! capsules those segments sweep (balls of radius 3 about every point of each). The kernel reads
//! that envelope exactly over the plate (`BallEnvelope::plan`: which cylinder or sphere is lowest
//! where, bounded by the curves where they meet) and builds the block below it (`milled_block`):
//! a face on each move's cylinder or each joint's sphere where it is lowest, the walls and the
//! floor.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::mill_envelope::BallEnvelope;
use brep_kernel::mill_faces::milled_block;
use brep_kernel::{BrepSolid, Vec3};
use std::f64::consts::PI;

const RADIUS: f64 = 3.0;

fn surface(x: f64, y: f64) -> f64 {
    -5.0 + 4.0 * (PI * x / 50.0).sin() * (PI * y / 40.0).sin()
}

/// The tip's path: the plunge, the 41 passes and the moves between them, and the rise.
fn program() -> Vec<Vec3> {
    let mut tip = vec![p(-5.0, 0.0, 5.0)];
    for j in 0..41 {
        let y = 2.0 * j as f64;
        let mut pass: Vec<Vec3> = (0..=22).map(|i| { let x = -5.0 + 5.0 * i as f64; p(x, y, surface(x, y)) }).collect();
        if j % 2 == 1 {
            pass.reverse();
        }
        tip.extend(pass);
    }
    let last = *tip.last().unwrap();
    tip.push(p(last.x, last.y, 5.0));
    tip
}

pub fn build() -> Result<BrepSolid, Refusal> {
    let centres: Vec<Vec3> = program().iter().map(|t| t.add(p(0.0, 0.0, RADIUS))).collect();
    let segments = centres.windows(2).map(|w| (w[0], w[1])).collect();
    let envelope = BallEnvelope::new(RADIUS, segments).ok_or("the mill's moves lie outside the envelope's reach")?;
    let plan = envelope.plan(0.0, 100.0, 0.0, 80.0, 0.0, 0.1)?;
    Ok(milled_block(&envelope, &plan, -20.0)?)
}
