//! #3DBenchy, the tugboat from its designer's CAD file: a reference shape, one closed B-rep of
//! 1385 faces (361 B-spline surfaces and 506 surfaces of extrusion among them), given only as the
//! reference's STEP file. A pass is that solid within 2e-5.
//!
//! Refused, not attempted. An implementation may read nothing at run time and may not take the
//! reference's faces in as its own, so a pass would need the boat modelled anew in kentron from
//! numbers measured off the reference (sections, profiles, nets) to within 2e-5 of the
//! original's 1385 faces. kentron has the lofts, sweeps, Booleans and fillets to model a
//! likeness, but that modelling has not been done here.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err("the solid is given only as the reference's STEP file (1385 faces, 361 of them B-spline \
         nets and 506 extrusions of free-form curves), which may not be read at run time nor its faces \
         taken in; modelling the boat anew in kentron to within 2e-5 has not been attempted"
        .into())
}
