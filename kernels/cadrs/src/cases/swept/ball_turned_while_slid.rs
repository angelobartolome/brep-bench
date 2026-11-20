//! A ball of radius 1 centered at (5, 0, 0), swept by turning it half a turn about the z axis
//! while sliding it 25 along x, both steadily: at instant t turned by 180°·t, then slid by
//! (25t, 0, 0). Its center runs along a curtate trochoid.
//!
//! cadrs's sweep of a solid by a rigid motion of two steps, the turn applied first.
use crate::bench::{ball, p3, v3};
use crate::shared::swept::z_axis;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Kernel, RigidStep, SolidMotion};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = ball(k, p3(5.0, 0.0, 0.0), 1.0)?;
    let steps = vec![
        RigidStep::Turn { axis: z_axis(), from: 0.0, to: std::f64::consts::PI },
        RigidStep::Slide { from: v3(0.0, 0.0, 0.0), to: v3(25.0, 0.0, 0.0) },
    ];
    Ok(k.sweep_solid(b, &SolidMotion::Rigid { steps })?.first()?)
}
