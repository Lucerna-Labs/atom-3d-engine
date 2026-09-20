//! Explicit bounded static correction after exact polygon coalescing.
//! The frozen triangulation survives through a simplicial vertex map; every
//! contraction has a counted topology/source-embedding gate, and the final
//! stored mesh must pass full embedding and direct correspondence validation.
use super::*;
use crate::meshing_local::{ConvexContraction, ConvexRepresentationPolicy, ConvexRepresentationReport};
use std::cell::RefCell;
use std::collections::{BTreeSet, VecDeque};
#[path = "rounding_constraints.rs"]
mod rounding_constraints;
type SourceComparison<'a> = dyn FnMut(u32, u32, usize, &mut WorkBudget) -> Result<i8, String> + 'a;

#[derive(Clone, Copy)]
struct Face {
    triangle: Option<[u32; 3]>,
    normal: [f64; 3],
    feature: u64,
}
fn stored_valid(face: &Face, positions: &[Vec3]) -> bool {
    let Some(triangle) = face.triangle else { return true };
    let [a, b, c] = triangle.map(|i| doubles(positions[i as usize]));
    dot(cross(sub(b, a), sub(c, a)), face.normal) > 0.
}
fn pool_candidates(vertex: u32, pools: &[SourceRounding], current: Vec3) -> Vec<Vec3> {
    let pool = &pools[vertex as usize];
    let axes: [Vec<f32>; 3] = std::array::from_fn(|i| {
        if pool.lower[i] == pool.upper[i] {
            vec![pool.lower[i]]
        } else {
            vec![pool.lower[i], pool.upper[i]]
        }
    });
    let mut candidates = vec![current];
    for &x in &axes[0] {
        for &y in &axes[1] {
            for &z in &axes[2] {
                let point = Vec3::new(x, y, z);
                if !candidates.contains(&point) {
                    candidates.push(point);
                }
            }
        }
    }
    candidates.sort_by(|a, b| {
        let distance = |p: Vec3| {
            let d = sub(doubles(p), pool.exact);
            dot(d, d)
        };
        distance(*a).total_cmp(&distance(*b)).then_with(|| {
            [a.x.to_bits(), a.y.to_bits(), a.z.to_bits()].cmp(&[b.x.to_bits(), b.y.to_bits(), b.z.to_bits()])
        })
    });
    candidates
}
fn cluster_bound(cluster: &[usize], witnesses: &[usize], pools: &[SourceRounding], point: Vec3) -> f64 {
    cluster.iter().map(|&i| source_displacement_bound(&pools[witnesses[i]], point)).fold(0., f64::max)
}
fn exact_point(
    vertex: u32,
    sources: &[SourcePoint],
    cache: &RefCell<Vec<Option<crate::exact_geometry::Point>>>,
    work: &mut WorkBudget,
) -> Result<crate::exact_geometry::Point, String> {
    charge(work, 1)?;
    if let Some(point) = &cache.borrow()[vertex as usize] {
        return Ok(point.clone());
    }
    let coordinates = ExactCoordinate::all_from_source(&sources[vertex as usize], work)?;
    match crate::exact_geometry::Point::from_expansions(
        [&coordinates[0].numerator, &coordinates[1].numerator, &coordinates[2].numerator],
        &coordinates[0].denominator,
        work.limit - work.used,
    ) {
        Ok(result) => {
            charge(work, result.work)?;
            cache.borrow_mut()[vertex as usize] = Some(result.point.clone());
            Ok(result.point)
        }
        Err(error) => {
            charge(work, error.work)?;
            Err(error.message)
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn round_component(
    positions: &mut [Vec3],
    faces: &[Face],
    incidents: &[BTreeSet<usize>],
    clusters: &[Vec<usize>],
    witnesses: &[usize],
    pools: &[SourceRounding],
    bad: usize,
    limit: f64,
    work: &mut WorkBudget,
) -> Result<crate::meshing_local::ConvexRoundingSearch, String> {
    use rounding_constraints::{ComponentLimit, Limits, Status};
    charge(work, faces.len() * 4 + positions.len())?;
    let mut triangles = Vec::new();
    let mut normals = Vec::new();
    let mut seed = None;
    for (index, face) in faces.iter().enumerate() {
        if let Some(triangle) = face.triangle {
            if index == bad {
                seed = Some(triangles.len());
            }
            triangles.push(triangle);
            normals.push(face.normal);
        }
    }
    let mut candidates = Vec::with_capacity(positions.len());
    for (vertex, &current) in positions.iter().enumerate() {
        if incidents[vertex].is_empty() {
            candidates.push(vec![current]);
            continue;
        }
        charge(work, 8)?;
        let mut domain = Vec::new();
        for point in pool_candidates(vertex as u32, pools, current) {
            charge(work, clusters[vertex].len() * 3)?;
            if cluster_bound(&clusters[vertex], witnesses, pools, point) <= limit {
                domain.push(point);
            }
        }
        candidates.push(domain);
    }
    let result = match rounding_constraints::solve(
        positions,
        &triangles,
        &normals,
        &candidates,
        seed.ok_or("rounding component seed was removed")?,
        Limits {
            max_variables: rounding_constraints::MAX_VARIABLES,
            max_allowed_tuples: rounding_constraints::MAX_ALLOWED_TUPLES,
            max_decisions: rounding_constraints::MAX_DECISIONS,
            max_work: work.limit - work.used,
        },
    ) {
        Ok(result) => {
            charge(work, result.work)?;
            result
        }
        Err(error) => {
            charge(work, error.work)?;
            return Err(error.message);
        }
    };
    let status = match result.status {
        Status::Solved => "solved",
        Status::Unsatisfiable => "unsatisfiable",
        Status::ComponentLimit => "component_limit",
        Status::DecisionLimit => "decision_limit",
    };
    if let Some(assignment) = result.assignment {
        charge(work, faces.len() * 20 + positions.len())?;
        if !stored_valid(&faces[bad], &assignment)
            || faces.iter().any(|face| stored_valid(face, positions) && !stored_valid(face, &assignment))
        {
            return Err("rounding component assignment violated its stored-face contract".into());
        }
        positions.copy_from_slice(&assignment);
    }
    Ok(crate::meshing_local::ConvexRoundingSearch {
        status,
        work: result.work,
        variables: result.variables,
        participants: result.participants,
        constraints: result.constraints,
        allowed_tuples: result.allowed_tuples,
        decisions: result.decisions,
        stable_faces: result.stable_faces,
        component_limit: result.component_limit.map(|limit| match limit {
            ComponentLimit::Variables { required } => ("variables", required),
            ComponentLimit::AllowedTuples { required } => ("allowed_tuples", required),
        }),
    })
}

struct EmbeddingResult {
    report: crate::surface_intersections::IntersectionReport,
    certificate: crate::surface_intersections::EmbeddingCertificate,
    certificate_work: usize,
    work: usize,
    attempts: usize,
    initial_contacts: usize,
}
#[allow(clippy::too_many_arguments)]
fn finish_embedding(
    positions: &mut [Vec3],
    triangles: &[[u32; 3]],
    faces: &[Face],
    incidents: &[BTreeSet<usize>],
    clusters: &[Vec<usize>],
    witnesses: &[usize],
    pools: &[SourceRounding],
    limit: f64,
    work: &mut WorkBudget,
) -> Result<EmbeddingResult, String> {
    let start = work.used;
    charge(work, positions.len())?;
    let baseline = positions.to_vec();
    let prepared =
        match crate::surface_intersections::PreparedIntersections::new(positions, triangles, work.limit - work.used) {
            Ok(prepared) => {
                charge(work, prepared.preparation_work)?;
                prepared
            }
            Err(error) => {
                charge(work, error.work)?;
                return Err(error.message);
            }
        };
    let inventory = match prepared.baseline_contacts(MAX_MESH_TRIANGLES, work.limit - work.used) {
        Ok(report) => {
            charge(work, report.work)?;
            report
        }
        Err(error) => {
            charge(work, error.work)?;
            return Err(error.message);
        }
    };
    if inventory.pairs.is_empty() {
        // Preparation certified every input face; the exhaustive inventory
        // certified every candidate pair. No coordinates have changed, so this
        // is already a complete certificate of the final stored mesh.
        let mut report = prepared.preparation_report().clone();
        report.work += inventory.work;
        report.candidate_pairs += inventory.candidate_pairs;
        report.predicate_tests += inventory.predicate_tests;
        report.exact_predicates += inventory.exact_predicates;
        let captured = match prepared.baseline_certificate(work.limit - work.used) {
            Ok(value) => {
                charge(work, value.work)?;
                value
            }
            Err(error) => {
                charge(work, error.work)?;
                return Err(error.message);
            }
        };
        return Ok(EmbeddingResult {
            report,
            certificate: captured.certificate,
            certificate_work: captured.work,
            work: work.used - start,
            attempts: 0,
            initial_contacts: 0,
        });
    }
    let contacts = inventory.pairs;
    let mut attempts = 0;
    let mut resolved = BTreeSet::new();
    let changed_pair = |pair: [usize; 2], positions: &[Vec3]| {
        pair.into_iter().any(|face| triangles[face].iter().any(|&v| positions[v as usize] != baseline[v as usize]))
    };
    for &pair in &contacts {
        charge(work, 6)?;
        if changed_pair(pair, positions) {
            // Every accumulated change was checked against the entire baseline
            // and all other changed faces. This original obligation is clear.
            resolved.insert(pair);
            continue;
        }
        let vertices: Vec<_> = [triangles[pair[0]], triangles[pair[1]]]
            .into_iter()
            .flatten()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let try_assignment = |assignment: &[(u32, Vec3)],
                              positions: &mut [Vec3],
                              work: &mut WorkBudget,
                              attempts: &mut usize|
         -> Result<bool, String> {
            *attempts += 1;
            let mut affected = BTreeSet::new();
            let mut previous = Vec::new();
            for &(vertex, point) in assignment {
                charge(work, clusters[vertex as usize].len() * 3 + incidents[vertex as usize].len())?;
                if cluster_bound(&clusters[vertex as usize], witnesses, pools, point) > limit {
                    return Ok(false);
                }
                affected.extend(incidents[vertex as usize].iter().copied());
                previous.push((vertex, positions[vertex as usize]));
            }
            charge(work, affected.len() * 10 + (resolved.len() + 1) * 6)?;
            for &(vertex, point) in assignment {
                positions[vertex as usize] = point;
            }
            let valid = affected.iter().all(|&i| stored_valid(&faces[i], positions))
                && changed_pair(pair, positions)
                // The changed-only certificate intentionally skips wholly
                // unchanged baseline pairs. Never resurrect an obligation by
                // returning all its vertices to the original invalid baseline.
                && resolved.iter().all(|&old_pair| changed_pair(old_pair, positions));
            if !valid {
                for (vertex, point) in previous {
                    positions[vertex as usize] = point;
                }
                return Ok(false);
            }
            match prepared.check_candidate(positions, work.limit - work.used) {
                Ok(report) => {
                    charge(work, report.work)?;
                    Ok(true)
                }
                Err(error) => {
                    charge(work, error.work)?;
                    for (vertex, point) in previous {
                        positions[vertex as usize] = point;
                    }
                    if error.triangles.is_none() {
                        Err(format!("{}; stored embedding changed-face phase after {} resolved baseline contacts/{attempts} trials, validation work {}", error.message, resolved.len(), work.used - start))
                    } else {
                        Ok(false)
                    }
                }
            }
        };
        let mut repaired = false;
        'single: for &vertex in &vertices {
            let previous = positions[vertex as usize];
            for point in pool_candidates(vertex, pools, previous) {
                if point != previous && try_assignment(&[(vertex, point)], positions, work, &mut attempts)? {
                    repaired = true;
                    break 'single;
                }
            }
        }
        if !repaired {
            'pairs: for i in 0..vertices.len() {
                for j in i + 1..vertices.len() {
                    let a = vertices[i];
                    let b = vertices[j];
                    let old = [positions[a as usize], positions[b as usize]];
                    for pa in pool_candidates(a, pools, old[0]) {
                        for pb in pool_candidates(b, pools, old[1]) {
                            if pa == old[0] && pb == old[1] {
                                continue;
                            }
                            if try_assignment(&[(a, pa), (b, pb)], positions, work, &mut attempts)? {
                                repaired = true;
                                break 'pairs;
                            }
                        }
                    }
                }
            }
        }
        if !repaired {
            return Err(format!("stored embedding cannot resolve original contact {pair:?} among {attempts} bounded source-rounding assignments after {} resolved contacts out of {}", resolved.len(), contacts.len()));
        }
        charge(work, contacts.len() * 6)?;
        for &contact in &contacts {
            if changed_pair(contact, positions) {
                resolved.insert(contact);
            }
        }
    }
    let certified = match prepared.validate_final_certified(positions, work.limit - work.used) {
        Ok(report) => {
            charge(work, report.work)?;
            report
        }
        Err(error) => {
            charge(work, error.work)?;
            return Err(format!("{}; stored embedding final phase after {} resolved baseline contacts/{attempts} trials, validation work {}", error.message, resolved.len(), work.used - start));
        }
    };
    Ok(EmbeddingResult {
        report: certified.report,
        certificate: certified.certificate,
        certificate_work: certified.certificate_work,
        work: work.used - start,
        attempts,
        initial_contacts: contacts.len(),
    })
}

fn source_polygon(
    perimeter: &[u32],
    pools: &[SourceRounding],
    normal: [f64; 3],
    work: &mut WorkBudget,
    preferred: Option<Vec<[u32; 3]>>,
    mut orient: impl FnMut([u32; 3], [usize; 2], &mut WorkBudget) -> Result<i8, String>,
    mut compare: impl FnMut(u32, u32, usize, &mut WorkBudget) -> Result<i8, String>,
) -> Result<Vec<[u32; 3]>, String> {
    let axis = (0..3).max_by(|&a, &b| normal[a].abs().total_cmp(&normal[b].abs())).unwrap();
    let axes = [(axis + 1) % 3, (axis + 2) % 3];
    let direction = if normal[axis] > 0. { 1 } else { -1 };
    let on_segment =
        |a: u32, b: u32, p: u32, work: &mut WorkBudget, compare: &mut SourceComparison<'_>| -> Result<bool, String> {
            for axis in axes {
                let order = compare(a, b, axis, work)?;
                let pa = compare(p, a, axis, work)?;
                let pb = compare(p, b, axis, work)?;
                if order <= 0 {
                    if pa < 0 || pb > 0 {
                        return Ok(false);
                    }
                } else if pa > 0 || pb < 0 {
                    return Ok(false);
                }
            }
            Ok(true)
        };
    for i in 0..perimeter.len() {
        for j in i + 1..perimeter.len() {
            if j == i + 1 || i == 0 && j + 1 == perimeter.len() {
                continue;
            }
            let [a, b, c, d] = [
                perimeter[i],
                perimeter[(i + 1) % perimeter.len()],
                perimeter[j],
                perimeter[(j + 1) % perimeter.len()],
            ];
            let signs = [
                orient([a, b, c], axes, work)?,
                orient([a, b, d], axes, work)?,
                orient([c, d, a], axes, work)?,
                orient([c, d, b], axes, work)?,
            ];
            if signs[0] * signs[1] < 0 && signs[2] * signs[3] < 0
                || signs[0] == 0 && on_segment(a, b, c, work, &mut compare)?
                || signs[1] == 0 && on_segment(a, b, d, work, &mut compare)?
                || signs[2] == 0 && on_segment(c, d, a, work, &mut compare)?
                || signs[3] == 0 && on_segment(c, d, b, work, &mut compare)?
            {
                return Err("exact source polygon has intersecting boundary edges".into());
            }
        }
    }
    if let Some(triangles) = preferred {
        let mut valid = true;
        for &triangle in &triangles {
            if orient(triangle, axes, work)? * direction <= 0 {
                valid = false;
                break;
            }
        }
        if valid {
            return Ok(triangles);
        }
    }
    let mut perimeter = perimeter.to_vec();
    let mut result = Vec::new();
    while perimeter.len() > 3 {
        let n = perimeter.len();
        let mut best = None;
        for i in 0..n {
            let triangle = [perimeter[(i + n - 1) % n], perimeter[i], perimeter[(i + 1) % n]];
            if orient(triangle, axes, work)? * direction <= 0 {
                continue;
            }
            let mut occupied = false;
            for &point in &perimeter {
                if triangle.contains(&point) {
                    continue;
                }
                let mut inside = true;
                for e in 0..3 {
                    if orient([triangle[e], triangle[(e + 1) % 3], point], axes, work)? * direction < 0 {
                        inside = false;
                        break;
                    }
                }
                if inside {
                    occupied = true;
                    break;
                }
            }
            if occupied {
                continue;
            }
            charge(work, 10)?;
            let [a, b, c] = triangle.map(|i| pools[i as usize].exact);
            let ab = sub(b, a);
            let ac = sub(c, a);
            let bc = sub(c, b);
            let area = cross(ab, ac);
            let sum = dot(ab, ab) + dot(ac, ac) + dot(bc, bc);
            let score = dot(area, area) / (sum * sum);
            if best.is_none_or(|(_, old)| score > old) {
                best = Some((i, score));
            }
        }
        let Some((i, _)) = best else {
            return Err("exact source polygon has no admissible constrained ear".into());
        };
        result.push([perimeter[(i + n - 1) % n], perimeter[i], perimeter[(i + 1) % n]]);
        perimeter.remove(i);
    }
    let last: [u32; 3] = perimeter.try_into().map_err(|_| "source polygon lost its last triangle")?;
    if orient(last, axes, work)? * direction <= 0 {
        return Err("source triangulation leaves a zero or reversed final face".into());
    }
    result.push(last);
    Ok(result)
}

fn interval_orient(
    triangle: [u32; 3],
    axes: [usize; 2],
    pools: &[SourceRounding],
    work: &mut WorkBudget,
) -> Result<Option<i8>, String> {
    charge(work, 24)?;
    let sub_i = |a: [f64; 2], b: [f64; 2]| [(a[0] - b[1]).next_down(), (a[1] - b[0]).next_up()];
    let mul = |a: [f64; 2], b: [f64; 2]| {
        let v = [a[0] * b[0], a[0] * b[1], a[1] * b[0], a[1] * b[1]];
        [
            v.into_iter().fold(f64::INFINITY, f64::min).next_down(),
            v.into_iter().fold(f64::NEG_INFINITY, f64::max).next_up(),
        ]
    };
    let get = |i: u32, axis: usize| [pools[i as usize].bounds[0][axis], pools[i as usize].bounds[1][axis]];
    let [a, b, c] = triangle;
    let [u, v] = axes;
    let det = sub_i(
        mul(sub_i(get(b, u), get(a, u)), sub_i(get(c, v), get(a, v))),
        mul(sub_i(get(b, v), get(a, v)), sub_i(get(c, u), get(a, u))),
    );
    Ok(if det[0] > 0. {
        Some(1)
    } else if det[1] < 0. {
        Some(-1)
    } else {
        None
    })
}

#[allow(clippy::too_many_arguments)]
pub(super) fn run(
    output: &mut Output,
    perimeters: &[Vec<u32>],
    features: &[u64],
    normals: &[[f64; 3]],
    families: &[Vec<usize>],
    pools: &[SourceRounding],
    policy: ConvexRepresentationPolicy,
    work: &mut WorkBudget,
) -> Result<ConvexRepresentationReport, String> {
    charge(work, output.sources.len())?;
    // Interval-certified queries need no integer-coordinate construction.
    // Cache the same exact fallback only for vertices that actually reach it.
    let exact_points = RefCell::new(vec![None; output.sources.len()]);
    let mut faces = Vec::new();
    for (region, perimeter) in perimeters.iter().enumerate() {
        if perimeter.is_empty() {
            continue;
        }
        let stored_triangulation = convex_triangulation(perimeter, &output.positions, normals[region], work)?;
        let triangles = source_polygon(
            perimeter,
            pools,
            normals[region],
            work,
            stored_triangulation,
            |triangle, axes, work| {
                if let Some(sign) = interval_orient(triangle, axes, pools, work)? {
                    return Ok(sign);
                }
                let points = [
                    exact_point(triangle[0], &output.sources, &exact_points, work)?,
                    exact_point(triangle[1], &output.sources, &exact_points, work)?,
                    exact_point(triangle[2], &output.sources, &exact_points, work)?,
                ];
                match crate::exact_geometry::orient2([&points[0], &points[1], &points[2]], axes, work.limit - work.used)
                {
                    Ok(result) => {
                        charge(work, result.work)?;
                        Ok(result.sign)
                    }
                    Err(error) => {
                        charge(work, error.work)?;
                        Err(error.message)
                    }
                }
            },
            |a, b, axis, work| {
                charge(work, 1)?;
                if a == b {
                    return Ok(0);
                }
                let pa = &pools[a as usize];
                let pb = &pools[b as usize];
                if pa.bounds[1][axis] < pb.bounds[0][axis] {
                    return Ok(-1);
                }
                if pa.bounds[0][axis] > pb.bounds[1][axis] {
                    return Ok(1);
                }
                if pa.bounds[0][axis] == pa.bounds[1][axis]
                    && pa.bounds[0][axis] == pb.bounds[0][axis]
                    && pb.bounds[0][axis] == pb.bounds[1][axis]
                {
                    return Ok(0);
                }
                let pa = exact_point(a, &output.sources, &exact_points, work)?;
                let pb = exact_point(b, &output.sources, &exact_points, work)?;
                match crate::exact_geometry::compare_axis(&pa, &pb, axis, work.limit - work.used) {
                    Ok(result) => {
                        charge(work, result.work)?;
                        Ok(result.sign)
                    }
                    Err(error) => {
                        charge(work, error.work)?;
                        Err(error.message)
                    }
                }
            },
        )?;
        for triangle in triangles {
            if faces.len() >= output.limits.map_or(MAX_MESH_TRIANGLES, |v| v[1]) {
                return Err("convex source triangulation exceeded triangle budget".into());
            }
            faces.push(Face { triangle: Some(triangle), normal: normals[region], feature: features[region] });
        }
    }
    let source_triangles: Vec<_> = faces.iter().filter_map(|f| f.triangle).collect();
    let topology = crate::mesh_topology::validate(&source_triangles, work.limit - work.used).map_err(|e| e.message)?;
    charge(work, topology.work)?;
    charge(work, output.positions.len() * 4 + source_triangles.len() * 3)?;
    let mut incidents = vec![BTreeSet::new(); output.positions.len()];
    for (face, triangle) in source_triangles.iter().enumerate() {
        for &vertex in triangle {
            incidents[vertex as usize].insert(face);
        }
    }
    let mut witnesses = Vec::new();
    let mut frozen_index = vec![u32::MAX; output.positions.len()];
    let mut clusters = vec![Vec::new(); output.positions.len()];
    for (vertex, incident) in incidents.iter().enumerate() {
        if incident.is_empty() {
            continue;
        }
        frozen_index[vertex] = witnesses.len() as u32;
        for &source in &families[vertex] {
            clusters[vertex].push(witnesses.len());
            witnesses.push(source);
        }
    }
    let original_enclosures: Vec<_> = witnesses.iter().map(|&i| pools[i].bounds).collect();
    let original_triangles: Vec<_> = source_triangles.iter().map(|t| t.map(|i| frozen_index[i as usize])).collect();
    let mut original_to_final = vec![u32::MAX; witnesses.len()];
    for (vertex, cluster) in clusters.iter().enumerate() {
        for &original in cluster {
            original_to_final[original] = vertex as u32;
        }
    }
    let source_intervals: Vec<_> = pools.iter().map(|p| p.bounds).collect();
    let mut contractions = Vec::new();
    let mut rejected = 0usize;
    let mut source_check_work = 0usize;
    let mut rounding_searches = Vec::new();
    loop {
        charge(work, faces.len())?;
        let Some(bad) = faces.iter().position(|face| !stored_valid(face, &output.positions)) else { break };
        let triangle = faces[bad].triangle.unwrap();
        let mut progress = false;
        // First exhaust source-directed representation choices without changing
        // the frozen simplicial map. Only all-incident positive choices commit.
        for vertex in triangle {
            charge(work, 8 + incidents[vertex as usize].len())?;
            let previous = output.positions[vertex as usize];
            for candidate in pool_candidates(vertex, pools, previous) {
                if candidate == previous {
                    continue;
                }
                charge(work, clusters[vertex as usize].len() * 3 + incidents[vertex as usize].len() * 10)?;
                if cluster_bound(&clusters[vertex as usize], &witnesses, pools, candidate)
                    > policy.max_representation_error_m
                {
                    continue;
                }
                output.positions[vertex as usize] = candidate;
                if stored_valid(&faces[bad], &output.positions)
                    && incidents[vertex as usize].iter().all(|&i| {
                        let old = faces[i]
                            .triangle
                            .unwrap()
                            .map(|v| doubles(if v == vertex { previous } else { output.positions[v as usize] }));
                        let old_valid = dot(cross(sub(old[1], old[0]), sub(old[2], old[0])), faces[i].normal) > 0.;
                        !old_valid || stored_valid(&faces[i], &output.positions)
                    })
                {
                    progress = true;
                    break;
                }
                output.positions[vertex as usize] = previous;
            }
            if progress {
                break;
            }
        }
        if progress {
            continue;
        }
        // A fixed two-vertex neighborhood can resolve coupled rounding while
        // retaining the same frozen triangulation and exact coordinate pools.
        'round_pairs: for i in 0..3 {
            for j in i + 1..3 {
                let a = triangle[i];
                let b = triangle[j];
                let previous = [output.positions[a as usize], output.positions[b as usize]];
                let affected: BTreeSet<_> = incidents[a as usize].union(&incidents[b as usize]).copied().collect();
                charge(work, affected.len() + 16)?;
                let before: Vec<_> =
                    affected.iter().map(|&f| (f, stored_valid(&faces[f], &output.positions))).collect();
                for pa in pool_candidates(a, pools, previous[0]) {
                    for pb in pool_candidates(b, pools, previous[1]) {
                        if pa == previous[0] && pb == previous[1] {
                            continue;
                        }
                        charge(
                            work,
                            (clusters[a as usize].len() + clusters[b as usize].len()) * 3 + affected.len() * 10,
                        )?;
                        if cluster_bound(&clusters[a as usize], &witnesses, pools, pa)
                            > policy.max_representation_error_m
                            || cluster_bound(&clusters[b as usize], &witnesses, pools, pb)
                                > policy.max_representation_error_m
                        {
                            continue;
                        }
                        output.positions[a as usize] = pa;
                        output.positions[b as usize] = pb;
                        if stored_valid(&faces[bad], &output.positions)
                            && before.iter().all(|&(f, valid)| !valid || stored_valid(&faces[f], &output.positions))
                        {
                            progress = true;
                            break 'round_pairs;
                        }
                        output.positions[a as usize] = previous[0];
                        output.positions[b as usize] = previous[1];
                    }
                }
            }
        }
        if progress {
            continue;
        }
        let mut proposals = Vec::new();
        let mut local_edges = BTreeSet::new();
        let mut local_vertices: BTreeSet<_> = triangle.into_iter().collect();
        let mut queue: VecDeque<_> = triangle.into_iter().collect();
        while let Some(vertex) = queue.pop_front() {
            charge(work, incidents[vertex as usize].len() * 3)?;
            for &face in &incidents[vertex as usize] {
                let face = faces[face].triangle.unwrap();
                for &other in &face {
                    if other != vertex {
                        local_edges.insert((vertex.min(other), vertex.max(other)));
                        if !local_vertices.contains(&other) {
                            charge(work, (clusters[vertex as usize].len() + clusters[other as usize].len()) * 3)?;
                            let short = cluster_bound(
                                &clusters[vertex as usize],
                                &witnesses,
                                pools,
                                output.positions[other as usize],
                            ) <= policy.max_representation_error_m
                                || cluster_bound(
                                    &clusters[other as usize],
                                    &witnesses,
                                    pools,
                                    output.positions[vertex as usize],
                                ) <= policy.max_representation_error_m;
                            if short {
                                let mut nearby = false;
                                for &seed in &triangle {
                                    charge(work, clusters[other as usize].len() * 3)?;
                                    if cluster_bound(
                                        &clusters[other as usize],
                                        &witnesses,
                                        pools,
                                        output.positions[seed as usize],
                                    ) <= policy.max_representation_error_m
                                    {
                                        nearby = true;
                                        break;
                                    }
                                }
                                if nearby {
                                    local_vertices.insert(other);
                                    queue.push_back(other);
                                }
                            }
                        }
                    }
                }
            }
        }
        for (a, b) in local_edges {
            for (removed, retained) in [(a, b), (b, a)] {
                charge(work, (clusters[removed as usize].len() + clusters[retained as usize].len()) * 3)?;
                let point = output.positions[retained as usize];
                let bound = cluster_bound(&clusters[removed as usize], &witnesses, pools, point).max(cluster_bound(
                    &clusters[retained as usize],
                    &witnesses,
                    pools,
                    point,
                ));
                if bound <= policy.max_representation_error_m {
                    proposals.push((bound, removed, retained));
                }
            }
        }
        proposals.sort_by(|a, b| a.0.total_cmp(&b.0).then((a.1, a.2).cmp(&(b.1, b.2))));
        let mut last_rejection = String::from("no source edge fits the cumulative representation error bound");
        let mut proposal_rejections = Vec::new();
        let mut link_valid = BTreeSet::new();
        let mut source_rejected = BTreeSet::new();
        'proposals: for (_, removed, retained) in proposals.iter().copied() {
            charge(work, faces.len())?;
            let current: Vec<_> = faces.iter().filter_map(|f| f.triangle).collect();
            match crate::mesh_topology::can_collapse(&current, removed, retained, work.limit - work.used) {
                Ok(report) => {
                    charge(work, report.work)?;
                    link_valid.insert((removed, retained));
                }
                Err(error) => {
                    charge(work, error.work)?;
                    rejected += 1;
                    last_rejection = error.message;
                    proposal_rejections.push((removed, retained, last_rejection.clone()));
                    continue;
                }
            }
            let affected: BTreeSet<_> =
                incidents[removed as usize].union(&incidents[retained as usize]).copied().collect();
            charge(work, affected.len() * 12)?;
            let mut replacement = Vec::new();
            for &index in &affected {
                let old = faces[index].triangle.unwrap();
                let mapped = old.map(|i| if i == removed { retained } else { i });
                if mapped[0] == mapped[1] || mapped[1] == mapped[2] || mapped[2] == mapped[0] {
                    replacement.push((index, None));
                    continue;
                }
                if mapped == old {
                    replacement.push((index, Some(mapped)));
                    continue;
                }
                replacement.push((index, Some(mapped)));
            }
            let previous = output.positions[retained as usize];
            for candidate in pool_candidates(retained, pools, previous) {
                charge(
                    work,
                    (clusters[removed as usize].len() + clusters[retained as usize].len()) * 3 + replacement.len() * 10,
                )?;
                let bound = cluster_bound(&clusters[removed as usize], &witnesses, pools, candidate)
                    .max(cluster_bound(&clusters[retained as usize], &witnesses, pools, candidate));
                if bound > policy.max_representation_error_m {
                    continue;
                }
                output.positions[retained as usize] = candidate;
                let valid = replacement.iter().all(|&(index, triangle)| {
                    let Some(triangle) = triangle else { return true };
                    if candidate == previous && Some(triangle) == faces[index].triangle {
                        return true;
                    }
                    let stored = triangle.map(|i| output.positions[i as usize]);
                    let old = faces[index].triangle.unwrap().map(|i| {
                        if i == retained {
                            previous
                        } else {
                            output.positions[i as usize]
                        }
                    });
                    if stored == old {
                        return true;
                    }
                    let [a, b, c] = stored.map(doubles);
                    dot(cross(sub(b, a), sub(c, a)), faces[index].normal) > 0.
                });
                if !valid {
                    output.positions[retained as usize] = previous;
                    continue;
                }
                let source_report = match crate::surface_intersections::validate_source_contraction_counted(
                    &source_intervals,
                    &current,
                    removed,
                    retained,
                    work.limit - work.used,
                ) {
                    Ok(report) => {
                        charge(work, report.work)?;
                        source_check_work += report.work;
                        report
                    }
                    Err(error) => {
                        charge(work, error.work)?;
                        source_check_work += error.work;
                        rejected += 1;
                        source_rejected.insert((removed, retained));
                        last_rejection = error.message;
                        proposal_rejections.push((removed, retained, last_rejection.clone()));
                        output.positions[retained as usize] = previous;
                        // The source endpoint proposal is identical for every
                        // f32 representation of this retained source point.
                        // Retrying it cannot change this certified rejection.
                        continue 'proposals;
                    }
                };
                let supports = output.global_supports.as_mut().unwrap();
                let incoming = supports[removed as usize].clone();
                let additional = incoming.iter().filter(|id| !supports[retained as usize].contains(id)).count();
                if supports[retained as usize].len() + additional > MAX_BOOLEAN_CHANNELS {
                    output.positions[retained as usize] = previous;
                    last_rejection = "representation endpoint exceeds 128 ancestry support IDs".into();
                    proposal_rejections.push((removed, retained, last_rejection.clone()));
                    rejected += 1;
                    continue 'proposals;
                }
                output.support_values -= supports[removed as usize].len();
                supports[removed as usize].clear();
                output.support_values += additional;
                supports[retained as usize].extend(incoming);
                supports[retained as usize].sort_unstable();
                supports[retained as usize].dedup();
                for &(index, mapped) in &replacement {
                    for vertex in faces[index].triangle.unwrap() {
                        incidents[vertex as usize].remove(&index);
                    }
                    faces[index].triangle = mapped;
                    if let Some(mapped) = mapped {
                        for vertex in mapped {
                            incidents[vertex as usize].insert(index);
                        }
                    }
                }
                let ancestry = std::mem::take(&mut clusters[removed as usize]);
                for &original in &ancestry {
                    original_to_final[original] = retained;
                }
                clusters[retained as usize].extend(ancestry);
                contractions.push(ConvexContraction {
                    removed_vertex: removed,
                    retained_vertex: retained,
                    max_displacement_m: bound,
                    source_check_work: source_report.work,
                    affected_triangles: replacement.len(),
                });
                progress = true;
                break 'proposals;
            }
            output.positions[retained as usize] = previous;
            last_rejection = "all bounded stored endpoint choices failed surviving-face checks".into();
            proposal_rejections.push((removed, retained, last_rejection.clone()));
        }
        if !progress {
            let result = round_component(
                &mut output.positions,
                &faces,
                &incidents,
                &clusters,
                &witnesses,
                pools,
                bad,
                policy.max_representation_error_m,
                work,
            )?;
            progress = result.status == "solved";
            rounding_searches.push(result);
        }
        if !progress {
            // A tiny source patch can require one topology-preserving
            // contraction and several coordinated rounding choices together.
            // Stage that bounded transaction: no source/topology state commits
            // until both its source certificate and all changed stored faces
            // pass. The exact coordinate pools and ancestry bound are unchanged.
            charge(work, faces.len())?;
            let current: Vec<_> = faces.iter().filter_map(|face| face.triangle).collect();
            for (_, removed, retained) in proposals {
                charge(work, 2)?;
                // No earlier attempt committed, and a failed rounding search
                // has no assignment. The source complex is identical to the
                // one already checked above; reuse those counted decisions.
                if !link_valid.contains(&(removed, retained)) || source_rejected.contains(&(removed, retained)) {
                    continue;
                }
                let supports = output.global_supports.as_ref().unwrap();
                charge(work, supports[removed as usize].len() * supports[retained as usize].len().max(1))?;
                let additional =
                    supports[removed as usize].iter().filter(|id| !supports[retained as usize].contains(id)).count();
                if supports[retained as usize].len() + additional > MAX_BOOLEAN_CHANNELS {
                    rejected += 1;
                    continue;
                }
                let source_report = match crate::surface_intersections::validate_source_contraction_counted(
                    &source_intervals,
                    &current,
                    removed,
                    retained,
                    work.limit - work.used,
                ) {
                    Ok(report) => {
                        charge(work, report.work)?;
                        source_check_work += report.work;
                        report
                    }
                    Err(error) => {
                        charge(work, error.work)?;
                        source_check_work += error.work;
                        rejected += 1;
                        proposal_rejections.push((removed, retained, error.message));
                        continue;
                    }
                };
                charge(work, faces.len() * 4 + output.positions.len() * 4 + witnesses.len())?;
                // Charge the union and every staged incidence edit before
                // cloning or mutation, including attempts later rejected by
                // the finite constraint solver.
                charge(
                    work,
                    (incidents[removed as usize].len() + incidents[retained as usize].len()) * 32
                        + clusters[removed as usize].len(),
                )?;
                let mut staged_faces = faces.clone();
                let mut staged_positions = output.positions.clone();
                let mut staged_incidents = incidents.clone();
                let mut staged_clusters = clusters.clone();
                let affected: BTreeSet<_> =
                    incidents[removed as usize].union(&incidents[retained as usize]).copied().collect();
                for &index in &affected {
                    let old = faces[index].triangle.unwrap();
                    for vertex in old {
                        staged_incidents[vertex as usize].remove(&index);
                    }
                    let mapped = old.map(|vertex| if vertex == removed { retained } else { vertex });
                    staged_faces[index].triangle =
                        if mapped[0] == mapped[1] || mapped[1] == mapped[2] || mapped[2] == mapped[0] {
                            None
                        } else {
                            for vertex in mapped {
                                staged_incidents[vertex as usize].insert(index);
                            }
                            Some(mapped)
                        };
                }
                let ancestry = std::mem::take(&mut staged_clusters[removed as usize]);
                staged_clusters[retained as usize].extend(ancestry);
                let seed =
                    affected.iter().copied().find(|&index| !stored_valid(&staged_faces[index], &staged_positions));
                if let Some(seed) = seed {
                    let result = round_component(
                        &mut staged_positions,
                        &staged_faces,
                        &staged_incidents,
                        &staged_clusters,
                        &witnesses,
                        pools,
                        seed,
                        policy.max_representation_error_m,
                        work,
                    )?;
                    let solved = result.status == "solved";
                    rounding_searches.push(result);
                    if !solved {
                        rejected += 1;
                        continue;
                    }
                }
                charge(work, faces.len() * 25 + staged_clusters[retained as usize].len() * 3)?;
                let valid = faces.iter().zip(&staged_faces).all(|(old, new)| {
                    let Some(mapped) = new.triangle else { return true };
                    let original = old.triangle.unwrap();
                    let changed = mapped != original
                        || mapped.map(|v| staged_positions[v as usize])
                            != original.map(|v| output.positions[v as usize]);
                    !(changed || stored_valid(old, &output.positions)) || stored_valid(new, &staged_positions)
                });
                if !valid {
                    rejected += 1;
                    proposal_rejections.push((
                        removed,
                        retained,
                        "coordinated assignment left a changed stored face invalid".into(),
                    ));
                    continue;
                }
                let bound = cluster_bound(
                    &staged_clusters[retained as usize],
                    &witnesses,
                    pools,
                    staged_positions[retained as usize],
                );
                if bound > policy.max_representation_error_m {
                    return Err("coordinated contraction violated its cumulative source envelope".into());
                }
                let supports = output.global_supports.as_mut().unwrap();
                let incoming = std::mem::take(&mut supports[removed as usize]);
                output.support_values -= incoming.len();
                output.support_values += additional;
                supports[retained as usize].extend(incoming);
                supports[retained as usize].sort_unstable();
                supports[retained as usize].dedup();
                for &original in &clusters[removed as usize] {
                    original_to_final[original] = retained;
                }
                output.positions = staged_positions;
                faces = staged_faces;
                incidents = staged_incidents;
                clusters = staged_clusters;
                contractions.push(ConvexContraction {
                    removed_vertex: removed,
                    retained_vertex: retained,
                    max_displacement_m: bound,
                    source_check_work: source_report.work,
                    affected_triangles: affected.len(),
                });
                progress = true;
                break;
            }
        }
        if !progress {
            return Err(format!("bounded convex representation cannot admit source face {bad} {triangle:?}, source points {:?}: {last_rejection}; local short component {} vertices; directions {:?}; {} contractions, {rejected} rejected proposals; recent {:?}; rounding search {:?}",triangle.map(|i|pools[i as usize].exact),local_vertices.len(),proposal_rejections.iter().take(32).collect::<Vec<_>>(),contractions.len(),contractions.iter().rev().take(16).collect::<Vec<_>>(),rounding_searches.last()));
        }
    }
    charge(work, faces.len())?;
    let final_triangles: Vec<_> = faces.iter().filter_map(|f| f.triangle).collect();
    let final_topology =
        crate::mesh_topology::validate(&final_triangles, work.limit - work.used).map_err(|e| e.message)?;
    charge(work, final_topology.work)?;
    if final_topology.components != topology.components {
        return Err("representation changed source component count".into());
    }
    let before_embedding = work.used;
    let embedding = finish_embedding(
        &mut output.positions,
        &final_triangles,
        &faces,
        &incidents,
        &clusters,
        &witnesses,
        pools,
        policy.max_representation_error_m,
        work,
    ).map_err(|error|format!("{error}; representation work before embedding {before_embedding}, source checker work {source_check_work}, {} contractions",contractions.len()))?;
    let correspondence = crate::mesh_correspondence::certify(
        &original_enclosures,
        &original_triangles,
        &original_to_final,
        &output.positions,
        &final_triangles,
        crate::mesh_correspondence::Options {
            max_error_m: policy.max_representation_error_m,
            max_work: work.limit - work.used,
        },
    )?;
    charge(work, correspondence.work)?;
    for face in &faces {
        let Some(triangle) = face.triangle else {
            continue;
        };
        for vertex in triangle {
            charge(work, 1)?;
            let support = &mut output.global_supports.as_mut().unwrap()[vertex as usize];
            if !support.contains(&face.feature) {
                if support.len() >= MAX_BOOLEAN_CHANNELS
                    || output.support_values >= crate::meshing_local::MAX_LOCAL_SUPPORT_VALUES
                {
                    return Err("represented face ancestry exceeds its support storage bound".into());
                }
                support.push(face.feature);
                support.sort_unstable();
                output.support_values += 1;
            }
        }
    }
    output.triangles = final_triangles;
    output.face_features = Some(faces.iter().filter(|f| f.triangle.is_some()).map(|f| f.feature).collect());
    output.face_source_normals = faces.iter().filter(|f| f.triangle.is_some()).map(|f| f.normal).collect();
    Ok(ConvexRepresentationReport {
        contractions,
        rejected_proposals: rejected,
        source_check_work,
        max_representation_error_m: policy.max_representation_error_m,
        correspondence,
        embedding_validation_work: embedding.work,
        embedding_repair_attempts: embedding.attempts,
        embedding_initial_contacts: embedding.initial_contacts,
        embedding: embedding.report,
        embedding_certificate: embedding.certificate,
        embedding_certificate_work: embedding.certificate_work,
        embedding_certificate_binding_work: 0,
        rounding_searches,
        adjacent_sweeps_certified: false,
    })
}
