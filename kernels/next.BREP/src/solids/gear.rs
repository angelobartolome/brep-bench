//! What the gear-generation cases share: a tooth space cut by a tool under a generating motion,
//! found point by point from the set's own definition and built from the curves so found.
//!
//! A point is in the space where some place of the tool along the motion holds it: the case
//! gives the tool as inequalities and the motion as a map, so `Generating::margin(x, t)` is the
//! least slack of the tool's inequalities at the point x carried back by the motion at t
//! (positive inside). The space is cut by planes square to the gear's axis (slices); in each, it
//! is a star seen from a point Q outside the gear, so along the ray from Q at each angle β the
//! space (with the air beyond the radius `r_top`, outside the blank, counted in) runs from Q to
//! one distance D(β), found by bisection. Each ray's end lies on one surface of the tool's sweep:
//! the envelope of one of the tool's faces, or the track of one of its edges (where two
//! inequalities bind at once), or the circle r = r_top where the ray meets a tooth's top. Where
//! the binding inequalities change, the curve has a corner; corners are found by bisection, and
//! the curve between two is interpolated through its exact points by the kernel's cubic fit.
//!
//! Every slice must have the same corners in the same order; the slices' closed loops (Q, out
//! along the first ray, the curve, back along the last) are lofted by the kernel into the
//! space's cutter.
use crate::Refusal;
use std::f64::consts::TAU;
use brep_kernel::{interpolate_curve, loft_profile_brep, make_line, BrepSolid, NurbsCurve, Vec3};

/// A generating motion and its tool, from the case.
pub trait Generating {
    /// The least slack of the tool's inequalities at `x` carried back by the motion at `t`, and
    /// which of them bind (within `tight` of the least), as a bit set.
    fn margin(&self, x: Vec3, t: f64) -> (f64, u32);
    /// The motion parameters at which the tool can hold `x`: a range, and a step fine enough to
    /// see every local best of the margin in it.
    fn window(&self, x: Vec3) -> Option<(f64, f64, f64)>;
}

/// The bit set of the circle r = r_top, where a ray meets a tooth's top in the air.
pub const RIM: u32 = 1 << 31;

/// The best margin over the motion at `x`, and what binds there.
fn best<G: Generating>(gear: &G, x: Vec3) -> (f64, u32) {
    let Some((a, b, step)) = gear.window(x) else { return (-1.0, 0) };
    let n = ((b - a) / step).ceil().max(2.0) as usize;
    let samples: Vec<f64> = (0..=n).map(|i| gear.margin(x, a + (b - a) * i as f64 / n as f64).0).collect();
    // Refine round the three best samples.
    let mut order: Vec<usize> = (0..=n).collect();
    order.sort_by(|&i, &j| samples[j].total_cmp(&samples[i]));
    let mut top = (f64::NEG_INFINITY, 0);
    for &i in order.iter().take(3) {
        let lo = a + (b - a) * (i.max(1) - 1) as f64 / n as f64;
        let hi = a + (b - a) * (i + 1).min(n) as f64 / n as f64;
        let (mut l, mut h) = (lo, hi);
        let phi = 0.5 * (5f64.sqrt() - 1.0);
        let (mut c, mut d) = (h - phi * (h - l), l + phi * (h - l));
        let (mut fc, mut fd) = (gear.margin(x, c).0, gear.margin(x, d).0);
        for _ in 0..40 {
            if fc > fd {
                h = d;
                d = c;
                fd = fc;
                c = h - phi * (h - l);
                fc = gear.margin(x, c).0;
            } else {
                l = c;
                c = d;
                fc = fd;
                d = l + phi * (h - l);
                fd = gear.margin(x, d).0;
            }
        }
        let found = gear.margin(x, 0.5 * (l + h));
        if found.0 > top.0 {
            top = found;
        }
    }
    top
}

/// One slice of the space: a surface given by `place`, from coordinates (a, b) in it, seen from
/// `q`, its rays straight in (a, b) at the angles in [`from`, `to`] measured from the direction
/// `toward` (a unit vector) anticlockwise; `air` says where (a, b) lies outside the blank, past
/// its face, where the space is counted in.
pub struct Slice {
    pub q: [f64; 2],
    pub toward: [f64; 2],
    pub from: f64,
    pub to: f64,
    /// Farthest a ray is followed (past any part of the space).
    pub reach: f64,
    pub place: Box<dyn Fn([f64; 2]) -> Vec3 + Send + Sync>,
    /// Where (a, b) is decided without the tool (air past the blank's face, or a plane bounding
    /// what is traced): whether it is in, and the label of that bound.
    pub bound: Box<dyn Fn([f64; 2]) -> Option<(bool, u32)> + Send + Sync>,
    /// Trace the material instead: a point is in where the tool never holds it.
    pub invert: bool,
    /// The rays go all the way round Q (from −π to π, the seam inside one piece): the loop is
    /// the curve alone.
    pub closed: bool,
}

/// A slice's `bound` for air past the blank's face (`air` true there), counted in the space.
pub fn air(f: impl Fn([f64; 2]) -> bool + Send + Sync + 'static) -> Box<dyn Fn([f64; 2]) -> Option<(bool, u32)> + Send + Sync> {
    Box::new(move |q| if f(q) { Some((true, RIM)) } else { None })
}

impl Slice {
    fn at(&self, beta: f64, s: f64) -> [f64; 2] {
        let (c, sn) = (beta.cos(), beta.sin());
        let u = [c * self.toward[0] - sn * self.toward[1], sn * self.toward[0] + c * self.toward[1]];
        [self.q[0] + s * u[0], self.q[1] + s * u[1]]
    }

    fn ray(&self, beta: f64, s: f64) -> Vec3 {
        (self.place)(self.at(beta, s))
    }

    /// Whether the point at distance `s` along the ray at `beta` is in the space or in the air
    /// past the blank's face, and what bounds it there.
    fn inside<G: Generating>(&self, gear: &G, beta: f64, s: f64) -> (bool, u32) {
        if let Some(decided) = (self.bound)(self.at(beta, s)) {
            return decided;
        }
        let (m, bind) = best(gear, self.ray(beta, s));
        (if self.invert { m < 0.0 } else { m >= 0.0 }, bind)
    }

    /// The ray's end (distance D) and what binds there.
    fn end<G: Generating>(&self, gear: &G, beta: f64) -> Result<(f64, u32), Refusal> {
        // March out from Q to the first point outside, then bisect.
        let steps = ((self.reach / 0.25).ceil() as usize).max(8);
        let mut last = 0.0;
        for i in 1..=steps {
            let s = self.reach * i as f64 / steps as f64;
            if !self.inside(gear, beta, s).0 {
                let (mut a, mut b) = (last, s);
                for _ in 0..60 {
                    let m = 0.5 * (a + b);
                    if self.inside(gear, beta, m).0 {
                        a = m;
                    } else {
                        b = m;
                    }
                }
                // What binds at the boundary: the face where the inside side is air, else the
                // tool's inequalities just inside it.
                // What binds there: a bound on either side, else the tool's inequalities on the
                // side the tool holds.
                let bind = match ((self.bound)(self.at(beta, a)), (self.bound)(self.at(beta, b))) {
                    (Some((_, label)), _) | (None, Some((_, label))) => label,
                    _ => best(gear, self.ray(beta, if self.invert { b } else { a })).1,
                };
                return Ok((0.5 * (a + b), bind));
            }
            last = s;
        }
        Err(Refusal::from("a ray from outside the gear never leaves the tooth space"))
    }
}

/// One slice's corners: the angles where what binds changes, and the labels between them.
pub struct Corners {
    pub cuts: Vec<f64>,
    pub labels: Vec<u32>,
}

/// Find one slice's corners, from `samples` rays and bisection between them.
pub fn corners<G: Generating>(gear: &G, slice: &Slice, samples: usize) -> Result<Corners, Refusal> {
    let betas: Vec<f64> = (0..=samples).map(|i| slice.from + (slice.to - slice.from) * i as f64 / samples as f64).collect();
    let mut ends = Vec::with_capacity(betas.len());
    for &b in &betas {
        ends.push(slice.end(gear, b)?.1);
    }
    let mut cuts = vec![slice.from];
    let mut labels = vec![ends[0]];
    for i in 0..samples {
        if ends[i + 1] != ends[i] {
            // Bisect for where the left label ends; a piece too short for the samples to see
            // shows as another label just past it, and is followed in turn.
            let (mut start, mut left) = (betas[i], ends[i]);
            for _ in 0..8 {
                let (mut a, mut b) = (start, betas[i + 1]);
                for _ in 0..44 {
                    let m = 0.5 * (a + b);
                    if slice.end(gear, m)?.1 == left {
                        a = m;
                    } else {
                        b = m;
                    }
                }
                let next = slice.end(gear, b)?.1;
                cuts.push(0.5 * (a + b));
                labels.push(next);
                if next == ends[i + 1] {
                    break;
                }
                (start, left) = (b, next);
            }
        }
    }
    cuts.push(slice.to);
    Ok(Corners { cuts, labels })
}

/// The pieces every slice must have, in order. A piece some slices lack (a face of the space
/// that grows from a point between two slices) is given to those slices as a thin strip split
/// off the start of the piece that follows it there (`strip` of that piece's angles): its points
/// are still exact points of the boundary, so the loft's face for it narrows to that strip on
/// the side where the face is not, and only the loft's interpolation across the place where the
/// face begins is not exact.
pub fn common(all: &[Corners], strip: f64) -> Result<Vec<Corners>, Refusal> {
    let mut out: Vec<Corners> = all.iter().map(|c| Corners { cuts: c.cuts.clone(), labels: c.labels.clone() }).collect();
    let fullest = all.iter().max_by_key(|c| c.labels.len()).expect("slices").labels.clone();
    for (k, &label) in fullest.iter().enumerate() {
        if out.iter().all(|c| c.labels.contains(&label)) {
            continue;
        }
        if k == 0 || k + 1 == fullest.len() {
            return Err(Refusal::from("a piece of the tooth space's boundary that some slices lack lies at an end"));
        }
        let (before, after) = (fullest[k - 1], fullest[k + 1]);
        for c in out.iter_mut() {
            if c.labels.contains(&label) {
                continue;
            }
            let i = (0..c.labels.len() - 1)
                .find(|&i| c.labels[i] == before && c.labels[i + 1] == after)
                .ok_or_else(|| Refusal::from("the tooth space's pieces come in another order between slices"))?;
            let (start, end) = (c.cuts[i + 1], c.cuts[i + 2]);
            c.cuts.insert(i + 2, start + strip * (end - start));
            c.labels.insert(i + 1, label);
        }
    }
    let first = out[0].labels.clone();
    for c in &out {
        if c.labels != first {
            return Err(Refusal::from(format!(
                "the tooth space's boundary changes its pieces between slices ({first:?} against {:?})",
                c.labels
            )));
        }
    }
    Ok(out)
}

/// Whether the boundary turns by less than `tolerance` radians across the angle `beta`.
fn tangent_across<G: Generating>(gear: &G, slice: &Slice, beta: f64, tolerance: f64) -> Result<bool, Refusal> {
    let h = 1e-4 * (slice.to - slice.from);
    let mut q = Vec::new();
    for k in [-2.0, -1.0, 1.0, 2.0] {
        let b = beta + k * h;
        q.push(slice.ray(b, slice.end(gear, b)?.0));
    }
    let (u, v) = (q[1].sub(q[0]), q[3].sub(q[2]));
    Ok(u.dot(v) / (u.length() * v.length()) > tolerance.cos())
}

/// The loop of one slice sampled at `points` exact points per piece.
pub struct Traced {
    pub pieces: Vec<Vec<Vec3>>,
    pub q: Vec3,
    pub closed: bool,
}

pub fn sample<G: Generating>(gear: &G, slice: &Slice, corners: &Corners, kept: &[u32], points: usize) -> Result<Traced, Refusal> {
    let mut ranges: Vec<(f64, f64)> = corners.cuts.windows(2).map(|w| (w[0], w[1])).collect();
    debug_assert_eq!(ranges.len(), kept.len());
    if slice.closed {
        // The seam lies inside one piece: its two ends make one range across it.
        let last = ranges.pop().expect("pieces");
        ranges[0] = (last.0 - TAU, ranges[0].1);
    }
    let mut pieces = Vec::new();
    for &(a, b) in &ranges {
        let mut piece = Vec::with_capacity(points);
        for j in 0..points {
            let beta = a + (b - a) * j as f64 / (points - 1) as f64;
            let (d, _) = slice.end(gear, beta)?;
            piece.push(slice.ray(beta, d));
        }
        pieces.push(piece);
    }
    for k in 1..pieces.len() {
        let shared = pieces[k - 1][points - 1];
        pieces[k][0] = shared;
    }
    if slice.closed {
        let n = pieces.len();
        let shared = pieces[n - 1][points - 1];
        pieces[0][0] = shared;
    }
    Ok(Traced { pieces, q: (slice.place)(slice.q), closed: slice.closed })
}

fn fit(points: &[Vec3]) -> Result<NurbsCurve, Refusal> {
    let n = points.len();
    let parameters: Vec<f64> = (0..n).map(|i| i as f64 / (n - 1) as f64).collect();
    Ok(interpolate_curve(points, 3, &parameters)?)
}

/// The cutter: the slices' loops (Q, out along the first ray, the pieces, back along the last
/// ray) lofted.
pub fn cutter(slices: &[Traced]) -> Result<BrepSolid, Refusal> {
    let mut sections = Vec::new();
    for s in slices {
        let first = s.pieces[0][0];
        let last = *s.pieces.last().unwrap().last().unwrap();
        let mut curves = if s.closed { Vec::new() } else { vec![make_line(s.q, first)?] };
        for piece in &s.pieces {
            curves.push(fit(piece)?);
        }
        if !s.closed {
            curves.push(make_line(last, s.q)?);
        }
        sections.push(curves);
    }
    Ok(loft_profile_brep(&sections)?)
}

/// Trace the space in every slice and loft it.
pub fn space<G: Generating + Sync>(gear: &G, slices: &[Slice], samples: usize, points: usize, strip: f64) -> Result<BrepSolid, Refusal> {
    // The slices are independent: traced on as many threads as there are cores.
    let all: Vec<Corners> = each(slices, |slice| corners(gear, slice, samples))?;
    let joined = common(&all, strip)?;
    let kept = joined[0].labels.clone();
    let pairs: Vec<(&Slice, &Corners)> = slices.iter().zip(&joined).collect();
    let traced: Vec<Traced> = each(&pairs, |(slice, c)| sample(gear, slice, c, &kept, points))?;
    cutter(&traced)
}

/// `f` of every item, in order, on as many threads as there are cores.
fn each<T: Sync, R: Send>(items: &[T], f: impl Fn(&T) -> Result<R, Refusal> + Sync) -> Result<Vec<R>, Refusal> {
    let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1).min(items.len()).max(1);
    let mut out: Vec<Option<Result<R, Refusal>>> = (0..items.len()).map(|_| None).collect();
    std::thread::scope(|scope| {
        let chunks: Vec<_> = out.chunks_mut(items.len().div_ceil(threads)).zip(items.chunks(items.len().div_ceil(threads))).collect();
        for (slots, part) in chunks {
            let f = &f;
            scope.spawn(move || {
                for (slot, item) in slots.iter_mut().zip(part) {
                    *slot = Some(f(item));
                }
            });
        }
    });
    out.into_iter().map(|r| r.expect("every item is done")).collect()
}

/// The runs of the space along the curve `point(c)` for c in [`lo`, `hi`], from `n` samples, each
/// end found by bisection: where a gap lies across a tooth's top, to place a slice's rays.
pub fn runs<G: Generating>(gear: &G, point: impl Fn(f64) -> Vec3, lo: f64, hi: f64, n: usize) -> Vec<(f64, f64)> {
    let inside = |c: f64| best(gear, point(c)).0 >= 0.0;
    let cs: Vec<f64> = (0..=n).map(|i| lo + (hi - lo) * i as f64 / n as f64).collect();
    let flags: Vec<bool> = cs.iter().map(|&c| inside(c)).collect();
    let edge = |a: f64, b: f64, from_inside: bool| {
        let (mut a, mut b) = (a, b);
        for _ in 0..50 {
            let m = 0.5 * (a + b);
            if inside(m) == from_inside {
                a = m;
            } else {
                b = m;
            }
        }
        0.5 * (a + b)
    };
    let mut out = Vec::new();
    let mut start = if flags[0] { Some(lo) } else { None };
    for i in 0..n {
        match (flags[i], flags[i + 1]) {
            (false, true) => start = Some(edge(cs[i], cs[i + 1], false)),
            (true, false) => {
                if let Some(s) = start.take() {
                    out.push((s, edge(cs[i], cs[i + 1], true)));
                }
            }
            _ => {}
        }
    }
    if let Some(s) = start {
        out.push((s, hi));
    }
    out
}

/// `count` copies of `cell` turned about the z axis by `step` each, joined into one solid where
/// they meet, without a boolean: the faces of `cell` on the surfaces dividing it from its
/// neighbours (every point `divider` holds: its edges' ends and points within them) are left out
/// of every copy, the rest put in one shell and sewn (the kernel's `sew_solid`) along the edges
/// each copy shares with the next. A cell is its neighbour turned, so the faces left out on either
/// side of a divider are the same face, and its edges are each other's.
pub fn pattern_sewn(cell: &BrepSolid, step: f64, count: usize, divider: &dyn Fn(Vec3) -> bool) -> Result<BrepSolid, Refusal> {
    use crate::solids::sweep::{moved, turn_z};
    use std::collections::HashSet;
    let [shell] = cell.shells.as_slice() else {
        return Err(Refusal::from("pattern_sewn: the cell is not one shell"));
    };
    let point = |id: u64| cell.vertices.iter().find(|v| v.id == id).map(|v| v.point);
    let edge = |id: u64| cell.edges.iter().find(|e| e.id == id);
    let on_divider = |face: &brep_kernel::FaceRecord| {
        let mut points = Vec::new();
        for e in face.loops.iter().flat_map(|l| &l.coedges).filter_map(|c| edge(c.edge_id)) {
            points.extend([point(e.start_vertex_id), point(e.end_vertex_id)].into_iter().flatten());
            for k in 1..4 {
                points.extend(e.curve.evaluate(e.t0 + (e.t1 - e.t0) * k as f64 / 4.0).ok());
            }
        }
        !points.is_empty() && points.iter().all(|&q| divider(q))
    };
    let dropped: HashSet<u64> = shell.faces.iter().filter(|f| on_divider(f)).map(|f| f.id).collect();
    if dropped.is_empty() {
        return Err(Refusal::from("pattern_sewn: no face of the cell lies on its dividers"));
    }
    let mut soup = cell.clone();
    soup.shells[0].faces.clear();
    soup.vertices.clear();
    soup.edges.clear();
    // Ids made distinct per copy (a copy's ids are the cell's).
    let span = 1 + cell.vertices.iter().map(|v| v.id).chain(cell.edges.iter().map(|e| e.id)).chain(
        shell.faces.iter().flat_map(|f| std::iter::once(f.id).chain(f.loops.iter().flat_map(|l| std::iter::once(l.id).chain(l.coedges.iter().map(|c| c.id))))),
    ).max().unwrap_or(0);
    for k in 0..count as u64 {
        let copy = moved(cell, turn_z(step * k as f64, Vec3::new(0.0, 0.0, 0.0))?)?;
        let offset = (k + 1) * span;
        let mut used_edges = HashSet::new();
        for face in copy.shells[0].faces.iter().filter(|f| !dropped.contains(&f.id)) {
            let mut f = face.clone();
            f.id += offset;
            for l in &mut f.loops {
                l.id += offset;
                for c in &mut l.coedges {
                    used_edges.insert(c.edge_id);
                    c.id += offset;
                    c.edge_id += offset;
                }
            }
            soup.shells[0].faces.push(f);
        }
        let mut used_vertices = HashSet::new();
        for e in copy.edges.iter().filter(|e| used_edges.contains(&e.id)) {
            let mut e = e.clone();
            used_vertices.extend([e.start_vertex_id, e.end_vertex_id]);
            e.id += offset;
            e.start_vertex_id += offset;
            e.end_vertex_id += offset;
            soup.edges.push(e);
        }
        for v in copy.vertices.iter().filter(|v| used_vertices.contains(&v.id)) {
            let mut v = v.clone();
            v.id += offset;
            soup.vertices.push(v);
        }
    }
    let (sewn, report) = brep_kernel::sew_solid(&soup, 1e-7)?;
    // (A one-use edge left open is a pole's collapsed edge where the cells meet on the axis, which
    // the validator admits; anything else it reports.)
    if let Some(issue) = report.issues.first() {
        return Err(Refusal::from(format!("pattern_sewn: the copies do not sew into one valid solid: {issue}")));
    }
    Ok(sewn)
}
