//! The forward member of the crossed pair, through the kernel's sweep of a solid of revolution
//! tumbled by two turns about skew axes (`sweep_solid_tumbled`): the torus of tube radius 0.22
//! turned by 0.7t about the vertical line through (2, 0, 0), then by −t about the line through
//! (0, 0.7071 cos 45°, 0.7071 sin 45°) along (cos 20°, sin 20° cos 45°, sin 20° sin 45°); t from
//! −100° to 100°.
use crate::solids::swept::crossed_tumble;
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    crossed_tumble(-1.0)
}
