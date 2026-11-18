//! The solid a torus fills as it tumbles, through the kernel's sweep of a solid of revolution
//! tumbled by two turns about skew axes (`sweep_solid_tumbled`): the torus centred at (3, 0, 2),
//! axis along z, the circle through its tube's centres of radius 1, tube radius 0.5; at t turned by
//! t/4 about the vertical line through (2, 0, 0), then by −t about the line through (0, 0.5, 0.5)
//! along x; t from −75° to 75°.
use crate::solids::swept::tumbled_torus;
use crate::Refusal;
use brep_kernel::BrepSolid;

pub fn build() -> Result<BrepSolid, Refusal> {
    tumbled_torus(0.5, 0.25, -1.0, 0.5f64.sqrt(), 45.0, 0.0, -75.0, 75.0)
}
