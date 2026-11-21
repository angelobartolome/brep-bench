//! A thin torus tumbled through space, through the kernel's sweep of a solid of revolution tumbled
//! by two turns about skew axes (`sweep_solid_tumbled`): `swept/tumbling-torus` with tube radius
//! 0.015.
use crate::solids::swept::tumbled_torus;
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    tumbled_torus(0.015, 0.25, -1.0, 0.5f64.sqrt(), 45.0, 0.0, -75.0, 75.0)
}
