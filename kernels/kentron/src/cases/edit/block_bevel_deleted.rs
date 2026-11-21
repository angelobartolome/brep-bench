//! The block from the origin to (20, 10, 10) with the edge where its top meets its side x = 20
//! beveled 3 back on each face: the pentagon (0, 0), (20, 0), (20, 7), (17, 10), (0, 10) in the
//! plane y = 0, as (x, z), extruded 10 along +y. The bevel is deleted: the top and the side run on
//! to meet, and the result is the box from the origin to (20, 10, 10).
use crate::bench::{p, v};
use crate::shared::chamfer::prism;
use crate::shared::edit::deleted;
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let section = [(0.0, 0.0), (20.0, 0.0), (20.0, 7.0), (17.0, 10.0), (0.0, 10.0)];
    let corners: Vec<_> = section.iter().map(|&(x, z)| p(x, 0.0, z)).collect();
    let block = prism(m, &corners, v(0.0, 10.0, 0.0))?;
    deleted(m, block, &[p(18.5, 5.0, 8.5)])
}
