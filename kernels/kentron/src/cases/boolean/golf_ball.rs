//! A ball of radius 10 about the origin with 62 dimples, each cut by its own ball of radius 1.2
//! whose center is 10.6 from the origin toward one of the 12 vertices, the 30 edge midpoints or
//! the 20 face centers of the icosahedron with vertices (0, ±1, ±φ), (±1, ±φ, 0), (±φ, 0, ±1).
//! Every ball's poles are on the vertical through its center.
use crate::bench::{frame, v};
use kentron::boolean;
use kentron::math::{Direction, Point, Vector};
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};

/// The 62 directions toward the icosahedron's vertices, edge midpoints and face centers: an edge
/// joins two vertices 2 apart, its shortest, and a face is three vertices each 2 from the others.
fn directions() -> Vec<Vector> {
    let phi = (1.0 + 5f64.sqrt()) / 2.0;
    let mut vertices = Vec::new();
    for s in [-1.0, 1.0] {
        for t in [-1.0, 1.0] {
            vertices.extend([v(0.0, s, t * phi), v(s, t * phi, 0.0), v(t * phi, 0.0, s)]);
        }
    }
    let edge = |a: Vector, b: Vector| ((a - b).length() - 2.0).abs() < 1e-9;
    let mut found = vertices.clone();
    for i in 0..12 {
        for j in i + 1..12 {
            if edge(vertices[i], vertices[j]) {
                found.push(vertices[i] + vertices[j]);
                for k in j + 1..12 {
                    if edge(vertices[i], vertices[k]) && edge(vertices[j], vertices[k]) {
                        found.push(vertices[i] + vertices[j] + vertices[k]);
                    }
                }
            }
        }
    }
    found
}

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let mut ball = Sphere::new(10.0).build(m)?;
    for u in directions() {
        let center = Point::ORIGIN + u * (10.6 / u.length());
        let dimple = Sphere::new(1.2).with_frame(frame(center, Direction::Z)).build(m)?;
        ball = boolean::difference(m, ball, dimple)?;
    }
    Ok(ball)
}
