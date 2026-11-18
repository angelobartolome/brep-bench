//! A torus of tube radius 0.16 tumbled through a full roll, through the kernel's sweep of a solid of
//! revolution tumbled by two turns about skew axes (`sweep_solid_tumbled`): `swept/tumbling-torus-
//! half-turn` with t from −180° to 180°, swept by halves and joined.
use crate::solids::swept::tumbled_torus;
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    tumbled_torus(0.16, 0.375, -1.0, 0.7071, 30.0, 20.0, -180.0, 180.0)
}
