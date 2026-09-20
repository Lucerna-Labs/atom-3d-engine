//! Bounded geometric embedding validation for finite indexed f32 triangles.
//!
//! Contact is permitted only on the indexed vertex or edge shared by a pair.
//! Conservative BVH boxes select pairs; outward-rounded interval predicates
//! fall back to exact expansions over the original f32 coordinates. This checks
//! the stored mesh, not whether a field extractor omitted an unrelated feature.
use crate::{
    meshing::{MAX_MESH_TRIANGLES, MAX_MESH_VERTICES},
    surface::{TriangleSurface, MAX_SURFACE_TRIANGLES, MAX_SURFACE_VERTICES},
    Vec3,
};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IntersectionReport {
    pub work: usize,
    pub candidate_pairs: usize,
    /// Final pairs covered by an immutable completed baseline no-contact proof.
    pub reused_pairs: usize,
    pub predicate_tests: usize,
    pub exact_predicates: usize,
    pub bvh_chunks: usize,
}
struct Budget {
    maximum: usize,
    report: IntersectionReport,
    failed_pair: Option<[usize; 2]>,
}
impl Budget {
    fn charge(&mut self, amount: usize) -> Result<(), String> {
        if amount > self.maximum.saturating_sub(self.report.work) {
            return Err(format!("surface intersection validation exhausted work budget (used {}, candidate pairs {}, predicates {}, exact {})", self.report.work, self.report.candidate_pairs, self.report.predicate_tests, self.report.exact_predicates));
        }
        self.report.work += amount;
        Ok(())
    }
    fn remaining(&self) -> usize {
        self.maximum - self.report.work
    }
}
/// Traversal has already executed when the query returns. Retain that work
/// even if the query failed or the later candidate-list processing cannot fit.
fn charged_candidates(
    surface: &TriangleSurface,
    min: Vec3,
    max: Vec3,
    context: &str,
    budget: &mut Budget,
) -> Result<Vec<u32>, String> {
    let candidates = match surface.triangle_candidates_counted(min, max, 0., budget.remaining()) {
        Ok(candidates) => {
            budget.charge(candidates.work)?;
            candidates
        }
        Err(error) => {
            budget.charge(error.work)?;
            return Err(format!("{context}: {}", error.message));
        }
    };
    budget.charge(candidates.triangles.len())?;
    Ok(candidates.triangles)
}

fn point(p: Vec3) -> [f64; 3] {
    [p.x, p.y, p.z].map(f64::from)
}
#[derive(Clone, Copy)]
struct Interval {
    lo: f64,
    hi: f64,
}
impl Interval {
    fn exact(v: f64) -> Self {
        Self { lo: v, hi: v }
    }
    fn add(self, b: Self) -> Self {
        Self { lo: (self.lo + b.lo).next_down(), hi: (self.hi + b.hi).next_up() }
    }
    fn sub(self, b: Self) -> Self {
        Self { lo: (self.lo - b.hi).next_down(), hi: (self.hi - b.lo).next_up() }
    }
    fn mul(self, b: Self) -> Self {
        let v = [self.lo * b.lo, self.lo * b.hi, self.hi * b.lo, self.hi * b.hi];
        Self {
            lo: v.into_iter().fold(f64::INFINITY, f64::min).next_down(),
            hi: v.into_iter().fold(f64::NEG_INFINITY, f64::max).next_up(),
        }
    }
    fn sign(self) -> Option<i8> {
        if self.lo > 0. {
            Some(1)
        } else if self.hi < 0. {
            Some(-1)
        } else {
            None
        }
    }
}
fn grow(expansion: &[f64], mut value: f64, budget: &mut Budget) -> Result<Vec<f64>, String> {
    // One structural unit per error-free accumulation and emitted component.
    budget.charge(expansion.len() + 1)?;
    let mut out = Vec::with_capacity(expansion.len() + 1);
    for &component in expansion {
        let sum = value + component;
        let vc = sum - value;
        let vv = sum - vc;
        let error = (value - vv) + (component - vc);
        if error != 0. {
            out.push(error);
        }
        value = sum;
    }
    if value != 0. {
        out.push(value);
    }
    Ok(out)
}
fn expansion_sign(v: &[f64]) -> i8 {
    v.last().map_or(0, |v| if *v < 0. { -1 } else { 1 })
}
fn orient2(a: Vec3, b: Vec3, c: Vec3, axes: [usize; 2], budget: &mut Budget) -> Result<i8, String> {
    budget.charge(3)?;
    budget.report.predicate_tests += 1;
    if a == b || b == c || c == a {
        return Ok(0);
    }
    let [x, y] = axes;
    let [a, b, c] = [a, b, c].map(point);
    budget.charge(2)?;
    if (a[x] == b[x] && b[x] == c[x]) || (a[y] == b[y] && b[y] == c[y]) {
        return Ok(0);
    }
    budget.charge(7)?;
    let i = |v| Interval::exact(v);
    let det = i(b[x]).sub(i(a[x])).mul(i(c[y]).sub(i(a[y]))).sub(i(b[y]).sub(i(a[y])).mul(i(c[x]).sub(i(a[x]))));
    if let Some(sign) = det.sign() {
        return Ok(sign);
    }
    budget.report.exact_predicates += 1;
    // Each term is a product of TWO original f32 values: it is exact in f64.
    let mut expansion = Vec::new();
    for term in [a[x] * b[y], b[x] * c[y], c[x] * a[y], -a[y] * b[x], -b[y] * c[x], -c[y] * a[x]] {
        expansion = grow(&expansion, term, budget)?;
    }
    Ok(expansion_sign(&expansion))
}
fn orient3(a: Vec3, b: Vec3, c: Vec3, d: Vec3, budget: &mut Budget) -> Result<i8, String> {
    budget.charge(6)?;
    budget.report.predicate_tests += 1;
    if a == b || a == c || a == d || b == c || b == d || c == d {
        return Ok(0);
    }
    let p = [a, b, c, d].map(point);
    budget.charge(3)?;
    if (0..3).any(|axis| p[1..].iter().all(|q| q[axis] == p[0][axis])) {
        return Ok(0);
    }
    budget.charge(23)?;
    let v: [[Interval; 3]; 3] = std::array::from_fn(|row| {
        std::array::from_fn(|axis| Interval::exact(p[row][axis]).sub(Interval::exact(p[3][axis])))
    });
    let cross = [
        v[1][1].mul(v[2][2]).sub(v[1][2].mul(v[2][1])),
        v[1][2].mul(v[2][0]).sub(v[1][0].mul(v[2][2])),
        v[1][0].mul(v[2][1]).sub(v[1][1].mul(v[2][0])),
    ];
    let det = v[0][0].mul(cross[0]).add(v[0][1].mul(cross[1])).add(v[0][2].mul(cross[2]));
    if let Some(sign) = det.sign() {
        return Ok(sign);
    }
    budget.report.exact_predicates += 1;
    orient3_expansion(p, budget)
}

/// Exact `a-b` as a high and low pair. Both inputs are converted finite f32;
/// their differences and all intermediates therefore remain finite binary64.
fn difference(a: f64, b: f64, budget: &mut Budget) -> Result<[f64; 2], String> {
    budget.charge(6)?;
    let high = a - b;
    let b_virtual = a - high;
    let a_virtual = high + b_virtual;
    let b_roundoff = b_virtual - b;
    let a_roundoff = a - a_virtual;
    let low = a_roundoff + b_roundoff;
    Ok([low, high])
}

/// Exact six-term determinant of error-free coordinate differences. Each
/// difference component is an integer multiple of 2^-149 with magnitude less
/// than 2^129. Products and FMA residuals through degree three have least bits
/// no lower than 2^-447 and magnitude below 2^387, safely inside binary64.
/// No rounded difference or rounded intermediate product is used on its own.
fn orient3_expansion(p: [[f64; 3]; 4], budget: &mut Budget) -> Result<i8, String> {
    let mut differences = [[[0.; 2]; 3]; 3];
    for row in 0..3 {
        for axis in 0..3 {
            differences[row][axis] = difference(p[row][axis], p[3][axis], budget)?;
        }
    }
    let mut expansion = Vec::new();
    for (axes, sign) in
        [([0, 1, 2], 1.), ([1, 2, 0], 1.), ([2, 0, 1], 1.), ([0, 2, 1], -1.), ([1, 0, 2], -1.), ([2, 1, 0], -1.)]
    {
        // Iterate the exact low/high components; zero terms require no product
        // or expansion allocation. Every inspected component is still charged.
        for a in differences[0][axes[0]] {
            budget.charge(1)?;
            if a == 0. {
                continue;
            }
            for b in differences[1][axes[1]] {
                budget.charge(1)?;
                if b == 0. {
                    continue;
                }
                budget.charge(2)?;
                let high = a * b;
                let low = a.mul_add(b, -high);
                for c in differences[2][axes[2]] {
                    budget.charge(1)?;
                    if c == 0. {
                        continue;
                    }
                    // BOTH parts of the first product are multiplied by c,
                    // and BOTH residuals are retained before accumulation.
                    for component in [low, high] {
                        budget.charge(1)?;
                        if component == 0. {
                            continue;
                        }
                        budget.charge(2)?;
                        let high = component * c;
                        let low = component.mul_add(c, -high);
                        for term in [low, high] {
                            budget.charge(1)?;
                            if term != 0. {
                                expansion = grow(&expansion, sign * term, budget)?;
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(expansion_sign(&expansion))
}

fn axes(triangle: [Vec3; 3], budget: &mut Budget) -> Result<[usize; 2], String> {
    for axes in [[0, 1], [1, 2], [2, 0]] {
        if orient2(triangle[0], triangle[1], triangle[2], axes, budget)? != 0 {
            return Ok(axes);
        }
    }
    Err("surface intersection input contains an exactly degenerate triangle".into())
}
fn same_side(signs: [i8; 3]) -> bool {
    signs.iter().all(|s| *s > 0) || signs.iter().all(|s| *s < 0)
}
fn consistent(signs: [i8; 3]) -> bool {
    signs.iter().all(|s| *s >= 0) || signs.iter().all(|s| *s <= 0)
}
fn inside(p: Vec3, t: [Vec3; 3], axes: [usize; 2], budget: &mut Budget) -> Result<bool, String> {
    let mut s = [0; 3];
    for i in 0..3 {
        s[i] = orient2(t[i], t[(i + 1) % 3], p, axes, budget)?;
    }
    Ok(consistent(s))
}
fn allowed(p: Vec3, common: Option<Vec3>) -> bool {
    common == Some(p)
}
fn on_segment(p: Vec3, a: Vec3, b: Vec3, axes: [usize; 2], budget: &mut Budget) -> Result<bool, String> {
    budget.charge(1)?;
    let [p, a, b] = [p, a, b].map(point);
    Ok(axes.into_iter().all(|i| p[i] >= a[i].min(b[i]) && p[i] <= a[i].max(b[i])))
}
fn segments_bad(
    a: Vec3,
    b: Vec3,
    c: Vec3,
    d: Vec3,
    axes: [usize; 2],
    common: Option<Vec3>,
    budget: &mut Budget,
) -> Result<bool, String> {
    let s = [
        orient2(a, b, c, axes, budget)?,
        orient2(a, b, d, axes, budget)?,
        orient2(c, d, a, axes, budget)?,
        orient2(c, d, b, axes, budget)?,
    ];
    if s[0] * s[1] < 0 && s[2] * s[3] < 0 {
        return Ok(true);
    }
    for (sign, p, x, y) in [(s[0], c, a, b), (s[1], d, a, b), (s[2], a, c, d), (s[3], b, c, d)] {
        if sign == 0 && on_segment(p, x, y, axes, budget)? && !allowed(p, common) {
            return Ok(true);
        }
    }
    Ok(false)
}
fn coplanar_bad(a: [Vec3; 3], b: [Vec3; 3], common: Option<Vec3>, budget: &mut Budget) -> Result<bool, String> {
    let axes = axes(a, budget)?;
    // An edge half-plane can separate the complete convex triangles before
    // their individual vertices and segments need contact classification.
    // Equality is permitted here only at the declared shared indexed vertex.
    // In every other case retain the full contact tests below.
    for (own, other) in [(a, b), (b, a)] {
        let winding = orient2(own[0], own[1], own[2], axes, budget)?;
        for edge in 0..3 {
            let mut separated = true;
            for point in other {
                let sign = orient2(own[edge], own[(edge + 1) % 3], point, axes, budget)?;
                if sign * winding >= 0 && !(sign == 0 && allowed(point, common)) {
                    separated = false;
                    break;
                }
            }
            if separated {
                return Ok(false);
            }
        }
    }
    for (points, other) in [(a, b), (b, a)] {
        for p in points {
            if !allowed(p, common) && inside(p, other, axes, budget)? {
                return Ok(true);
            }
        }
    }
    for i in 0..3 {
        for j in 0..3 {
            if segments_bad(a[i], a[(i + 1) % 3], b[j], b[(j + 1) % 3], axes, common, budget)? {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
fn edges_bad(
    a: [Vec3; 3],
    b: [Vec3; 3],
    signs: [i8; 3],
    common: Option<Vec3>,
    budget: &mut Budget,
) -> Result<bool, String> {
    let axes = axes(b, budget)?;
    for i in 0..3 {
        let j = (i + 1) % 3;
        if signs[i] == 0 && !allowed(a[i], common) && inside(a[i], b, axes, budget)? {
            return Ok(true);
        }
        if signs[i] * signs[j] < 0 {
            let mut s = [0; 3];
            for k in 0..3 {
                s[k] = orient3(a[i], a[j], b[k], b[(k + 1) % 3], budget)?;
            }
            if consistent(s) {
                return Ok(true);
            }
        } else if signs[i] == 0 && signs[j] == 0 {
            for k in 0..3 {
                if segments_bad(a[i], a[j], b[k], b[(k + 1) % 3], axes, common, budget)? {
                    return Ok(true);
                }
            }
        }
    }
    Ok(false)
}
fn bad_pair(positions: &[Vec3], a: [u32; 3], b: [u32; 3], budget: &mut Budget) -> Result<bool, String> {
    budget.charge(1)?;
    budget.report.candidate_pairs += 1;
    let common: Vec<_> = a.into_iter().filter(|i| b.contains(i)).collect();
    if common.len() == 3 {
        return Ok(true);
    }
    if common.len() == 2 {
        let x = positions[common[0] as usize];
        let y = positions[common[1] as usize];
        let a = positions[*a.iter().find(|i| !common.contains(i)).unwrap() as usize];
        let b = positions[*b.iter().find(|i| !common.contains(i)).unwrap() as usize];
        if orient3(x, y, a, b, budget)? != 0 {
            return Ok(false);
        }
        let axes = axes([x, y, a], budget)?;
        return Ok(orient2(x, y, a, axes, budget)? == orient2(x, y, b, axes, budget)?);
    }
    let common = common.first().map(|i| positions[*i as usize]);
    let a = a.map(|i| positions[i as usize]);
    let b = b.map(|i| positions[i as usize]);
    let mut sa = [0; 3];
    let mut sb = [0; 3];
    for i in 0..3 {
        sa[i] = orient3(b[0], b[1], b[2], a[i], budget)?;
    }
    if same_side(sa) {
        return Ok(false);
    }
    if sa == [0; 3] {
        return coplanar_bad(a, b, common, budget);
    }
    if consistent(sa) {
        // A lies in a closed halfspace: intersection can only occur on its
        // zero vertex/edge, so no reverse plane tests or unrelated edges are needed.
        if sa.iter().filter(|&&sign| sign == 0).count() == 1 {
            let p = a[sa.iter().position(|&sign| sign == 0).unwrap()];
            return Ok(!allowed(p, common) && inside(p, b, axes(b, budget)?, budget)?);
        }
        return edges_bad(a, b, sa, common, budget);
    }
    for i in 0..3 {
        sb[i] = orient3(a[0], a[1], a[2], b[i], budget)?;
    }
    if same_side(sb) {
        return Ok(false);
    }
    if consistent(sb) {
        if sb.iter().filter(|&&sign| sign == 0).count() == 1 {
            let p = b[sb.iter().position(|&sign| sign == 0).unwrap()];
            return Ok(!allowed(p, common) && inside(p, a, axes(a, budget)?, budget)?);
        }
        return edges_bad(b, a, sb, common, budget);
    }
    Ok(edges_bad(a, b, sa, common, budget)? || edges_bad(b, a, sb, common, budget)?)
}
struct Chunk {
    surface: TriangleSurface,
    ids: Vec<usize>,
}
fn make_chunk(
    vertices: Vec<Vec3>,
    triangles: Vec<[u32; 3]>,
    ids: Vec<usize>,
    budget: &mut Budget,
) -> Result<Chunk, String> {
    // BVH setup gets a conservative structural allocation for validation and
    // balanced recursive sorting, separate from actual bounded query visits.
    let levels = usize::BITS - triangles.len().leading_zeros();
    let setup = triangles.len().saturating_mul((levels as usize + 1).pow(2)).saturating_add(vertices.len());
    budget.charge(setup)?;
    let surface = TriangleSurface::new(vertices, triangles, f32::MIN_POSITIVE)
        .map_err(|e| format!("surface intersection BVH cannot represent input: {e}"))?;
    Ok(Chunk { surface, ids })
}
/// Validate individual triangles and build bounded candidate structures.
/// This does not check pair intersections, which may remain in a retry baseline.
fn build_chunks(positions: &[Vec3], triangles: &[[u32; 3]], budget: &mut Budget) -> Result<Vec<Chunk>, String> {
    if positions.is_empty()
        || triangles.is_empty()
        || positions.len() > MAX_MESH_VERTICES
        || triangles.len() > MAX_MESH_TRIANGLES
        || budget.maximum == 0
    {
        return Err("surface intersection input exceeds nonempty geometry/work bounds".into());
    }
    budget.charge(positions.len().saturating_add(triangles.len()))?;
    if positions.iter().any(|p| !point(*p).into_iter().all(f64::is_finite)) {
        return Err("surface intersection positions must be finite".into());
    }
    let mut chunks = Vec::new();
    let mut vertices = Vec::new();
    let mut local = Vec::new();
    let mut ids = Vec::new();
    let mut map = BTreeMap::new();
    for (index, &triangle) in triangles.iter().enumerate() {
        budget.charge(3)?;
        if triangle.iter().any(|i| *i as usize >= positions.len()) {
            return Err("surface intersection triangle index is invalid".into());
        }
        axes(triangle.map(|i| positions[i as usize]), budget)?;
        let added = triangle.iter().filter(|&&i| !map.contains_key(&i)).count();
        if vertices.len() + added > MAX_SURFACE_VERTICES || local.len() == MAX_SURFACE_TRIANGLES {
            chunks.push(make_chunk(
                std::mem::take(&mut vertices),
                std::mem::take(&mut local),
                std::mem::take(&mut ids),
                budget,
            )?);
            map.clear();
        }
        let mut target = [0; 3];
        for (k, i) in triangle.into_iter().enumerate() {
            target[k] = *map.entry(i).or_insert_with(|| {
                let j = vertices.len() as u32;
                vertices.push(positions[i as usize]);
                j
            });
        }
        local.push(target);
        ids.push(index);
    }
    if !local.is_empty() {
        chunks.push(make_chunk(vertices, local, ids, budget)?);
    }
    budget.report.bvh_chunks = chunks.len();
    Ok(chunks)
}
fn validate_inner(
    positions: &[Vec3],
    triangles: &[[u32; 3]],
    budget: &mut Budget,
) -> Result<IntersectionReport, String> {
    let chunks = build_chunks(positions, triangles, budget)?;
    for (i, &triangle) in triangles.iter().enumerate() {
        budget.charge(3)?;
        let p = triangle.map(|j| positions[j as usize]);
        let min = p[0].min(p[1]).min(p[2]);
        let max = p[0].max(p[1]).max(p[2]);
        for chunk in &chunks {
            let candidates =
                charged_candidates(&chunk.surface, min, max, "surface intersection candidate query failed", budget)?;
            for candidate in candidates {
                let j = chunk.ids[candidate as usize];
                if j > i && bad_pair(positions, triangle, triangles[j], budget)? {
                    budget.failed_pair = Some([i, j]);
                    return Err(format!(
                        "surface triangles {i} and {j} intersect or overlap beyond their shared indexed boundary"
                    ));
                }
            }
        }
    }
    Ok(budget.report.clone())
}

/// A failed final-geometry check retains its work even if a caller considers
/// another bounded representation. Pair IDs are present only for a proved
/// forbidden contact, not an input, precision, or exhausted-work rejection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntersectionFailure {
    pub work: usize,
    pub triangles: Option<[usize; 2]>,
    pub message: String,
}
impl std::fmt::Display for IntersectionFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(formatter)
    }
}
impl std::error::Error for IntersectionFailure {}

/// Complete stored-f32 check with counted rejection data for bounded searches.
pub fn validate_counted(
    positions: &[Vec3],
    triangles: &[[u32; 3]],
    max_work: usize,
) -> Result<IntersectionReport, IntersectionFailure> {
    let mut budget = Budget { maximum: max_work, report: IntersectionReport::default(), failed_pair: None };
    validate_inner(positions, triangles, &mut budget).map_err(|message| IntersectionFailure {
        work: budget.report.work,
        triangles: budget.failed_pair,
        message,
    })
}

/// Convenience wrapper for callers that abort after any rejection. Search
/// callers must use `validate_counted` and charge failed attempts as well.
pub fn validate(positions: &[Vec3], triangles: &[[u32; 3]], max_work: usize) -> Result<IntersectionReport, String> {
    validate_counted(positions, triangles, max_work).map_err(|failure| failure.message)
}

/// Immutable geometry used for sparse representation retries. Preparing it
/// validates each source triangle, but intentionally does not certify that the
/// baseline is intersection-free. Its unresolved contacts remain obligations.
pub struct PreparedIntersections {
    pub preparation_work: usize,
    preparation_report: IntersectionReport,
    positions: Vec<Vec3>,
    triangles: Vec<[u32; 3]>,
    chunks: Vec<Chunk>,
    vertex_faces: Vec<Vec<usize>>,
    baseline_proof: std::sync::OnceLock<BaselineProof>,
}
struct BaselineProof {
    // Canonical, sorted pairs from exhaustive validation of this object's
    // private immutable positions/triangles. Never exposed for caller mutation.
    forbidden_pairs: Vec<[usize; 2]>,
    method: EmbeddingProofMethod,
    proof_work: usize,
}
impl std::fmt::Debug for PreparedIntersections {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PreparedIntersections")
            .field("vertices", &self.positions.len())
            .field("triangles", &self.triangles.len())
            .field("preparation_work", &self.preparation_work)
            .finish_non_exhaustive()
    }
}
/// Only the geometry changed from an immutable prepared baseline is certified.
/// Untouched baseline intersections are not covered; full validation is required.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ChangedGeometryReport {
    pub work: usize,
    pub changed_vertices: usize,
    pub changed_triangles: usize,
    pub candidate_pairs: usize,
    pub predicate_tests: usize,
    pub exact_predicates: usize,
}
/// Exhaustive forbidden contacts in one immutable prepared baseline.
/// No partial contact list is returned on a pair or work limit.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BaselineContacts {
    pub pairs: Vec<[usize; 2]>,
    pub work: usize,
    pub candidate_pairs: usize,
    pub predicate_tests: usize,
    pub exact_predicates: usize,
}
impl PreparedIntersections {
    pub fn new(positions: &[Vec3], triangles: &[[u32; 3]], max_work: usize) -> Result<Self, IntersectionFailure> {
        let mut budget = Budget { maximum: max_work, report: IntersectionReport::default(), failed_pair: None };
        let result = (|| {
            let chunks = build_chunks(positions, triangles, &mut budget)?;
            budget.charge(positions.len().saturating_mul(2).saturating_add(triangles.len()))?;
            let mut vertex_faces = vec![Vec::new(); positions.len()];
            for (face, triangle) in triangles.iter().enumerate() {
                budget.charge(3)?;
                for &vertex in triangle {
                    vertex_faces[vertex as usize].push(face);
                }
            }
            Ok(Self {
                preparation_work: budget.report.work,
                preparation_report: budget.report.clone(),
                positions: positions.to_vec(),
                triangles: triangles.to_vec(),
                chunks,
                vertex_faces,
                baseline_proof: std::sync::OnceLock::new(),
            })
        })();
        result.map_err(|message| IntersectionFailure {
            work: budget.report.work,
            triangles: budget.failed_pair,
            message,
        })
    }
    /// Build a fresh private BVH while reusing a complete exact-matching
    /// embedding proof. Match and construction work are charged by this call;
    /// the certificate's original proof work remains provenance only.
    pub fn new_certified(
        positions: &[Vec3],
        triangles: &[[u32; 3]],
        certificate: &EmbeddingCertificate,
        max_work: usize,
    ) -> Result<Self, IntersectionFailure> {
        let matched = certificate.matches(positions, triangles, max_work)?;
        if !matched.matches {
            return Err(IntersectionFailure {
                work: matched.work,
                triangles: None,
                message: "embedding certificate does not match prepared input geometry".into(),
            });
        }
        let mut prepared = Self::new(positions, triangles, max_work - matched.work).map_err(|mut error| {
            error.work += matched.work;
            error
        })?;
        let mut budget = Budget { maximum: max_work, report: IntersectionReport::default(), failed_pair: None };
        budget.charge(matched.work + prepared.preparation_work).map_err(|message| IntersectionFailure {
            work: budget.report.work,
            triangles: None,
            message,
        })?;
        budget.charge(1).map_err(|message| IntersectionFailure {
            work: budget.report.work,
            triangles: None,
            message,
        })?;
        prepared.preparation_work = budget.report.work;
        prepared.preparation_report.work = budget.report.work;
        let _ = prepared.baseline_proof.set(BaselineProof {
            forbidden_pairs: Vec::new(),
            method: certificate.method(),
            proof_work: certificate.original_proof_work(),
        });
        Ok(prepared)
    }
    /// Snapshot the unchanged baseline only after a privately completed empty
    /// contact inventory (or an exact-matched complete certificate) exists.
    pub fn baseline_certificate(&self, max_work: usize) -> Result<CertificateCapture, IntersectionFailure> {
        let mut budget = Budget { maximum: max_work, report: IntersectionReport::default(), failed_pair: None };
        budget.charge(1).map_err(|message| IntersectionFailure {
            work: budget.report.work,
            triangles: None,
            message,
        })?;
        let proof = self.baseline_proof.get().ok_or_else(|| IntersectionFailure {
            work: budget.report.work,
            triangles: None,
            message: "baseline certificate requires a completed private inventory".into(),
        })?;
        if !proof.forbidden_pairs.is_empty() {
            return Err(IntersectionFailure {
                work: budget.report.work,
                triangles: None,
                message: "baseline certificate cannot certify unresolved contacts".into(),
            });
        }
        let mut captured = capture_completed_embedding(
            &self.positions,
            &self.triangles,
            proof.method,
            proof.proof_work,
            max_work - budget.report.work,
        )
        .map_err(|mut error| {
            error.work += budget.report.work;
            error
        })?;
        captured.work += budget.report.work;
        Ok(captured)
    }
    pub fn validate_final_certified(
        &self,
        positions: &[Vec3],
        max_work: usize,
    ) -> Result<CertifiedIntersectionReport, IntersectionFailure> {
        let report = self.validate_final(positions, max_work)?;
        let captured = capture_completed_embedding(
            positions,
            &self.triangles,
            EmbeddingProofMethod::PreparedFinal,
            report.work,
            max_work - report.work,
        )
        .map_err(|mut error| {
            error.work += report.work;
            error
        })?;
        Ok(CertifiedIntersectionReport {
            work: report.work + captured.work,
            certificate_work: captured.work,
            report,
            certificate: captured.certificate,
        })
    }
    pub fn validate_retopologized_certified(
        &self,
        positions: &[Vec3],
        triangles: &[[u32; 3]],
        max_work: usize,
    ) -> Result<CertifiedIntersectionReport, IntersectionFailure> {
        let report = self.validate_retopologized(positions, triangles, max_work)?;
        let captured = capture_completed_embedding(
            positions,
            triangles,
            EmbeddingProofMethod::PreparedRetopologized,
            report.work,
            max_work - report.work,
        )
        .map_err(|mut error| {
            error.work += report.work;
            error
        })?;
        Ok(CertifiedIntersectionReport {
            work: report.work + captured.work,
            certificate_work: captured.work,
            report,
            certificate: captured.certificate,
        })
    }
    /// Exact immutable counters from baseline preparation. Together with an
    /// exhaustive empty `baseline_contacts` result, these account for complete
    /// validation of the unchanged baseline without repeating the same scan.
    pub fn preparation_report(&self) -> &IntersectionReport {
        &self.preparation_report
    }
    /// Enumerate every forbidden baseline pair once, reusing the immutable
    /// BVHs. A driver can retain this complete obligation list while repairing
    /// changed geometry; it must forbid resurrecting already-resolved pairs
    /// and still perform complete validation on its final candidate.
    pub fn baseline_contacts(
        &self,
        max_pairs: usize,
        max_work: usize,
    ) -> Result<BaselineContacts, IntersectionFailure> {
        let mut budget = Budget { maximum: max_work, report: IntersectionReport::default(), failed_pair: None };
        let operation = (|| {
            if max_work == 0 {
                return Err("prepared baseline contact enumeration requires positive work".into());
            }
            let mut pairs = Vec::new();
            for (i, &triangle) in self.triangles.iter().enumerate() {
                budget.charge(3)?;
                let p = triangle.map(|j| self.positions[j as usize]);
                let min = p[0].min(p[1]).min(p[2]);
                let max = p[0].max(p[1]).max(p[2]);
                for chunk in &self.chunks {
                    let candidates = charged_candidates(
                        &chunk.surface,
                        min,
                        max,
                        "prepared baseline contact query failed",
                        &mut budget,
                    )?;
                    for candidate in candidates {
                        let j = chunk.ids[candidate as usize];
                        if j > i && bad_pair(&self.positions, triangle, self.triangles[j], &mut budget)? {
                            if pairs.len() >= max_pairs {
                                return Err(format!("prepared baseline contact enumeration exceeded pair limit {max_pairs}; found at least {} contacts", pairs.len() + 1));
                            }
                            budget.charge(1)?;
                            pairs.push([i, j]);
                        }
                    }
                }
            }
            // Publish only after the full scan AND proof-storage work fit.
            // Repeated inventories still execute/charge the same work; only
            // final validation reads this private reusable certificate.
            budget.charge(pairs.len() + 1)?;
            let _ = self.baseline_proof.set(BaselineProof {
                forbidden_pairs: pairs.clone(),
                method: EmbeddingProofMethod::PreparedBaseline,
                proof_work: self
                    .preparation_report
                    .work
                    .checked_add(budget.report.work)
                    .ok_or("baseline proof provenance work overflow")?,
            });
            Ok(BaselineContacts {
                pairs,
                work: budget.report.work,
                candidate_pairs: budget.report.candidate_pairs,
                predicate_tests: budget.report.predicate_tests,
                exact_predicates: budget.report.exact_predicates,
            })
        })();
        operation.map_err(|message| IntersectionFailure { work: budget.report.work, triangles: None, message })
    }
    /// Complete final validation with a fresh final BVH and complete pair
    /// traversal. A private, successfully completed baseline inventory may
    /// replace a no-contact predicate only when BOTH face coordinate arrays
    /// are bit-identical and that pair was not forbidden in the baseline.
    /// All other pairs and every final triangle are checked afresh.
    pub fn validate_final(
        &self,
        positions: &[Vec3],
        max_work: usize,
    ) -> Result<IntersectionReport, IntersectionFailure> {
        self.validate_final_geometry(positions, &self.triangles, false, max_work)
    }
    /// Complete validation after triangle-index changes with fixed face slots.
    /// Baseline pair proofs are reusable only when BOTH index triples and all
    /// referenced coordinate bits remain unchanged. Fresh candidate boxes and
    /// exact nondegeneracy checks cover the supplied current triangles.
    pub fn validate_retopologized(
        &self,
        positions: &[Vec3],
        triangles: &[[u32; 3]],
        max_work: usize,
    ) -> Result<IntersectionReport, IntersectionFailure> {
        self.validate_final_geometry(positions, triangles, true, max_work)
    }
    fn validate_final_geometry(
        &self,
        positions: &[Vec3],
        triangles: &[[u32; 3]],
        compare_topology: bool,
        max_work: usize,
    ) -> Result<IntersectionReport, IntersectionFailure> {
        let mut budget = Budget { maximum: max_work, report: IntersectionReport::default(), failed_pair: None };
        let operation = (|| {
            if positions.len() != self.positions.len() || max_work == 0 {
                return Err(
                    "prepared final validation requires the fixed baseline vertex count and positive work".into()
                );
            }
            if triangles.len() != self.triangles.len() {
                return Err("prepared retopology validation requires the fixed baseline triangle-slot count".into());
            }
            budget.charge(1)?;
            let proof = self
                .baseline_proof
                .get()
                .ok_or("prepared final validation requires a completed private baseline inventory")?;
            // Input finiteness, exact per-face nondegeneracy and candidate
            // boxes are independently reconstructed for the final positions.
            let chunks = build_chunks(positions, triangles, &mut budget)?;
            budget.charge(triangles.len() * if compare_topology { 6 } else { 3 })?;
            let bits = |p: Vec3| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()];
            let changed: Vec<_> = triangles
                .iter()
                .enumerate()
                .map(|(face, t)| {
                    compare_topology && *t != self.triangles[face]
                        || t.iter().any(|&i| bits(positions[i as usize]) != bits(self.positions[i as usize]))
                })
                .collect();
            let lookup_bound = if proof.forbidden_pairs.is_empty() {
                0
            } else {
                (usize::BITS - (proof.forbidden_pairs.len() - 1).leading_zeros()) as usize + 1
            };
            for (i, &triangle) in triangles.iter().enumerate() {
                budget.charge(3)?;
                let p = triangle.map(|j| positions[j as usize]);
                let min = p[0].min(p[1]).min(p[2]);
                let max = p[0].max(p[1]).max(p[2]);
                for chunk in &chunks {
                    let candidates = charged_candidates(
                        &chunk.surface,
                        min,
                        max,
                        "prepared final candidate query failed",
                        &mut budget,
                    )?;
                    for candidate in candidates {
                        let j = chunk.ids[candidate as usize];
                        if j <= i {
                            continue;
                        }
                        budget.charge(1)?;
                        if !changed[i] && !changed[j] {
                            budget.charge(lookup_bound)?;
                            if proof.forbidden_pairs.binary_search(&[i, j]).is_err() {
                                budget.report.candidate_pairs += 1;
                                budget.report.reused_pairs += 1;
                                continue;
                            }
                        }
                        if bad_pair(positions, triangle, triangles[j], &mut budget)? {
                            budget.failed_pair = Some([i, j]);
                            return Err(format!("surface triangles {i} and {j} intersect or overlap beyond their shared indexed boundary"));
                        }
                    }
                }
            }
            Ok(budget.report.clone())
        })();
        operation.map_err(|message| IntersectionFailure {
            work: budget.report.work,
            triangles: budget.failed_pair,
            message,
        })
    }
    /// Check all faces differing from the original prepared positions, including
    /// previous accepted edits. The fixed topology and cached baseline remain
    /// immutable. A successful result is NOT a complete-mesh certificate.
    pub fn check_candidate(
        &self,
        candidate_positions: &[Vec3],
        max_work: usize,
    ) -> Result<ChangedGeometryReport, IntersectionFailure> {
        let mut budget = Budget { maximum: max_work, report: IntersectionReport::default(), failed_pair: None };
        self.check_inner(candidate_positions, &mut budget).map_err(|message| IntersectionFailure {
            work: budget.report.work,
            triangles: budget.failed_pair,
            message,
        })
    }
    fn check_inner(&self, positions: &[Vec3], budget: &mut Budget) -> Result<ChangedGeometryReport, String> {
        if positions.len() != self.positions.len() || budget.maximum == 0 {
            return Err(
                "prepared surface intersection candidate requires the fixed baseline vertex count and positive work"
                    .into(),
            );
        }
        budget.charge(positions.len().saturating_add(self.triangles.len()))?;
        let mut changed = vec![false; self.triangles.len()];
        let mut changed_vertices = 0;
        for (vertex, (&p, &baseline)) in positions.iter().zip(&self.positions).enumerate() {
            if !point(p).into_iter().all(f64::is_finite) {
                return Err("prepared surface intersection candidate must be finite".into());
            }
            let bits = |p: Vec3| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()];
            if bits(p) != bits(baseline) {
                changed_vertices += 1;
                budget.charge(self.vertex_faces[vertex].len())?;
                for &face in &self.vertex_faces[vertex] {
                    changed[face] = true;
                }
            }
        }
        let mut faces = Vec::new();
        for (face, &is_changed) in changed.iter().enumerate() {
            budget.charge(1)?;
            if is_changed {
                let points = self.triangles[face].map(|i| positions[i as usize]);
                axes(points, budget)?;
                budget.charge(3)?;
                faces.push((face, points[0].min(points[1]).min(points[2]), points[0].max(points[1]).max(points[2])));
            }
        }
        for &(i, min, max) in &faces {
            for chunk in &self.chunks {
                let candidates = charged_candidates(
                    &chunk.surface,
                    min,
                    max,
                    "prepared surface intersection candidate query failed",
                    budget,
                )?;
                for candidate in candidates {
                    let j = chunk.ids[candidate as usize];
                    // Changed targets have moved away from their cached boxes;
                    // they are checked directly below, never queried as stale geometry.
                    if !changed[j] && bad_pair(positions, self.triangles[i], self.triangles[j], budget)? {
                        let pair = [i.min(j), i.max(j)];
                        budget.failed_pair = Some(pair);
                        return Err(format!(
                            "surface triangles {} and {} intersect or overlap beyond their shared indexed boundary",
                            pair[0], pair[1]
                        ));
                    }
                }
            }
        }
        for (index, &(i, lo, hi)) in faces.iter().enumerate() {
            for &(j, other_lo, other_hi) in &faces[index + 1..] {
                budget.charge(3)?;
                if hi.x < other_lo.x
                    || other_hi.x < lo.x
                    || hi.y < other_lo.y
                    || other_hi.y < lo.y
                    || hi.z < other_lo.z
                    || other_hi.z < lo.z
                {
                    continue;
                }
                if bad_pair(positions, self.triangles[i], self.triangles[j], budget)? {
                    budget.failed_pair = Some([i, j]);
                    return Err(format!(
                        "surface triangles {i} and {j} intersect or overlap beyond their shared indexed boundary"
                    ));
                }
            }
        }
        Ok(ChangedGeometryReport {
            work: budget.report.work,
            changed_vertices,
            changed_triangles: faces.len(),
            candidate_pairs: budget.report.candidate_pairs,
            predicate_tests: budget.report.predicate_tests,
            exact_predicates: budget.report.exact_predicates,
        })
    }
}

/// Certified source-coordinate admission for a proposed endpoint contraction.
/// Changed surviving faces have certified positive old/new source-normal dots.
/// This does not check the link condition, metric error allowance, unchanged
/// source embedding/orientation, or final stored f32 embedding.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceContractionReport {
    pub work: usize,
    pub changed_triangles: usize,
    /// Changed surviving triangles with a strictly positive normal-dot lower bound.
    pub source_orientations: usize,
    pub swept_pairs: usize,
    pub final_pairs: usize,
    pub separating_axes: usize,
    /// Adjacent endpoint pairs are checked; adjacent swept trajectories are not.
    pub adjacent_sweeps_certified: bool,
}
type SourceBounds = [[f64; 3]; 2];
type IntervalPoint = [Interval; 3];
type Vector = [f64; 3];
fn source_point(bounds: SourceBounds) -> IntervalPoint {
    std::array::from_fn(|i| Interval { lo: bounds[0][i], hi: bounds[1][i] })
}
fn vector_sub(a: Vector, b: Vector) -> Vector {
    std::array::from_fn(|i| a[i] - b[i])
}
fn vector_cross(a: Vector, b: Vector) -> Vector {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn unit(a: Vector) -> Option<Vector> {
    let length = a[0].hypot(a[1]).hypot(a[2]);
    (length.is_finite() && length > 0.).then(|| a.map(|v| v / length))
}
fn source_normal(triangle: [u32; 3], bounds: &[SourceBounds], budget: &mut Budget) -> Result<IntervalPoint, String> {
    budget.charge(15)?;
    let [a, b, c] = triangle.map(|i| source_point(bounds[i as usize]));
    let u: IntervalPoint = std::array::from_fn(|i| b[i].sub(a[i]));
    let v: IntervalPoint = std::array::from_fn(|i| c[i].sub(a[i]));
    Ok([u[1].mul(v[2]).sub(u[2].mul(v[1])), u[2].mul(v[0]).sub(u[0].mul(v[2])), u[0].mul(v[1]).sub(u[1].mul(v[0]))])
}
fn source_normal_dot(
    original: [u32; 3],
    mapped: [u32; 3],
    bounds: &[SourceBounds],
    budget: &mut Budget,
) -> Result<Interval, String> {
    let a = source_normal(original, bounds, budget)?;
    let b = source_normal(mapped, bounds, budget)?;
    budget.charge(5)?;
    Ok(a[0].mul(b[0]).add(a[1].mul(b[1])).add(a[2].mul(b[2])))
}
fn source_orient3(
    a: IntervalPoint,
    b: IntervalPoint,
    c: IntervalPoint,
    d: IntervalPoint,
    budget: &mut Budget,
) -> Result<Option<i8>, String> {
    budget.charge(32)?;
    let v: [IntervalPoint; 3] = [a, b, c].map(|p| std::array::from_fn(|i| p[i].sub(d[i])));
    let cross = [
        v[1][1].mul(v[2][2]).sub(v[1][2].mul(v[2][1])),
        v[1][2].mul(v[2][0]).sub(v[1][0].mul(v[2][2])),
        v[1][0].mul(v[2][1]).sub(v[1][1].mul(v[2][0])),
    ];
    Ok(v[0][0].mul(cross[0]).add(v[0][1].mul(cross[1])).add(v[0][2].mul(cross[2])).sign())
}
fn source_orient2(
    a: IntervalPoint,
    b: IntervalPoint,
    c: IntervalPoint,
    axes: [usize; 2],
    budget: &mut Budget,
) -> Result<Option<i8>, String> {
    budget.charge(12)?;
    let [x, y] = axes;
    Ok(b[x].sub(a[x]).mul(c[y].sub(a[y])).sub(b[y].sub(a[y]).mul(c[x].sub(a[x]))).sign())
}
fn source_box(ids: &[u32], bounds: &[SourceBounds], budget: &mut Budget) -> Result<SourceBounds, String> {
    budget.charge(ids.len() * 3)?;
    let mut result = [[f64::INFINITY; 3], [f64::NEG_INFINITY; 3]];
    for &id in ids {
        for i in 0..3 {
            result[0][i] = result[0][i].min(bounds[id as usize][0][i]);
            result[1][i] = result[1][i].max(bounds[id as usize][1][i]);
        }
    }
    Ok(result)
}
fn boxes_disjoint(a: SourceBounds, b: SourceBounds, budget: &mut Budget) -> Result<bool, String> {
    budget.charge(3)?;
    Ok((0..3).any(|i| a[1][i] < b[0][i] || b[1][i] < a[0][i]))
}
fn source_dot(p: IntervalPoint, origin: IntervalPoint, axis: Vector, budget: &mut Budget) -> Result<Interval, String> {
    budget.charge(8)?;
    let terms: IntervalPoint = std::array::from_fn(|i| p[i].sub(origin[i]).mul(Interval::exact(axis[i])));
    Ok(terms[0].add(terms[1]).add(terms[2]))
}
fn projection_separates(
    a: &[u32],
    b: &[u32],
    bounds: &[SourceBounds],
    axis: Vector,
    budget: &mut Budget,
) -> Result<bool, String> {
    budget.charge(1)?;
    let Some(axis) = unit(axis) else { return Ok(false) };
    let origin = source_point(bounds[a[0] as usize]);
    let mut ranges = [[f64::INFINITY, f64::NEG_INFINITY]; 2];
    for (ids, range) in [a, b].into_iter().zip(&mut ranges) {
        for &i in ids {
            let v = source_dot(source_point(bounds[i as usize]), origin, axis, budget)?;
            range[0] = range[0].min(v.lo);
            range[1] = range[1].max(v.hi);
        }
    }
    Ok(ranges[0][1] < ranges[1][0] || ranges[1][1] < ranges[0][0])
}
fn hulls_separate(
    a: &[u32],
    b: &[u32],
    bounds: &[SourceBounds],
    mid: &[Vector],
    budget: &mut Budget,
    axes_tested: &mut usize,
) -> Result<bool, String> {
    let mut candidates = vec![[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]];
    let mut face_normals = Vec::new();
    for ids in [a, b] {
        for i in 0..ids.len() {
            for j in i + 1..ids.len() {
                for k in j + 1..ids.len() {
                    budget.charge(1)?;
                    let normal = vector_cross(
                        vector_sub(mid[ids[j] as usize], mid[ids[i] as usize]),
                        vector_sub(mid[ids[k] as usize], mid[ids[i] as usize]),
                    );
                    candidates.push(normal);
                    if let Some(normal) = unit(normal) {
                        face_normals.push(normal);
                    }
                }
            }
        }
    }
    // Coplanar hulls also need IN-PLANE edge normals. Their face normals
    // and edge-cross-edge directions all point out of the shared plane and
    // cannot distinguish disjoint polygons whose XYZ boxes overlap.
    // These approximate normals only propose axes; outward interval ranges
    // still supply the strict separation certificate, with no tolerance.
    for normal in face_normals {
        for ids in [a, b] {
            for i in 0..ids.len() {
                for j in i + 1..ids.len() {
                    budget.charge(1)?;
                    candidates.push(vector_cross(normal, vector_sub(mid[ids[j] as usize], mid[ids[i] as usize])));
                }
            }
        }
    }
    for i in 0..a.len() {
        for j in i + 1..a.len() {
            for k in 0..b.len() {
                for l in k + 1..b.len() {
                    budget.charge(1)?;
                    candidates.push(vector_cross(
                        vector_sub(mid[a[j] as usize], mid[a[i] as usize]),
                        vector_sub(mid[b[l] as usize], mid[b[k] as usize]),
                    ));
                }
            }
        }
    }
    for axis in candidates {
        *axes_tested += 1;
        if projection_separates(a, b, bounds, axis, budget)? {
            return Ok(true);
        }
    }
    Ok(false)
}
fn final_source_pair(
    a: [u32; 3],
    b: [u32; 3],
    bounds: &[SourceBounds],
    mid: &[Vector],
    budget: &mut Budget,
    axes_tested: &mut usize,
) -> Result<bool, String> {
    budget.charge(1)?;
    let common: Vec<_> = a.into_iter().filter(|i| b.contains(i)).collect();
    if common.len() == 3 {
        return Ok(false);
    }
    if common.is_empty() {
        return hulls_separate(&a, &b, bounds, mid, budget, axes_tested);
    }
    let p = |i: u32| source_point(bounds[i as usize]);
    if common.len() == 2 {
        let x = common[0];
        let y = common[1];
        let aa = *a.iter().find(|i| !common.contains(i)).unwrap();
        let bb = *b.iter().find(|i| !common.contains(i)).unwrap();
        if source_orient3(p(x), p(y), p(aa), p(bb), budget)?.is_some() {
            return Ok(true);
        }
        for axes in [[0, 1], [1, 2], [2, 0]] {
            if let (Some(sa), Some(sb)) =
                (source_orient2(p(x), p(y), p(aa), axes, budget)?, source_orient2(p(x), p(y), p(bb), axes, budget)?)
            {
                if sa != sb {
                    return Ok(true);
                }
            }
        }
        return Ok(false);
    }
    let origin = common[0];
    let aa: Vec<_> = a.into_iter().filter(|i| *i != origin).collect();
    let bb: Vec<_> = b.into_iter().filter(|i| *i != origin).collect();
    // A supporting triangle plane can meet the other triangle only at its
    // indexed common vertex when both other vertices lie strictly on one side.
    for (face, other) in [(a, &bb), (b, &aa)] {
        if let (Some(x), Some(y)) = (
            source_orient3(p(face[0]), p(face[1]), p(face[2]), p(other[0]), budget)?,
            source_orient3(p(face[0]), p(face[1]), p(face[2]), p(other[1]), budget)?,
        ) {
            if x == y {
                return Ok(true);
            }
        }
    }
    for (own, other) in [(&aa, &bb), (&bb, &aa)] {
        for edge in 0..2 {
            for axes in [[0, 1], [1, 2], [2, 0]] {
                if let (Some(s), Some(x), Some(y)) = (
                    source_orient2(p(origin), p(own[edge]), p(own[1 - edge]), axes, budget)?,
                    source_orient2(p(origin), p(own[edge]), p(other[0]), axes, budget)?,
                    source_orient2(p(origin), p(own[edge]), p(other[1]), axes, budget)?,
                ) {
                    if s != x && x == y {
                        return Ok(true);
                    }
                }
            }
        }
    }
    // Any strictly separating plane through the exact common vertex is enough.
    // Midpoint-derived directions are merely candidate normals: interval dots,
    // not their approximate construction, establish each admission decision.
    let rays: Vec<_> = aa.iter().chain(&bb).map(|&i| unit(vector_sub(mid[i as usize], mid[origin as usize]))).collect();
    if rays.iter().any(Option::is_none) {
        return Ok(false);
    }
    let rays: Vec<_> = rays.into_iter().map(Option::unwrap).collect();
    let mut candidates = vec![
        [1., 0., 0.],
        [0., 1., 0.],
        [0., 0., 1.],
        std::array::from_fn(|i| rays[0][i] + rays[1][i] - rays[2][i] - rays[3][i]),
    ];
    for i in 0..4 {
        for j in i + 1..4 {
            budget.charge(1)?;
            candidates.push(vector_cross(rays[i], rays[j]));
        }
    }
    for i in 0..2 {
        for j in 2..4 {
            budget.charge(1)?;
            candidates.push(vector_sub(rays[i], rays[j]));
        }
    }
    for axis in candidates {
        *axes_tested += 1;
        budget.charge(1)?;
        let Some(axis) = unit(axis) else { continue };
        let mut signs = [None; 4];
        for (slot, &i) in signs.iter_mut().zip(aa.iter().chain(&bb)) {
            *slot = source_dot(p(i), p(origin), axis, budget)?.sign();
        }
        if let [Some(a), Some(b), Some(c), Some(d)] = signs {
            if a == b && c == d && a != c {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
/// Sufficient interval certificate for source-space endpoint contraction.
///
/// Callers must separately establish valid source topology, the full link
/// condition and the displacement envelope. Changed surviving triangles need
/// strictly positive interval lower bounds on their old/new source-normal dot.
/// This also checks every changed swept hull against nonadjacent original triangles,
/// including source faces removed by contraction, and every affected surviving
/// endpoint pair including adjacent contacts. It does NOT certify adjacent
/// swept trajectories or ambient isotopy. Unresolved intervals reject; a final
/// stored-f32 `validate` call remains required after representation selection.
fn source_contraction_inner(
    intervals: &[SourceBounds],
    triangles: &[[u32; 3]],
    removed: u32,
    retained: u32,
    budget: &mut Budget,
) -> Result<SourceContractionReport, String> {
    if removed == retained
        || removed as usize >= intervals.len()
        || retained as usize >= intervals.len()
        || intervals.len() > MAX_MESH_VERTICES
        || triangles.is_empty()
        || triangles.len() > MAX_MESH_TRIANGLES
        || budget.maximum == 0
    {
        return Err("source contraction interval input exceeds geometry/work bounds".into());
    }
    budget.charge(intervals.len() * 3 + triangles.len() * 3)?;
    let limit = f64::from(f32::MAX) * 2.;
    if intervals.iter().any(|p| {
        (0..3).any(|i| {
            !p[0][i].is_finite()
                || !p[1][i].is_finite()
                || p[0][i] > p[1][i]
                || p[0][i].abs() > limit
                || p[1][i].abs() > limit
        })
    }) {
        return Err("source contraction needs finite ordered coordinate enclosures".into());
    }
    if triangles
        .iter()
        .any(|t| t.iter().any(|i| *i as usize >= intervals.len()) || t[0] == t[1] || t[1] == t[2] || t[2] == t[0])
    {
        return Err("source contraction source triangle indices are invalid".into());
    }
    let mid: Vec<Vector> = intervals.iter().map(|p| std::array::from_fn(|i| p[0][i] * 0.5 + p[1][i] * 0.5)).collect();
    let original_boxes: Vec<_> =
        triangles.iter().map(|t| source_box(t, intervals, budget)).collect::<Result<_, _>>()?;
    let changed: Vec<_> = triangles.iter().enumerate().filter(|(_, t)| t.contains(&removed)).map(|(i, _)| i).collect();
    if changed.is_empty() || !triangles.iter().any(|t| t.contains(&removed) && t.contains(&retained)) {
        return Err("source contraction endpoints do not form a source edge".into());
    }
    let mut report = SourceContractionReport { changed_triangles: changed.len(), ..SourceContractionReport::default() };
    for &i in &changed {
        budget.charge(1)?;
        let original = triangles[i];
        if original.contains(&retained) {
            continue;
        }
        let mapped = original.map(|v| if v == removed { retained } else { v });
        if mapped == original {
            continue;
        }
        let dot = source_normal_dot(original, mapped, intervals, budget)?;
        if dot.lo <= 0. || !dot.lo.is_finite() || !dot.hi.is_finite() {
            return Err(format!("source contraction cannot certify positive source orientation for triangle {i} (normal-dot interval {}..{})", dot.lo, dot.hi));
        }
        report.source_orientations += 1;
    }
    for &i in &changed {
        let mut hull = triangles[i].to_vec();
        if !hull.contains(&retained) {
            hull.push(retained);
        }
        let bounds = source_box(&hull, intervals, budget)?;
        for (j, t) in triangles.iter().enumerate() {
            budget.charge(1)?;
            if t.iter().any(|v| hull.contains(v)) || boxes_disjoint(bounds, original_boxes[j], budget)? {
                continue;
            }
            report.swept_pairs += 1;
            if !hulls_separate(&hull, t, intervals, &mid, budget, &mut report.separating_axes)? {
                return Err(format!(
                    "source contraction cannot certify nonadjacent swept separation for triangles {i} and {j}"
                ));
            }
        }
    }
    let mut surviving = Vec::new();
    for (i, t) in triangles.iter().enumerate() {
        budget.charge(1)?;
        if t.contains(&removed) && t.contains(&retained) {
            continue;
        }
        let next = t.map(|v| if v == removed { retained } else { v });
        surviving.push((i, next, t.contains(&removed), source_box(&next, intervals, budget)?));
    }
    for (index, &(i, a, changed, ab)) in surviving.iter().enumerate() {
        if !changed {
            continue;
        }
        for (other, &(j, b, other_changed, bb)) in surviving.iter().enumerate() {
            budget.charge(1)?;
            if index == other || other_changed && other < index || boxes_disjoint(ab, bb, budget)? {
                continue;
            }
            report.final_pairs += 1;
            if !final_source_pair(a, b, intervals, &mid, budget, &mut report.separating_axes)? {
                return Err(format!("source contraction cannot certify final embedding for triangles {i} and {j}"));
            }
        }
    }
    report.work = budget.report.work;
    Ok(report)
}

/// A rejected proposal still consumed bounded predicate work. Search callers
/// must charge this count before trying another edge or direction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceContractionFailure {
    pub work: usize,
    pub message: String,
}
impl std::fmt::Display for SourceContractionFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(formatter)
    }
}
impl std::error::Error for SourceContractionFailure {}

/// Sufficient interval admission with work retained on BOTH outcomes. This
/// certifies changed-survivor source orientations, nonadjacent sweeps, and all
/// affected final pairs, not adjacent swept trajectories. See `SourceContractionReport` for the scope exclusions.
pub fn validate_source_contraction_counted(
    intervals: &[SourceBounds],
    triangles: &[[u32; 3]],
    removed: u32,
    retained: u32,
    max_work: usize,
) -> Result<SourceContractionReport, SourceContractionFailure> {
    let mut budget = Budget { maximum: max_work, report: IntersectionReport::default(), failed_pair: None };
    source_contraction_inner(intervals, triangles, removed, retained, &mut budget)
        .map_err(|message| SourceContractionFailure { work: budget.report.work, message })
}

/// Convenience wrapper for callers that abort the operation on any rejection.
/// A search that retries candidates must use `validate_source_contraction_counted`.
pub fn validate_source_contraction(
    intervals: &[SourceBounds],
    triangles: &[[u32; 3]],
    removed: u32,
    retained: u32,
    max_work: usize,
) -> Result<SourceContractionReport, String> {
    validate_source_contraction_counted(intervals, triangles, removed, retained, max_work)
        .map_err(|failure| failure.message)
}

#[cfg(test)]
#[path = "surface_intersections_orient3_tests.rs"]
mod orient3_tests;

#[path = "prepared_source_contractions.rs"]
mod prepared_source_contractions;
pub use prepared_source_contractions::{
    PreparedSourceContractions, SourceContractionCommit, SourceContractionSnapshot, SourceContractionTicket,
};

#[path = "embedding_certificate.rs"]
mod embedding_certificate;
pub(crate) use embedding_certificate::capture_completed_embedding;
pub use embedding_certificate::{
    validate_certified, CertificateCapture, CertificateMatch, CertifiedIntersectionReport, EmbeddingCertificate,
    EmbeddingProofMethod,
};
