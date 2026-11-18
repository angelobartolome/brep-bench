//! The union of the crossed pair of torus tumbles, each swept by the kernel's
//! `sweep_solid_tumbled` (`swept/tumbling-torus-crossed-forward` and `-reverse`), then a boolean.
use crate::solids::swept::crossed_tumble;
use crate::Refusal;
use brep_kernel::{boolean_operation, BooleanOperation, BooleanOptions, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let forward = crossed_tumble(-1.0)?;
    let reverse = crossed_tumble(1.0)?;
    Ok(boolean_operation(&forward, &reverse, BooleanOperation::Union, &BooleanOptions::default())?)
}
