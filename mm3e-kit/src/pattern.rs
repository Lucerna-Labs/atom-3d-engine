//! Bounded triangulation of garment outlines, including concavities and holes.
//!
//! Input loops omit a repeated closing vertex. Either winding is accepted: output
//! boundary loops are counterclockwise for the outer contour and clockwise for
//! holes, and triangles are counterclockwise. Original control points occupy the
//! unchanged prefix of `points`, outer first then holes in caller order. No control
//! point is removed, even on a straight edge. Refinement appends edge midpoints.
//!
//! Predicates use finite f64 coordinates normalized about the outline bounds.
//! Distinct contour features closer than 1e-12 of the normalization scale are
//! rejected explicitly as too close; they are never welded or silently removed.
//! Holes must be strictly interior, mutually disjoint and not nested.

use std::collections::BTreeMap;

pub const MAX_PATTERN_VERTICES: usize = 65_536;
const CLEARANCE: f64 = 1e-12;
type Point = [f64; 2];

#[derive(Clone, Debug, PartialEq)]
pub struct Pattern2D {
    pub outer: Vec<Point>,
    pub holes: Vec<Vec<Point>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TriangulationOptions {
    /// Optional maximum length of every final edge, including interior diagonals.
    pub max_edge_length: Option<f64>,
    /// Includes original controls and every appended refinement vertex.
    pub max_vertices: usize,
    /// Strict limit on inspected vertices, edge pairs, containment/ear candidates,
    /// refinement scans and topology entries. Work is charged before each visit.
    pub max_work: usize,
}

impl Default for TriangulationOptions {
    fn default() -> Self {
        Self { max_edge_length: None, max_vertices: 256, max_work: 1_000_000 }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternMesh {
    pub points: Vec<Point>,
    pub triangles: Vec<[u32; 3]>,
    /// Outer loop first, then holes in input order; includes refinement vertices.
    pub boundary_loops: Vec<Vec<u32>>,
    /// Maps each input loop's original control order to stable `points` indices.
    pub control_vertices: Vec<Vec<u32>>,
    pub work: usize,
}

struct Budget {
    used: usize,
    limit: usize,
}

impl Budget {
    fn charge(&mut self) -> Result<(), String> {
        if self.used >= self.limit {
            return Err(format!("pattern triangulation work budget exceeded ({})", self.limit));
        }
        self.used += 1;
        Ok(())
    }
}

impl Pattern2D {
    /// Validate, triangulate and optionally refine without changing the input.
    /// No partial mesh is returned on invalid geometry or budget exhaustion.
    pub fn triangulate(&self, options: TriangulationOptions) -> Result<PatternMesh, String> {
        if !(3..=MAX_PATTERN_VERTICES).contains(&options.max_vertices) {
            return Err(format!("pattern max_vertices must be in 3..={MAX_PATTERN_VERTICES}"));
        }
        if options.max_edge_length.is_some_and(|length| !length.is_finite() || length <= 0.0) {
            return Err("pattern max_edge_length must be finite and positive".into());
        }
        let mut budget = Budget { used: 0, limit: options.max_work };
        let mut total = 0usize;
        for contour in std::iter::once(&self.outer).chain(&self.holes) {
            budget.charge()?;
            if contour.len() < 3 {
                return Err("every pattern contour requires at least three controls".into());
            }
            total = total.checked_add(contour.len()).ok_or("pattern vertex count overflow")?;
            if total > options.max_vertices {
                return Err(format!("pattern controls exceed vertex budget ({})", options.max_vertices));
            }
        }
        let mut points = Vec::with_capacity(total);
        let mut control_vertices = Vec::with_capacity(self.holes.len() + 1);
        let mut lo = [f64::INFINITY; 2];
        let mut hi = [f64::NEG_INFINITY; 2];
        for contour in std::iter::once(&self.outer).chain(&self.holes) {
            let mut indices = Vec::with_capacity(contour.len());
            for &point in contour {
                budget.charge()?;
                if point.iter().any(|v| !v.is_finite()) {
                    return Err("pattern control coordinates must be finite".into());
                }
                for axis in 0..2 {
                    lo[axis] = lo[axis].min(point[axis]);
                    hi[axis] = hi[axis].max(point[axis]);
                }
                indices.push(points.len() as u32);
                points.push(point);
            }
            control_vertices.push(indices);
        }
        let center = [lo[0] * 0.5 + hi[0] * 0.5, lo[1] * 0.5 + hi[1] * 0.5];
        let scale = (hi[0] - center[0])
            .abs()
            .max((lo[0] - center[0]).abs())
            .max((hi[1] - center[1]).abs())
            .max((lo[1] - center[1]).abs());
        if !scale.is_finite() || scale == 0.0 {
            return Err("pattern has zero or unrepresentable coordinate extent".into());
        }
        let mut normalized = Vec::with_capacity(total);
        for &point in &points {
            budget.charge()?;
            let point = [(point[0] - center[0]) / scale, (point[1] - center[1]) / scale];
            if point.iter().any(|v| !v.is_finite()) {
                return Err("pattern coordinate normalization is nonfinite".into());
            }
            normalized.push(point);
        }
        validate_contours(&normalized, &control_vertices, &mut budget)?;
        let mut boundary_loops = control_vertices.clone();
        let mut expected_area = 0.0;
        for (index, contour) in boundary_loops.iter_mut().enumerate() {
            let signed = loop_area(&normalized, contour, &mut budget)?;
            if signed == 0.0 {
                return Err("pattern contour has zero area".into());
            }
            let desired_positive = index == 0;
            if (signed > 0.0) != desired_positive {
                // Keep the caller's first control as the first loop entry.
                contour[1..].reverse();
            }
            expected_area += if desired_positive { signed.abs() } else { -signed.abs() };
        }
        if expected_area <= 0.0 || !expected_area.is_finite() {
            return Err("pattern holes leave no finite positive material area".into());
        }
        let ring = bridge_holes(&normalized, &boundary_loops, &mut budget)?;
        let mut triangles = clip_ears(&normalized, ring, &mut budget)?;
        if let Some(max_edge) = options.max_edge_length {
            let limit = max_edge / scale;
            if limit == 0.0 {
                return Err("pattern edge length is below representable refinement precision".into());
            }
            refine(
                &mut points,
                &mut normalized,
                &mut triangles,
                &mut boundary_loops,
                Refinement { max_edge, max_vertices: options.max_vertices, center, scale },
                &mut budget,
            )?;
        }
        validate_mesh(&normalized, &triangles, &boundary_loops, expected_area, &mut budget)?;
        Ok(PatternMesh { points, triangles, boundary_loops, control_vertices, work: budget.used })
    }
}

fn cross(a: Point, b: Point, c: Point) -> f64 {
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
}

fn distance_squared(a: Point, b: Point) -> f64 {
    (a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)
}

fn segment_distance_squared(p: Point, a: Point, b: Point) -> f64 {
    let denominator = distance_squared(a, b);
    if denominator == 0.0 {
        return distance_squared(p, a);
    }
    let t = (((p[0] - a[0]) * (b[0] - a[0]) + (p[1] - a[1]) * (b[1] - a[1])) / denominator).clamp(0.0, 1.0);
    distance_squared(p, [a[0] + t * (b[0] - a[0]), a[1] + t * (b[1] - a[1])])
}

fn segments_close(a: Point, b: Point, c: Point, d: Point) -> bool {
    let ab_c = cross(a, b, c);
    let ab_d = cross(a, b, d);
    let cd_a = cross(c, d, a);
    let cd_b = cross(c, d, b);
    if ((ab_c > 0.0 && ab_d < 0.0) || (ab_c < 0.0 && ab_d > 0.0))
        && ((cd_a > 0.0 && cd_b < 0.0) || (cd_a < 0.0 && cd_b > 0.0))
    {
        return true;
    }
    let tolerance = CLEARANCE * CLEARANCE;
    segment_distance_squared(a, c, d) <= tolerance
        || segment_distance_squared(b, c, d) <= tolerance
        || segment_distance_squared(c, a, b) <= tolerance
        || segment_distance_squared(d, a, b) <= tolerance
}

fn loop_area(points: &[Point], contour: &[u32], budget: &mut Budget) -> Result<f64, String> {
    let origin = points[contour[0] as usize];
    let mut twice = 0.0;
    for i in 1..contour.len() - 1 {
        budget.charge()?;
        twice += cross(origin, points[contour[i] as usize], points[contour[i + 1] as usize]);
    }
    Ok(twice * 0.5)
}

fn inside(point: Point, points: &[Point], contour: &[u32], budget: &mut Budget) -> Result<bool, String> {
    let mut result = false;
    for i in 0..contour.len() {
        budget.charge()?;
        let (a, b) = (points[contour[i] as usize], points[contour[(i + 1) % contour.len()] as usize]);
        if (a[1] > point[1]) != (b[1] > point[1]) {
            let x = a[0] + (point[1] - a[1]) / (b[1] - a[1]) * (b[0] - a[0]);
            if point[0] < x {
                result = !result;
            }
        }
    }
    Ok(result)
}

fn validate_contours(points: &[Point], loops: &[Vec<u32>], budget: &mut Budget) -> Result<(), String> {
    let mut edges = Vec::with_capacity(points.len());
    for contour in loops {
        for i in 0..contour.len() {
            budget.charge()?;
            let edge = [contour[i], contour[(i + 1) % contour.len()]];
            if distance_squared(points[edge[0] as usize], points[edge[1] as usize]) <= CLEARANCE * CLEARANCE {
                return Err("pattern has repeated or too-close adjacent controls".into());
            }
            edges.push(edge);
        }
    }
    for i in 0..edges.len() {
        for j in i + 1..edges.len() {
            budget.charge()?;
            let [a, b] = edges[i];
            let [c, d] = edges[j];
            let common = if a == c || a == d {
                Some(a)
            } else if b == c || b == d {
                Some(b)
            } else {
                None
            };
            if let Some(shared) = common {
                let p = if a == shared { b } else { a };
                let q = if c == shared { d } else { c };
                if segment_distance_squared(points[p as usize], points[shared as usize], points[q as usize])
                    <= CLEARANCE * CLEARANCE
                    || segment_distance_squared(points[q as usize], points[shared as usize], points[p as usize])
                        <= CLEARANCE * CLEARANCE
                {
                    return Err("pattern adjacent edges overlap or fold back too closely".into());
                }
            } else if segments_close(points[a as usize], points[b as usize], points[c as usize], points[d as usize]) {
                return Err("pattern contours intersect, overlap, touch or are too close".into());
            }
        }
    }
    for i in 1..loops.len() {
        if !inside(points[loops[i][0] as usize], points, &loops[0], budget)? {
            return Err("pattern hole must lie strictly inside the outer contour".into());
        }
        for j in 1..loops.len() {
            budget.charge()?;
            if i != j && inside(points[loops[i][0] as usize], points, &loops[j], budget)? {
                return Err("pattern holes may not overlap or nest".into());
            }
        }
    }
    Ok(())
}

// A bridge may meet an edge only at a shared endpoint, without overlapping it.
fn blocked_bridge(points: &[Point], a: u32, b: u32, c: u32, d: u32) -> bool {
    if a == c || a == d || b == c || b == d {
        let shared = if a == c || a == d { a } else { b };
        let p = if a == shared { b } else { a };
        let q = if c == shared { d } else { c };
        return segment_distance_squared(points[p as usize], points[shared as usize], points[q as usize])
            <= CLEARANCE * CLEARANCE
            || segment_distance_squared(points[q as usize], points[shared as usize], points[p as usize])
                <= CLEARANCE * CLEARANCE;
    }
    segments_close(points[a as usize], points[b as usize], points[c as usize], points[d as usize])
}

fn bridge_holes(points: &[Point], loops: &[Vec<u32>], budget: &mut Budget) -> Result<Vec<u32>, String> {
    let mut ring = loops[0].clone();
    let mut pending = Vec::with_capacity(loops.len() - 1);
    for (index, hole) in loops.iter().enumerate().skip(1) {
        let mut h = 0;
        for i in 1..hole.len() {
            budget.charge()?;
            let p = points[hole[i] as usize];
            let q = points[hole[h] as usize];
            if p[0] < q[0] || (p[0] == q[0] && p[1] < q[1]) {
                h = i;
            }
        }
        pending.push((index, h));
    }
    // Leftmost holes are connected first so unconnected holes cannot hide every
    // outer vertex from an interior hole. This changes no caller-visible IDs/order.
    while !pending.is_empty() {
        let mut next = 0;
        for i in 1..pending.len() {
            budget.charge()?;
            let (a, ah) = pending[i];
            let (b, bh) = pending[next];
            let p = points[loops[a][ah] as usize];
            let q = points[loops[b][bh] as usize];
            if p[0] < q[0] || (p[0] == q[0] && p[1] < q[1]) {
                next = i;
            }
        }
        let (hole_index, preferred) = pending.remove(next);
        let hole = &loops[hole_index];
        let mut bridge = None;
        for offset in 0..hole.len() {
            budget.charge()?;
            let h = (preferred + offset) % hole.len();
            let a = hole[h];
            let mut best: Option<(f64, usize)> = None;
            'candidate: for (index, &b) in ring.iter().enumerate() {
                budget.charge()?;
                let squared = distance_squared(points[a as usize], points[b as usize]);
                if best.is_some_and(|(distance, _)| squared >= distance) {
                    continue;
                }
                for contour in loops.iter().map(Vec::as_slice).chain(std::iter::once(ring.as_slice())) {
                    for i in 0..contour.len() {
                        budget.charge()?;
                        if blocked_bridge(points, a, b, contour[i], contour[(i + 1) % contour.len()]) {
                            continue 'candidate;
                        }
                    }
                }
                let p = points[a as usize];
                let q = points[b as usize];
                let midpoint = [(p[0] + q[0]) * 0.5, (p[1] + q[1]) * 0.5];
                if !inside(midpoint, points, &loops[0], budget)? {
                    continue;
                }
                for other in &loops[1..] {
                    if inside(midpoint, points, other, budget)? {
                        continue 'candidate;
                    }
                }
                best = Some((squared, index));
            }
            if let Some((_, index)) = best {
                bridge = Some((h, index));
                break;
            }
        }
        let (h, index) = bridge.ok_or("pattern hole has no numerically separated visible bridge")?;
        let a = hole[h];
        let mut joined = Vec::with_capacity(ring.len() + hole.len() + 2);
        for &vertex in &ring[..=index] {
            budget.charge()?;
            joined.push(vertex);
        }
        for offset in 0..hole.len() {
            budget.charge()?;
            joined.push(hole[(h + offset) % hole.len()]);
        }
        joined.extend([a, ring[index]]);
        for &vertex in &ring[index + 1..] {
            budget.charge()?;
            joined.push(vertex);
        }
        ring = joined;
    }
    Ok(ring)
}

fn clip_ears(points: &[Point], mut ring: Vec<u32>, budget: &mut Budget) -> Result<Vec<[u32; 3]>, String> {
    let mut triangles = Vec::with_capacity(ring.len() - 2);
    let mut cursor = 0;
    while ring.len() > 3 {
        let mut found = false;
        for offset in 0..ring.len() {
            budget.charge()?;
            let i = (cursor + offset) % ring.len();
            let triangle = [ring[(i + ring.len() - 1) % ring.len()], ring[i], ring[(i + 1) % ring.len()]];
            let [a, b, c] = triangle.map(|v| points[v as usize]);
            if cross(a, b, c) <= 0.0 {
                continue;
            }
            let mut blocked = false;
            for &vertex in &ring {
                budget.charge()?;
                // Bridges have two occurrences of each endpoint, with one ID.
                if triangle.contains(&vertex) {
                    continue;
                }
                let p = points[vertex as usize];
                if cross(a, b, p) >= 0.0 && cross(b, c, p) >= 0.0 && cross(c, a, p) >= 0.0 {
                    blocked = true;
                    break;
                }
            }
            if !blocked {
                triangles.push(triangle);
                ring.remove(i);
                cursor = i % ring.len();
                found = true;
                break;
            }
        }
        if !found {
            return Err("pattern triangulation could not find a nondegenerate constrained ear".into());
        }
    }
    budget.charge()?;
    let triangle = [ring[0], ring[1], ring[2]];
    let [a, b, c] = triangle.map(|v| points[v as usize]);
    if cross(a, b, c) <= 0.0 {
        return Err("pattern triangulation ended with a degenerate or reversed triangle".into());
    }
    triangles.push(triangle);
    Ok(triangles)
}

fn edge_key(a: u32, b: u32) -> [u32; 2] {
    [a.min(b), a.max(b)]
}

struct Refinement {
    max_edge: f64,
    max_vertices: usize,
    center: Point,
    scale: f64,
}

fn refine(
    points: &mut Vec<Point>,
    normalized: &mut Vec<Point>,
    triangles: &mut Vec<[u32; 3]>,
    loops: &mut [Vec<u32>],
    settings: Refinement,
    budget: &mut Budget,
) -> Result<(), String> {
    loop {
        let mut longest: Option<(f64, [u32; 2])> = None;
        for triangle in triangles.iter() {
            for i in 0..3 {
                budget.charge()?;
                let edge = edge_key(triangle[i], triangle[(i + 1) % 3]);
                let squared = distance_squared(normalized[edge[0] as usize], normalized[edge[1] as usize]);
                // Compare the actual returned coordinates to the requested length.
                // Overflow denotes an edge longer than every finite target; its
                // ordering still uses the finite normalized squared distance.
                let a = points[edge[0] as usize];
                let b = points[edge[1] as usize];
                let length = (a[0] - b[0]).hypot(a[1] - b[1]);
                if length > settings.max_edge
                    && longest
                        .is_none_or(|(length, previous)| squared > length || (squared == length && edge < previous))
                {
                    longest = Some((squared, edge));
                }
            }
        }
        let Some((_, edge)) = longest else { break };
        if points.len() >= settings.max_vertices {
            return Err(format!("pattern refinement exceeds vertex budget ({})", settings.max_vertices));
        }
        budget.charge()?;
        let mid = points.len() as u32;
        let p = normalized[edge[0] as usize];
        let q = normalized[edge[1] as usize];
        let a = points[edge[0] as usize];
        let b = points[edge[1] as usize];
        let physical = [a[0] * 0.5 + b[0] * 0.5, a[1] * 0.5 + b[1] * 0.5];
        // Reproject the rounded physical midpoint so every final predicate checks
        // the coordinates actually returned, including at large translations.
        let midpoint =
            [(physical[0] - settings.center[0]) / settings.scale, (physical[1] - settings.center[1]) / settings.scale];
        if midpoint == p || midpoint == q || physical == a || physical == b {
            return Err("pattern refinement midpoint is below representable precision".into());
        }
        normalized.push(midpoint);
        points.push(physical);
        let mut next = Vec::with_capacity(triangles.len() + 2);
        let mut incident = 0;
        for &triangle in triangles.iter() {
            budget.charge()?;
            if let Some(i) = (0..3).find(|&i| edge_key(triangle[i], triangle[(i + 1) % 3]) == edge) {
                let [a, b, c] = [triangle[i], triangle[(i + 1) % 3], triangle[(i + 2) % 3]];
                if cross(normalized[a as usize], midpoint, normalized[c as usize]) <= 0.0
                    || cross(midpoint, normalized[b as usize], normalized[c as usize]) <= 0.0
                {
                    return Err("pattern refinement cannot preserve triangle orientation at available precision".into());
                }
                next.extend([[a, mid, c], [mid, b, c]]);
                incident += 1;
            } else {
                next.push(triangle);
            }
        }
        if incident != 1 && incident != 2 {
            return Err("pattern refinement found a nonmanifold edge".into());
        }
        let mut boundary = 0;
        for contour in loops.iter_mut() {
            let mut insertion = None;
            for i in 0..contour.len() {
                budget.charge()?;
                if edge_key(contour[i], contour[(i + 1) % contour.len()]) == edge {
                    insertion = Some(i + 1);
                    break;
                }
            }
            if let Some(index) = insertion {
                contour.insert(index, mid);
                boundary += 1;
            }
        }
        if boundary != usize::from(incident == 1) {
            return Err("pattern refinement edge disagrees with contour topology".into());
        }
        *triangles = next;
    }
    Ok(())
}

fn validate_mesh(
    points: &[Point],
    triangles: &[[u32; 3]],
    loops: &[Vec<u32>],
    expected_area: f64,
    budget: &mut Budget,
) -> Result<(), String> {
    let mut edges = BTreeMap::<[u32; 2], (usize, i32)>::new();
    let mut referenced = vec![false; points.len()];
    let mut area = 0.0;
    for &triangle in triangles {
        budget.charge()?;
        let [a, b, c] = triangle.map(|v| points[v as usize]);
        let twice = cross(a, b, c);
        if twice <= 0.0 || !twice.is_finite() {
            return Err("pattern output contains a degenerate, reversed or nonfinite triangle".into());
        }
        area += twice * 0.5;
        for i in 0..3 {
            budget.charge()?;
            let (a, b) = (triangle[i], triangle[(i + 1) % 3]);
            referenced[a as usize] = true;
            let entry = edges.entry(edge_key(a, b)).or_default();
            entry.0 += 1;
            entry.1 += if a < b { 1 } else { -1 };
        }
    }
    for contour in loops {
        for i in 0..contour.len() {
            budget.charge()?;
            let (a, b) = (contour[i], contour[(i + 1) % contour.len()]);
            let expected = (1, if a < b { 1 } else { -1 });
            if edges.remove(&edge_key(a, b)) != Some(expected) {
                return Err("pattern output did not preserve a directed contour edge".into());
            }
        }
    }
    for (_, entry) in edges {
        budget.charge()?;
        if entry != (2, 0) {
            return Err("pattern output has an open or nonmanifold interior edge".into());
        }
    }
    for used in referenced {
        budget.charge()?;
        if !used {
            return Err("pattern output discarded a control or refinement vertex".into());
        }
    }
    let tolerance = 128.0 * f64::EPSILON * (triangles.len() + points.len()) as f64;
    if !area.is_finite() || (area - expected_area).abs() > tolerance * expected_area.max(1.0) {
        return Err("pattern output area disagrees with the outlined material".into());
    }
    Ok(())
}
