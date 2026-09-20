//! Isolated finite assignment of caller-certified stored-coordinate candidates.
//!
//! Universally positive face tables are stable borders. Remaining tables couple
//! their non-singleton variables; fixed shared anchors do not join components.
//! A solution covers the complete fragile component containing the seed face.
//! Unrelated baseline failures may remain. No source coordinate, topology, or
//! face list is changed here. Final embedding and correspondence remain caller
//! gates. Solved/Unsatisfiable refer to the nominal finite f64 stored predicate,
//! not an exact rational orientation sign. Nonfinite arithmetic rejects with
//! counted Failure; finite underflow/cancellation retain the stored predicate.
use crate::{
    meshing::{MAX_MESH_TRIANGLES, MAX_MESH_VERTICES},
    Vec3,
};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

pub const MAX_VARIABLES: usize = 64;
pub const MAX_ALLOWED_TUPLES: usize = 50_000;
pub const MAX_DECISIONS: usize = 4096;
const MAX_CANDIDATES: usize = 8;

#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub max_variables: usize,
    pub max_allowed_tuples: usize,
    /// One decision is one attempted candidate value during backtracking.
    pub max_decisions: usize,
    pub max_work: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Solved,
    Unsatisfiable,
    ComponentLimit,
    DecisionLimit,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentLimit {
    Variables { required: usize },
    AllowedTuples { required: usize },
}
#[derive(Clone, Debug, PartialEq)]
pub struct Outcome {
    pub status: Status,
    /// Complete stored positions, including unchanged positions outside the
    /// component. Present only for Solved; never a partial search assignment.
    pub assignment: Option<Vec<Vec3>>,
    pub work: usize,
    pub variables: usize,
    /// Unique vertices in retained fragile constraints, including fixed anchors.
    pub participants: usize,
    pub constraints: usize,
    pub allowed_tuples: usize,
    pub decisions: usize,
    pub stable_faces: usize,
    pub component_limit: Option<ComponentLimit>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Failure {
    /// Spent work must be charged by any caller that tries another component.
    pub work: usize,
    pub message: String,
}
struct Budget {
    used: usize,
    max: usize,
}
impl Budget {
    fn charge(&mut self, amount: usize) -> Result<(), String> {
        if amount > self.max.saturating_sub(self.used) {
            return Err("rounding constraints exhausted work budget".into());
        }
        self.used += amount;
        Ok(())
    }
}
struct Constraint {
    variables: [Option<usize>; 3],
    fixed_masks: [u8; 3],
    allowed: Vec<[u8; 3]>,
}
#[derive(Default)]
struct Component {
    vertices: Vec<usize>,
    participants: BTreeSet<usize>,
    constraints: Vec<Constraint>,
    faces: Vec<usize>,
    allowed_tuples: usize,
    stable_faces: usize,
    decisions: usize,
    component_limit: Option<ComponentLimit>,
}
struct Problem<'a> {
    positions: &'a [Vec3],
    triangles: &'a [[u32; 3]],
    normals: &'a [[f64; 3]],
    candidates: &'a [Vec<Vec3>],
    masks: Vec<u8>,
    incidents: Vec<Vec<usize>>,
}
fn positive(points: [Vec3; 3], normal: [f64; 3], budget: &mut Budget) -> Result<bool, String> {
    budget.charge(44)?;
    let failure = || "rounding constraints stored orientation predicate produced a nonfinite intermediate".to_string();
    let [a, b, c] = points.map(|p| [f64::from(p.x), f64::from(p.y), f64::from(p.z)]);
    let ab = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let ac = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    if ab.iter().chain(&ac).any(|v| !v.is_finite()) {
        return Err(failure());
    }
    let products = [ab[1] * ac[2], ab[2] * ac[1], ab[2] * ac[0], ab[0] * ac[2], ab[0] * ac[1], ab[1] * ac[0]];
    if products.iter().any(|v| !v.is_finite()) {
        return Err(failure());
    }
    let cross = [products[0] - products[1], products[2] - products[3], products[4] - products[5]];
    if cross.iter().any(|v| !v.is_finite()) {
        return Err(failure());
    }
    let terms = [cross[0] * normal[0], cross[1] * normal[1], cross[2] * normal[2]];
    if terms.iter().any(|v| !v.is_finite()) {
        return Err(failure());
    }
    let partial = terms[0] + terms[1];
    let value = partial + terms[2];
    if !partial.is_finite() || !value.is_finite() {
        return Err(failure());
    }
    // Preserve the finite operation order and boolean predicate, including
    // finite cancellation/underflow. This is not an exact sign certificate.
    Ok(value > 0.)
}

fn prepare<'a>(
    positions: &'a [Vec3],
    triangles: &'a [[u32; 3]],
    normals: &'a [[f64; 3]],
    candidates: &'a [Vec<Vec3>],
    seed: usize,
    limits: Limits,
    budget: &mut Budget,
) -> Result<Problem<'a>, String> {
    if positions.is_empty()
        || positions.len() > MAX_MESH_VERTICES
        || triangles.is_empty()
        || triangles.len() > MAX_MESH_TRIANGLES
        || normals.len() != triangles.len()
        || candidates.len() != positions.len()
        || seed >= triangles.len()
    {
        return Err("rounding constraints require bounded geometry and matching input lengths".into());
    }
    if limits.max_variables > MAX_VARIABLES
        || limits.max_allowed_tuples > MAX_ALLOWED_TUPLES
        || limits.max_decisions > MAX_DECISIONS
    {
        return Err("rounding constraints requested limits exceed the fixed bounds".into());
    }
    budget.charge(positions.len() * 2 + triangles.len() * 3)?;
    let mut masks = Vec::with_capacity(positions.len());
    for (vertex, domain) in candidates.iter().enumerate() {
        budget.charge(1)?;
        if domain.is_empty() || domain.len() > MAX_CANDIDATES {
            return Err("rounding constraints require one to eight candidates per vertex".into());
        }
        budget.charge(domain.len() * 4 + 1)?;
        if [positions[vertex].x, positions[vertex].y, positions[vertex].z].iter().any(|v| !v.is_finite())
            || domain.iter().any(|p| [p.x, p.y, p.z].iter().any(|v| !v.is_finite()))
        {
            return Err("rounding constraints positions and candidates must be finite".into());
        }
        if !domain.contains(&positions[vertex]) {
            return Err(format!(
                "rounding constraints current position at vertex {vertex} is absent from its candidates"
            ));
        }
        let mut mask = 0;
        for (index, point) in domain.iter().enumerate() {
            budget.charge(index + 1)?;
            if !domain[..index].contains(point) {
                mask |= 1 << index;
            }
        }
        masks.push(mask);
    }
    let mut incidents = vec![Vec::new(); positions.len()];
    for (face, &triangle) in triangles.iter().enumerate() {
        budget.charge(9)?;
        if triangle.iter().any(|&v| v as usize >= positions.len())
            || triangle[0] == triangle[1]
            || triangle[1] == triangle[2]
            || triangle[2] == triangle[0]
        {
            return Err("rounding constraints triangle indices are invalid".into());
        }
        if normals[face].iter().any(|v| !v.is_finite()) {
            return Err("rounding constraints face normals must be finite".into());
        }
        for vertex in triangle {
            incidents[vertex as usize].push(face);
        }
    }
    Ok(Problem { positions, triangles, normals, candidates, masks, incidents })
}
fn discover(
    problem: &Problem<'_>,
    seed: usize,
    limits: Limits,
    component: &mut Component,
    budget: &mut Budget,
) -> Result<Option<Status>, String> {
    budget.charge(problem.triangles.len() + 1)?;
    let mut seen = vec![false; problem.triangles.len()];
    seen[seed] = true;
    let mut pending = VecDeque::from([seed]);
    component.faces.push(seed);
    let mut variables = BTreeMap::new();
    while let Some(face) = pending.pop_front() {
        budget.charge(1)?;
        let vertices = problem.triangles[face].map(|v| v as usize);
        let domain_masks = vertices.map(|v| problem.masks[v]);
        let total = domain_masks.iter().map(|m| m.count_ones() as usize).product::<usize>();
        let mut allowed = Vec::new();
        for a in 0..problem.candidates[vertices[0]].len() {
            if domain_masks[0] & (1 << a) == 0 {
                continue;
            }
            for b in 0..problem.candidates[vertices[1]].len() {
                if domain_masks[1] & (1 << b) == 0 {
                    continue;
                }
                for c in 0..problem.candidates[vertices[2]].len() {
                    if domain_masks[2] & (1 << c) == 0 {
                        continue;
                    }
                    if positive(
                        [
                            problem.candidates[vertices[0]][a],
                            problem.candidates[vertices[1]][b],
                            problem.candidates[vertices[2]][c],
                        ],
                        problem.normals[face],
                        budget,
                    )? {
                        budget.charge(1)?;
                        allowed.push([a as u8, b as u8, c as u8]);
                    }
                }
            }
        }
        if allowed.len() == total {
            component.stable_faces += 1;
            continue;
        }
        if allowed.is_empty() {
            return Ok(Some(Status::Unsatisfiable));
        }
        budget.charge(6)?;
        let added: Vec<_> =
            vertices.into_iter().filter(|v| problem.masks[*v].count_ones() > 1 && !variables.contains_key(v)).collect();
        if component.vertices.len() + added.len() > limits.max_variables {
            component.component_limit =
                Some(ComponentLimit::Variables { required: component.vertices.len() + added.len() });
            return Ok(Some(Status::ComponentLimit));
        }
        if component.allowed_tuples + allowed.len() > limits.max_allowed_tuples {
            component.component_limit =
                Some(ComponentLimit::AllowedTuples { required: component.allowed_tuples + allowed.len() });
            return Ok(Some(Status::ComponentLimit));
        }
        for vertex in added {
            let index = component.vertices.len();
            variables.insert(vertex, index);
            component.vertices.push(vertex);
            budget.charge(problem.incidents[vertex].len())?;
            for &other in &problem.incidents[vertex] {
                if !seen[other] {
                    seen[other] = true;
                    pending.push_back(other);
                    component.faces.push(other);
                }
            }
        }
        component.participants.extend(vertices);
        component.allowed_tuples += allowed.len();
        component.constraints.push(Constraint {
            variables: vertices.map(|v| variables.get(&v).copied()),
            fixed_masks: domain_masks,
            allowed,
        });
    }
    Ok(None)
}
fn propagate(constraints: &[Constraint], domains: &mut [u8], budget: &mut Budget) -> Result<bool, String> {
    loop {
        let mut changed = false;
        for constraint in constraints {
            budget.charge(1)?;
            let masks = std::array::from_fn::<_, 3, _>(|i| {
                constraint.variables[i].map_or(constraint.fixed_masks[i], |v| domains[v])
            });
            let mut supported = [0_u8; 3];
            for &tuple in &constraint.allowed {
                budget.charge(4)?;
                if (0..3).all(|i| masks[i] & (1 << tuple[i]) != 0) {
                    for i in 0..3 {
                        supported[i] |= 1 << tuple[i];
                    }
                }
            }
            if supported.contains(&0) {
                return Ok(false);
            }
            for (i, variable) in constraint.variables.iter().enumerate() {
                if let Some(variable) = variable {
                    let remaining = domains[*variable] & supported[i];
                    if remaining != domains[*variable] {
                        domains[*variable] = remaining;
                        changed = true;
                    }
                }
            }
        }
        if !changed {
            return Ok(true);
        }
    }
}
enum Search {
    Solved(Vec<u8>),
    Unsatisfiable,
    DecisionLimit,
}
fn search(
    problem: &Problem<'_>,
    component: &Component,
    mut domains: Vec<u8>,
    limits: Limits,
    decisions: &mut usize,
    budget: &mut Budget,
) -> Result<Search, String> {
    if !propagate(&component.constraints, &mut domains, budget)? {
        return Ok(Search::Unsatisfiable);
    }
    budget.charge(domains.len())?;
    let variable = domains
        .iter()
        .enumerate()
        .filter(|(_, mask)| mask.count_ones() > 1)
        .min_by_key(|&(i, mask)| (mask.count_ones(), component.vertices[i]))
        .map(|(i, _)| i);
    let Some(variable) = variable else {
        return Ok(Search::Solved(domains));
    };
    let vertex = component.vertices[variable];
    let current = problem.candidates[vertex].iter().position(|p| p == &problem.positions[vertex]).unwrap();
    let order = std::iter::once(current).chain((0..problem.candidates[vertex].len()).filter(|&i| i != current));
    for value in order {
        budget.charge(1)?;
        if domains[variable] & (1 << value) == 0 {
            continue;
        }
        if *decisions >= limits.max_decisions {
            return Ok(Search::DecisionLimit);
        }
        *decisions += 1;
        budget.charge(domains.len() + 1)?;
        let mut branch = domains.clone();
        branch[variable] = 1 << value;
        match search(problem, component, branch, limits, decisions, budget)? {
            Search::Unsatisfiable => {}
            result => return Ok(result),
        }
    }
    Ok(Search::Unsatisfiable)
}
fn outcome(component: Component, status: Status, assignment: Option<Vec<Vec3>>, budget: &Budget) -> Outcome {
    Outcome {
        status,
        assignment,
        work: budget.used,
        variables: component.vertices.len(),
        participants: component.participants.len(),
        constraints: component.constraints.len(),
        allowed_tuples: component.allowed_tuples,
        decisions: component.decisions,
        stable_faces: component.stable_faces,
        component_limit: component.component_limit,
    }
}
/// Solve only the connected fragile component. All input domains must contain
/// their current position so fixed outside coordinates satisfy the stable-border
/// quantification. Candidate coordinates are never expanded or invented. Duplicate
/// candidate values are collapsed internally without changing the allowed set.
pub fn solve(
    positions: &[Vec3],
    triangles: &[[u32; 3]],
    normals: &[[f64; 3]],
    candidates: &[Vec<Vec3>],
    seed_face: usize,
    limits: Limits,
) -> Result<Outcome, Failure> {
    let mut budget = Budget { used: 0, max: limits.max_work };
    let operation = (|| -> Result<Outcome, String> {
        let problem = prepare(positions, triangles, normals, candidates, seed_face, limits, &mut budget)?;
        let mut component = Component::default();
        if let Some(status) = discover(&problem, seed_face, limits, &mut component, &mut budget)? {
            return Ok(outcome(component, status, None, &budget));
        }
        budget.charge(component.vertices.len())?;
        let domains = component.vertices.iter().map(|&v| problem.masks[v]).collect();
        let mut decisions = 0;
        let result = search(&problem, &component, domains, limits, &mut decisions, &mut budget)?;
        component.decisions = decisions;
        let domains = match result {
            Search::Unsatisfiable => return Ok(outcome(component, Status::Unsatisfiable, None, &budget)),
            Search::DecisionLimit => return Ok(outcome(component, Status::DecisionLimit, None, &budget)),
            Search::Solved(domains) => domains,
        };
        budget.charge(positions.len() + component.vertices.len())?;
        let mut assignment = positions.to_vec();
        for (&vertex, mask) in component.vertices.iter().zip(domains) {
            assignment[vertex] = candidates[vertex][mask.trailing_zeros() as usize];
        }
        for &face in &component.faces {
            if !positive(triangles[face].map(|v| assignment[v as usize]), normals[face], &mut budget)? {
                return Err("rounding constraints complete assignment failed an incident face".into());
            }
        }
        Ok(outcome(component, Status::Solved, Some(assignment), &budget))
    })();
    operation.map_err(|message| Failure { work: budget.used, message })
}
