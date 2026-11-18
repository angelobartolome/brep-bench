//! The reverse member of the crossed pair: `swept/tumbling-torus-crossed-forward` with the second
//! turn by +t, through the kernel's `sweep_solid_tumbled`.
use crate::solids::swept::crossed_tumble;
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    crossed_tumble(1.0)
}
