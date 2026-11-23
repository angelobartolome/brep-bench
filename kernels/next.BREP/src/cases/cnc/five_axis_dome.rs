//! A round blank (radius 50 about the z axis, z from 0 to 45) machined into a dome by a five-axis
//! program of four cutters: roughed by a bull-nose mill in level rings, finished by a ball-end mill of
//! radius 4 held square to the dome of radius 40 along 180 meridians (latitude 3° to 75°, every 2° of
//! longitude), round 15 circles near the pole (latitude 75° to 89°) and at the pole, then spot-faced
//! (a flat end mill of radius 5 to 38.5 along the normal) and drilled (radius 2.5, a 90° point, to 30)
//! at twelve points of latitude 40°.
//!
//! Held square to the dome with its ball touching it, the finishing mill's ball centre always lies 44
//! from the origin and its shank reaches away from it; its passes turn it about lines through the
//! origin (the meridians: great-circle arcs of centres) or about the z axis (the circles near the
//! pole). So along each ray from the origin the part ends where the nearest ball first meets the ray,
//! a function of the angle to the nearest centre curve alone; the roughing lies wholly beyond that
//! surface, and the shanks clear everything beyond it. The kernel reads that surface over a chart of
//! the upper half of the sphere (`DomeField`, `plan_field`) and builds the solid under it, closed by
//! the plane z = 0 (`block_from_plan`). The seats and holes are cut from it in one n-ary subtract.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::mill_dome::{CentreCurve, Centres, DomeField};
use brep_kernel::mill_faces::{block_from_plan, Sides};
use brep_kernel::mill_plan::plan_field;
use brep_kernel::{boolean_operation_nary, make_cone_brep, make_cylinder_brep, BooleanOperation, BrepSolid, Vec3};
use std::f64::consts::PI;

fn rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

/// The dome's outward normal at latitude `lat`, longitude `lon` (degrees).
fn n(lat: f64, lon: f64) -> Vec3 {
    let (l, o) = (rad(lat), rad(lon));
    p(l.cos() * o.cos(), l.cos() * o.sin(), l.sin())
}

pub fn build() -> Result<BrepSolid, Refusal> {
    // The finishing ball's centre curves: each meridian pass turns the centre from latitude 3° by 72°
    // about the level line along (sin φ, −cos φ, 0); each pass round the pole a whole turn about z; the
    // pole itself one point.
    let mut curves = Vec::new();
    for k in 0..180 {
        let phi = 2.0 * k as f64;
        curves.push(CentreCurve::Arc { from: n(3.0, phi), axis: p(rad(phi).sin(), -rad(phi).cos(), 0.0), angle: rad(72.0) });
    }
    for lat in 75..=89 {
        curves.push(CentreCurve::Circle { axis: p(0.0, 0.0, 1.0), polar: rad(90.0 - lat as f64) });
    }
    curves.push(CentreCurve::Point(p(0.0, 0.0, 1.0)));
    let field = DomeField::new(Centres { ball: 4.0, dome: 40.0, curves }, 3, 100.0)?;
    let plan = plan_field(&field, [-1.0, 1.0, -1.0, 1.0], 0.01)?;
    let dome = block_from_plan(&field, &field, &plan, Sides::Floor { origin: p(0.0, 0.0, 0.0), normal: p(0.0, 0.0, -1.0) }, 4.0 / 32.0 / 40.0)?;
    // The seats (a flat end mill of radius 5, its end 38.5 from the origin, reaching out along the
    // normal) and the holes (a drill of radius 2.5 whose 90° point is at 30), at latitude 40°.
    let mut operands = vec![dome];
    for k in 0..12 {
        let d = n(40.0, 15.0 + 30.0 * k as f64);
        operands.push(make_cylinder_brep(d.scale(38.5), d, 5.0, 15.0)?);
        operands.push(make_cone_brep(d.scale(30.0), d, 0.0, 2.5, 2.5)?);
        operands.push(make_cylinder_brep(d.scale(32.5), d, 2.5, 20.0)?);
    }
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}
