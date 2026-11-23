//! Closed plan outlines in the plane z = `height` as chains of straight pieces and circular arcs,
//! each piece's middle kept so an edge over it can be picked by a point.
use crate::Refusal;
use brep_kernel::{make_arc, make_line, NurbsCurve, Vec3};
use std::f64::consts::TAU;

/// A chain being built: its curves and each one's middle point.
pub struct Chain {
    pub curves: Vec<NurbsCurve>,
    pub middles: Vec<Vec3>,
}

impl Chain {
    pub fn new() -> Self {
        Chain { curves: Vec::new(), middles: Vec::new() }
    }

    /// The straight piece from (x0, y0) to (x1, y1).
    pub fn line(&mut self, (x0, y0): (f64, f64), (x1, y1): (f64, f64)) -> Result<&mut Self, Refusal> {
        self.curves.push(make_line(Vec3::new(x0, y0, 0.0), Vec3::new(x1, y1, 0.0))?);
        self.middles.push(Vec3::new(0.5 * (x0 + x1), 0.5 * (y0 + y1), 0.0));
        Ok(self)
    }

    /// The arc of radius `r` about (cx, cy) from angle `a` to angle `b` (radians), anticlockwise
    /// when `ccw`, else clockwise; the end angle is taken a whole turn on or back as needed.
    pub fn arc(&mut self, (cx, cy): (f64, f64), r: f64, a: f64, mut b: f64, ccw: bool) -> Result<&mut Self, Refusal> {
        let (x, y, c) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(cx, cy, 0.0));
        if ccw {
            while b <= a {
                b += TAU;
            }
            self.curves.push(make_arc(c, x, y, r, a, b)?);
        } else {
            while b >= a {
                b -= TAU;
            }
            self.curves.push(make_arc(c, x, y, r, b, a)?.reversed()?);
        }
        let m = 0.5 * (a + b);
        self.middles.push(Vec3::new(cx + r * m.cos(), cy + r * m.sin(), 0.0));
        Ok(self)
    }

    /// The middles lifted to height `z`.
    pub fn middles_at(&self, z: f64) -> Vec<Vec3> {
        self.middles.iter().map(|m| Vec3::new(m.x, m.y, z)).collect()
    }
}

/// Degrees as radians.
pub fn deg(d: f64) -> f64 {
    d.to_radians()
}
