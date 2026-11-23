//! A dog-bone plate with a round boss on one lobe, the boss as wide as the lobe and on its axis.
//!
//! The outline (lobes of radius 10 about (±30, 0), sides y = ±4, concave blends of radius 6 about
//! (±(30 − 2√39), ±10)) as eight exact arcs and lines, extruded from z = 0 to 10; the boss the
//! cylinder of radius 10 about (30, 0) from z = 0 to 30; their union by the kernel.
use crate::solids::boolean::union;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, make_arc, make_cylinder_brep, BrepSolid, NurbsCurve, Vec3, Vec4};

fn line(a: Vec3, b: Vec3) -> Result<NurbsCurve, Refusal> {
    Ok(NurbsCurve::new(1, vec![0.0, 0.0, 1.0, 1.0], vec![Vec4::from_point(a, 1.0), Vec4::from_point(b, 1.0)])?)
}

/// The arc of radius `r` about `c` from angle `a` to angle `b`, counter-clockwise when `b > a`
/// and clockwise (built the other way and reversed) when `b < a`.
fn arc(c: Vec3, r: f64, a: f64, b: f64) -> Result<NurbsCurve, Refusal> {
    let (x, y) = (p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0));
    if b > a {
        Ok(make_arc(c, x, y, r, a, b)?)
    } else {
        Ok(make_arc(c, x, y, r, b, a)?.reversed()?)
    }
}

pub fn build() -> Result<BrepSolid, Refusal> {
    let s = 30.0 - 2.0 * 39f64.sqrt();
    // The blends' tangent points on the lobes: 10 from a lobe's centre toward a blend's.
    let toward = |lobe: f64, blend: Vec3| {
        let d = blend.sub(p(lobe, 0.0, 0.0));
        p(lobe, 0.0, 0.0).add(d.scale(10.0 / d.length()))
    };
    let angle = |c: Vec3, q: Vec3| (q.y - c.y).atan2(q.x - c.x);
    let (br, tr, tl, bl) = (p(s, -10.0, 0.0), p(s, 10.0, 0.0), p(-s, 10.0, 0.0), p(-s, -10.0, 0.0));
    let (t_br, t_tr, t_tl, t_bl) = (toward(30.0, br), toward(30.0, tr), toward(-30.0, tl), toward(-30.0, bl));
    let (right, left) = (p(30.0, 0.0, 0.0), p(-30.0, 0.0, 0.0));
    let tau = std::f64::consts::TAU;
    let outline = vec![
        line(p(-s, -4.0, 0.0), p(s, -4.0, 0.0))?,
        arc(br, 6.0, angle(br, p(s, -4.0, 0.0)), angle(br, t_br))?,
        arc(right, 10.0, angle(right, t_br), angle(right, t_tr) + if angle(right, t_tr) < angle(right, t_br) { tau } else { 0.0 })?,
        arc(tr, 6.0, angle(tr, t_tr), angle(tr, p(s, 4.0, 0.0)))?,
        line(p(s, 4.0, 0.0), p(-s, 4.0, 0.0))?,
        arc(tl, 6.0, angle(tl, p(-s, 4.0, 0.0)), angle(tl, t_tl))?,
        arc(left, 10.0, angle(left, t_tl), angle(left, t_bl) + if angle(left, t_bl) < angle(left, t_tl) { tau } else { 0.0 })?,
        arc(bl, 6.0, angle(bl, t_bl), angle(bl, p(-s, -4.0, 0.0)))?,
    ];
    let plate = extrude_profile_brep(&outline, p(0.0, 0.0, 1.0), 10.0)?;
    let boss = make_cylinder_brep(right, p(0.0, 0.0, 1.0), 10.0, 30.0)?;
    Ok(union(&plate, &boss)?)
}
