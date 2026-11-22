//! The ring sector 8 ≤ ρ ≤ 12, |z| ≤ 1, 0° ≤ φ ≤ 60° less what a bar (10 ≤ x ≤ 11, |y| ≤ 0.5, |z| ≤ 0.6) passes through, turned about
//! the z axis from −140° to 30°.
use anvil_brep::Solid;

use crate::shared::{block, cut_all, intersect, rect, swung, turn_z};

pub fn build() -> Result<Solid, String> {
    let sector = swung(rect((8., -1.), (12., 1.)).segments, 0., 60.)?;
    let tool = block([10., -0.5, -0.6], [11., 0.5, 0.6])?;
    let fan = swung(rect((10., -0.6), (121.25f64.sqrt(), 0.6)).segments, -140., 170.)?;
    // anvil-brep has no sweep of a solid. Turned about the z axis the tool fills the revolve of
    // what it covers in a half-plane through the axis, from its inner face, 10 out, to the circle √(11² + 0.5²) its outer corners run on, within |z| ≤ 0.6, and itself where it starts and stops
    // Between 11 and √(11² + 0.5²) from the axis the bar covers, on a circle about it, only the
    // angles between its outer face and its side, so the fan leaves out a sliver past the stop at
    // 30°, which the outer corners sweep: what the bar, carried on past its outer face, covers
    // there within the fan's reach
    let longer = block([10., -0.5, -0.6], [11.5, 0.5, 0.6])?;
    let beyond = swung(rect((10., -0.6), (121.25f64.sqrt(), 0.6)).segments, 0., 90.)?;
    let sliver = intersect(&longer, &beyond)?.transformed(&turn_z(30.));
    cut_all(
        sector,
        [Ok(fan), Ok(tool.transformed(&turn_z(-140.))), Ok(tool.transformed(&turn_z(30.))), Ok(sliver)],
    )
}
