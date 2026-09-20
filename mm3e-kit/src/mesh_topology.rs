//! Counted topology validation for closed, oriented triangle complexes.
//!
//! This module uses indices only. It does not establish geometric face area,
//! source orientation, intersections, displacement, or preservation of UVs.
//! Retrying callers must charge `TopologyFailure::work` as well as successful work.
use crate::meshing::{MAX_MESH_TRIANGLES, MAX_MESH_VERTICES};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

type EdgeKey = (u32, u32);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TopologyFailure {
    pub work: usize,
    pub message: String,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TopologyReport {
    pub work: usize,
    pub components: usize,
    pub vertices: usize,
    pub edges: usize,
    pub triangles: usize,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CollapseReport {
    pub work: usize,
    pub components: usize,
    pub edge_faces: usize,
    pub common_link_vertices: usize,
    pub common_link_edges: usize,
}
struct Budget {
    maximum: usize,
    used: usize,
}
impl Budget {
    fn charge(&mut self, amount: usize) -> Result<(), String> {
        if amount > self.maximum.saturating_sub(self.used) {
            return Err("mesh topology exhausted work budget".into());
        }
        self.used += amount;
        Ok(())
    }
}
#[derive(Clone, Debug, Default)]
struct Edge {
    incidence: u8,
    direction_sum: i8,
}
struct Complex {
    edges: BTreeMap<EdgeKey, Edge>,
    links: BTreeMap<u32, Vec<EdgeKey>>,
    components: usize,
    triangles: usize,
}
fn edge(a: u32, b: u32) -> EdgeKey {
    (a.min(b), a.max(b))
}
fn counted<T>(maximum: usize, operation: impl FnOnce(&mut Budget) -> Result<T, String>) -> Result<T, TopologyFailure> {
    let mut budget = Budget { maximum, used: 0 };
    operation(&mut budget).map_err(|message| TopologyFailure { work: budget.used, message })
}

fn checked_complex(triangles: &[[u32; 3]], budget: &mut Budget) -> Result<Complex, String> {
    if triangles.is_empty() || triangles.len() > MAX_MESH_TRIANGLES {
        return Err("mesh topology requires bounded nonempty triangles".into());
    }
    budget.charge(triangles.len())?;
    let mut faces = BTreeSet::new();
    let mut edges = BTreeMap::<EdgeKey, Edge>::new();
    let mut links = BTreeMap::<u32, Vec<EdgeKey>>::new();
    for &triangle in triangles {
        budget.charge(4)?;
        if triangle.iter().any(|&i| i as usize >= MAX_MESH_VERTICES) {
            return Err("mesh topology vertex index exceeds the mesh limit".into());
        }
        let [a, b, c] = triangle;
        if a == b || b == c || c == a {
            return Err("mesh topology triangle repeats a vertex index".into());
        }
        let mut unordered = triangle;
        unordered.sort_unstable();
        if !faces.insert(unordered) {
            return Err("mesh topology contains a duplicate face, including reversed orientation".into());
        }
        for (a, b, c) in [(a, b, c), (b, c, a), (c, a, b)] {
            budget.charge(4)?;
            let entry = edges.entry(edge(a, b)).or_default();
            if entry.incidence == 2 {
                return Err(format!("mesh topology edge ({},{}) has more than two incident faces", a.min(b), a.max(b)));
            }
            entry.incidence += 1;
            entry.direction_sum += if a < b { 1 } else { -1 };
            links.entry(a).or_default().push(edge(b, c));
        }
    }
    for (&(a, b), entry) in &edges {
        budget.charge(1)?;
        if entry.incidence != 2 {
            return Err(format!("mesh topology edge ({a},{b}) is open: {} incident face(s)", entry.incidence));
        }
        if entry.direction_sum != 0 {
            return Err(format!("mesh topology edge ({a},{b}) has inconsistent face orientation"));
        }
    }
    // A closed edge manifold can still pinch at a vertex. Its link must be ONE
    // cycle, rather than a disjoint collection of cycles with degree two.
    for (&vertex, link_edges) in &links {
        budget.charge(1)?;
        let mut adjacency = BTreeMap::<u32, Vec<u32>>::new();
        for &(a, b) in link_edges {
            budget.charge(4)?;
            for (x, y) in [(a, b), (b, a)] {
                let neighbors = adjacency.entry(x).or_default();
                if neighbors.len() == 2 {
                    return Err(format!("mesh topology vertex {vertex} has a branching link"));
                }
                neighbors.push(y);
            }
        }
        budget.charge(adjacency.len())?;
        if adjacency.len() < 3
            || adjacency.values().any(|neighbors| neighbors.len() != 2 || neighbors[0] == neighbors[1])
        {
            return Err(format!("mesh topology vertex {vertex} does not have a cyclic link"));
        }
        let start = *adjacency.keys().next().unwrap();
        let mut previous = None;
        let mut current = start;
        let mut visited = 0;
        loop {
            budget.charge(2)?;
            let neighbors = &adjacency[&current];
            let next = if Some(neighbors[0]) == previous { neighbors[1] } else { neighbors[0] };
            previous = Some(current);
            current = next;
            visited += 1;
            if current == start {
                break;
            }
            if visited >= adjacency.len() {
                return Err(format!("mesh topology vertex {vertex} has an invalid link cycle"));
            }
        }
        if visited != adjacency.len() {
            return Err(format!("mesh topology vertex {vertex} has a disconnected link (vertex pinch)"));
        }
    }
    // Use compressed IDs so sparse but bounded source indices do not allocate
    // by their largest numerical ID. Every map entry/union traversal is charged.
    budget.charge(links.len() * 3)?;
    let indices: BTreeMap<_, _> = links.keys().enumerate().map(|(i, &id)| (id, i)).collect();
    let mut parents: Vec<usize> = (0..indices.len()).collect();
    let mut ranks = vec![0_u8; indices.len()];
    let mut components = indices.len();
    fn root(parents: &mut [usize], mut i: usize, budget: &mut Budget) -> Result<usize, String> {
        loop {
            budget.charge(1)?;
            if parents[i] == i {
                return Ok(i);
            }
            parents[i] = parents[parents[i]];
            i = parents[i];
        }
    }
    for &(a, b) in edges.keys() {
        budget.charge(2)?;
        let mut a = root(&mut parents, indices[&a], budget)?;
        let mut b = root(&mut parents, indices[&b], budget)?;
        if a != b {
            if ranks[a] < ranks[b] {
                std::mem::swap(&mut a, &mut b);
            }
            parents[b] = a;
            if ranks[a] == ranks[b] {
                ranks[a] += 1;
            }
            components -= 1;
        }
    }
    Ok(Complex { edges, links, components, triangles: triangles.len() })
}

/// Require unique unoriented faces, exactly two oppositely oriented faces at
/// each edge, and one cyclic link at every vertex. No repair is performed.
pub fn validate(triangles: &[[u32; 3]], max_work: usize) -> Result<TopologyReport, TopologyFailure> {
    counted(max_work, |budget| {
        let complex = checked_complex(triangles, budget)?;
        Ok(TopologyReport {
            work: budget.used,
            components: complex.components,
            vertices: complex.links.len(),
            edges: complex.edges.len(),
            triangles: complex.triangles,
        })
    })
}

/// Validate the entire source complex, then require the full simplicial link
/// condition L(removed) intersection L(retained) = L(edge). In a triangle
/// complex the edge link has vertices only: common link EDGES must be absent.
/// The two directions have identical topological eligibility. Geometry and
/// metric/intersection certificates must be checked separately for each one.
pub fn can_collapse(
    triangles: &[[u32; 3]],
    removed: u32,
    retained: u32,
    max_work: usize,
) -> Result<CollapseReport, TopologyFailure> {
    counted(max_work, |budget| {
        if removed == retained || removed as usize >= MAX_MESH_VERTICES || retained as usize >= MAX_MESH_VERTICES {
            return Err("mesh topology contraction requires two distinct bounded vertex indices".into());
        }
        let complex = checked_complex(triangles, budget)?;
        budget.charge(1)?;
        let Some(incident) = complex.edges.get(&edge(removed, retained)) else {
            return Err("mesh topology contraction endpoints do not form a source edge".into());
        };
        let link = |vertex, budget: &mut Budget| -> Result<(BTreeSet<u32>, BTreeSet<EdgeKey>), String> {
            let mut vertices = BTreeSet::new();
            let mut edges = BTreeSet::new();
            for &(a, b) in &complex.links[&vertex] {
                budget.charge(3)?;
                vertices.extend([a, b]);
                edges.insert((a, b));
            }
            Ok((vertices, edges))
        };
        let (a_vertices, a_edges) = link(removed, budget)?;
        let (b_vertices, b_edges) = link(retained, budget)?;
        budget.charge(a_vertices.len() + b_vertices.len() + a_edges.len() + b_edges.len())?;
        let common_vertices: BTreeSet<_> = a_vertices.intersection(&b_vertices).copied().collect();
        let common_edges: BTreeSet<_> = a_edges.intersection(&b_edges).copied().collect();
        let mut edge_link = BTreeSet::new();
        for &(a, b) in &complex.links[&removed] {
            budget.charge(1)?;
            if a == retained {
                edge_link.insert(b);
            } else if b == retained {
                edge_link.insert(a);
            }
        }
        if common_vertices != edge_link {
            return Err(format!(
                "mesh topology full link condition failed: {} common link vertices versus {} edge-link vertices",
                common_vertices.len(),
                edge_link.len()
            ));
        }
        if !common_edges.is_empty() {
            return Err(format!(
                "mesh topology full link condition failed: {} common link edge(s) are not in the edge link",
                common_edges.len()
            ));
        }
        Ok(CollapseReport {
            work: budget.used,
            components: complex.components,
            edge_faces: usize::from(incident.incidence),
            common_link_vertices: common_vertices.len(),
            common_link_edges: common_edges.len(),
        })
    })
}

/// A validated closed oriented complex with privately owned, tracked adjacency.
/// Coordinates and geometric certificates remain the caller's responsibility.
/// The initial proof is retained only across full-link endpoint contractions;
/// there is no operation for replacing or externally mutating its triangles.
#[derive(Debug)]
pub struct PreparedTopology {
    identity: Arc<()>,
    revision: u64,
    triangles: Vec<Option<[u32; 3]>>,
    faces: BTreeMap<[u32; 3], usize>,
    edges: BTreeMap<EdgeKey, Edge>,
    links: BTreeMap<u32, BTreeSet<EdgeKey>>,
    incident: BTreeMap<u32, BTreeSet<usize>>,
    initial_report: TopologyReport,
}

/// An opaque full-link admission tied to one prepared complex revision.
/// Other direction-specific geometry checks may run before this is committed.
#[derive(Debug)]
pub struct PreparedCollapse {
    identity: Arc<()>,
    revision: u64,
    removed: u32,
    retained: u32,
    report: CollapseReport,
}
impl PreparedCollapse {
    pub fn report(&self) -> &CollapseReport {
        &self.report
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CollapseCommitReport {
    pub work: usize,
    pub revision: u64,
    /// Stable input row IDs. Deleted rows are never reassigned.
    pub removed_faces: Vec<usize>,
    /// Stable input row ID followed by its newly stored oriented triangle.
    pub updated_faces: Vec<(usize, [u32; 3])>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TopologySnapshot {
    pub work: usize,
    pub revision: u64,
    pub triangles: Vec<[u32; 3]>,
    /// One original input triangle row per corresponding current triangle.
    pub original_face_indices: Vec<usize>,
}

fn unordered(mut triangle: [u32; 3]) -> [u32; 3] {
    triangle.sort_unstable();
    triangle
}

fn cyclic_link(vertex: u32, links: &BTreeSet<EdgeKey>, budget: &mut Budget) -> Result<(), String> {
    let mut adjacency = BTreeMap::<u32, Vec<u32>>::new();
    for &(a, b) in links {
        budget.charge(4)?;
        adjacency.entry(a).or_default().push(b);
        adjacency.entry(b).or_default().push(a);
    }
    budget.charge(adjacency.len())?;
    if adjacency.len() < 3 || adjacency.values().any(|neighbors| neighbors.len() != 2 || neighbors[0] == neighbors[1]) {
        return Err(format!("prepared mesh topology vertex {vertex} does not have a cyclic link"));
    }
    let start = *adjacency.keys().next().unwrap();
    let (mut previous, mut current, mut visited) = (None, start, 0);
    loop {
        budget.charge(2)?;
        let neighbors = &adjacency[&current];
        let next = if Some(neighbors[0]) == previous { neighbors[1] } else { neighbors[0] };
        previous = Some(current);
        current = next;
        visited += 1;
        if current == start {
            break;
        }
        if visited >= adjacency.len() {
            return Err(format!("prepared mesh topology vertex {vertex} has an invalid link cycle"));
        }
    }
    if visited != adjacency.len() {
        return Err(format!("prepared mesh topology vertex {vertex} has a disconnected link"));
    }
    Ok(())
}

impl PreparedTopology {
    /// Establish the complete proof once and take an owned copy of the input.
    /// Constructor work includes both validation and private cache construction.
    pub fn new(triangles: &[[u32; 3]], max_work: usize) -> Result<Self, TopologyFailure> {
        counted(max_work, |budget| {
            budget.charge(1)?;
            let complex = checked_complex(triangles, budget)?;
            let mut faces = BTreeMap::new();
            let mut incident = BTreeMap::<u32, BTreeSet<usize>>::new();
            let mut stored = Vec::with_capacity(triangles.len());
            for (index, &triangle) in triangles.iter().enumerate() {
                budget.charge(6)?;
                stored.push(Some(triangle));
                faces.insert(unordered(triangle), index);
                for vertex in triangle {
                    incident.entry(vertex).or_default().insert(index);
                }
            }
            let mut links = BTreeMap::new();
            for (vertex, adjacent) in complex.links {
                budget.charge(1 + adjacent.len())?;
                links.insert(vertex, adjacent.into_iter().collect());
            }
            let initial_report = TopologyReport {
                work: budget.used,
                components: complex.components,
                vertices: links.len(),
                edges: complex.edges.len(),
                triangles: complex.triangles,
            };
            Ok(Self {
                identity: Arc::new(()),
                revision: 0,
                triangles: stored,
                faces,
                edges: complex.edges,
                links,
                incident,
                initial_report,
            })
        })
    }

    pub fn initial_report(&self) -> &TopologyReport {
        &self.initial_report
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// Snapshot in original input row order. Its mutable returned data is an
    /// independent copy and cannot invalidate the privately owned proof.
    pub fn snapshot(&self, max_work: usize) -> Result<TopologySnapshot, TopologyFailure> {
        counted(max_work, |budget| {
            budget.charge(self.faces.len() * 2)?;
            let mut triangles = Vec::with_capacity(self.faces.len());
            let mut original_face_indices = Vec::with_capacity(self.faces.len());
            for (index, &triangle) in self.triangles.iter().enumerate() {
                budget.charge(1)?;
                if let Some(triangle) = triangle {
                    triangles.push(triangle);
                    original_face_indices.push(index);
                }
            }
            Ok(TopologySnapshot { work: budget.used, revision: self.revision, triangles, original_face_indices })
        })
    }

    /// Check the entire simplicial edge link from current private endpoint stars.
    /// The rest of the already validated complex is neither rebuilt nor rescanned.
    pub fn check_collapse(
        &self,
        removed: u32,
        retained: u32,
        max_work: usize,
    ) -> Result<PreparedCollapse, TopologyFailure> {
        counted(max_work, |budget| {
            budget.charge(1)?;
            if removed == retained || removed as usize >= MAX_MESH_VERTICES || retained as usize >= MAX_MESH_VERTICES {
                return Err("prepared mesh topology contraction requires two distinct bounded vertex indices".into());
            }
            let Some(incident) = self.edges.get(&edge(removed, retained)) else {
                return Err("prepared mesh topology contraction endpoints do not form a current source edge".into());
            };
            let vertices = |vertex, budget: &mut Budget| -> Result<BTreeSet<u32>, String> {
                let mut vertices = BTreeSet::new();
                for &(a, b) in &self.links[&vertex] {
                    budget.charge(2)?;
                    vertices.extend([a, b]);
                }
                Ok(vertices)
            };
            let a_vertices = vertices(removed, budget)?;
            let b_vertices = vertices(retained, budget)?;
            let (a_edges, b_edges) = (&self.links[&removed], &self.links[&retained]);
            budget.charge(a_vertices.len() + b_vertices.len() + a_edges.len() + b_edges.len())?;
            let common_vertices: BTreeSet<_> = a_vertices.intersection(&b_vertices).copied().collect();
            let common_edges: BTreeSet<_> = a_edges.intersection(b_edges).copied().collect();
            let mut edge_link = BTreeSet::new();
            for &(a, b) in a_edges {
                budget.charge(1)?;
                if a == retained {
                    edge_link.insert(b);
                } else if b == retained {
                    edge_link.insert(a);
                }
            }
            if common_vertices != edge_link {
                return Err(format!(
                    "prepared mesh topology full link condition failed: {} common link vertices versus {} edge-link vertices",
                    common_vertices.len(), edge_link.len()
                ));
            }
            if !common_edges.is_empty() {
                return Err(format!(
                    "prepared mesh topology full link condition failed: {} common link edge(s) are not in the edge link",
                    common_edges.len()
                ));
            }
            Ok(PreparedCollapse {
                identity: Arc::clone(&self.identity),
                revision: self.revision,
                removed,
                retained,
                report: CollapseReport {
                    work: budget.used,
                    components: self.initial_report.components,
                    edge_faces: usize::from(incident.incidence),
                    common_link_vertices: common_vertices.len(),
                    common_link_edges: common_edges.len(),
                },
            })
        })
    }

    /// Stage the affected stars, verify updated edge incidence and cyclic links,
    /// precharge all writes, then commit without a fallible step. Any budget,
    /// stale-ticket or invariant failure leaves every private cache unchanged.
    /// This operation establishes topology only, not geometric embedding.
    pub fn commit_collapse(
        &mut self,
        admission: PreparedCollapse,
        max_work: usize,
    ) -> Result<CollapseCommitReport, TopologyFailure> {
        counted(max_work, |budget| {
            budget.charge(1)?;
            if !Arc::ptr_eq(&admission.identity, &self.identity) || admission.revision != self.revision {
                return Err("prepared mesh topology contraction ticket is stale or belongs to another complex".into());
            }
            let next_revision = self.revision.checked_add(1).ok_or("prepared mesh topology revision overflow")?;
            let (removed, retained) = (admission.removed, admission.retained);
            let mut updates = Vec::new();
            let mut vertices = BTreeSet::new();
            let mut old_faces = BTreeSet::new();
            for &index in &self.incident[&removed] {
                budget.charge(5)?;
                let old = self.triangles[index].unwrap();
                let new = (!old.contains(&retained)).then(|| old.map(|i| if i == removed { retained } else { i }));
                vertices.extend(old);
                old_faces.insert(index);
                updates.push((index, old, new));
            }
            let mut incident = BTreeMap::new();
            let mut links = BTreeMap::new();
            for &vertex in &vertices {
                budget.charge(2 + self.incident[&vertex].len())?;
                incident.insert(vertex, self.incident[&vertex].clone());
                links.insert(vertex, BTreeSet::new());
            }
            let mut edges = BTreeMap::<EdgeKey, Edge>::new();
            let mut new_faces = BTreeMap::new();
            for &(index, old, new) in &updates {
                for (triangle, adding) in [(Some(old), false), (new, true)] {
                    let Some([a, b, c]) = triangle else { continue };
                    budget.charge(1)?;
                    if adding {
                        let key = unordered([a, b, c]);
                        if new_faces.insert(key, index).is_some()
                            || self.faces.get(&key).is_some_and(|row| !old_faces.contains(row))
                        {
                            return Err("prepared mesh topology contraction would duplicate a face".into());
                        }
                    }
                    for (a, b) in [(a, b), (b, c), (c, a)] {
                        budget.charge(7)?;
                        let key = edge(a, b);
                        let entry =
                            edges.entry(key).or_insert_with(|| self.edges.get(&key).cloned().unwrap_or_default());
                        let direction = if a < b { 1 } else { -1 };
                        if adding {
                            entry.incidence =
                                entry.incidence.checked_add(1).ok_or("prepared edge incidence overflow")?;
                            entry.direction_sum += direction;
                            incident.get_mut(&a).unwrap().insert(index);
                            // Another old affected face can still carry this same
                            // link edge until its deletion is staged below. Rebuild
                            // the small affected links from final rows afterward.
                        } else {
                            entry.incidence =
                                entry.incidence.checked_sub(1).ok_or("prepared edge incidence underflow")?;
                            entry.direction_sum -= direction;
                            incident.get_mut(&a).unwrap().remove(&index);
                        }
                    }
                }
            }
            for (&(a, b), entry) in &edges {
                budget.charge(1)?;
                if entry.direction_sum != 0 || (entry.incidence != 0 && entry.incidence != 2) {
                    return Err(format!("prepared mesh topology contraction produced invalid edge ({a},{b})"));
                }
            }
            budget.charge(updates.len())?;
            let replacements: BTreeMap<_, _> = updates.iter().map(|&(row, _, triangle)| (row, triangle)).collect();
            for (&vertex, adjacent) in &incident {
                let link = links.get_mut(&vertex).unwrap();
                link.clear();
                for &row in adjacent {
                    budget.charge(4)?;
                    let triangle = replacements.get(&row).copied().unwrap_or(self.triangles[row]).unwrap();
                    let opposite: Vec<_> = triangle.into_iter().filter(|&i| i != vertex).collect();
                    if opposite.len() != 2 || !link.insert(edge(opposite[0], opposite[1])) {
                        return Err("prepared mesh topology contraction produced an invalid vertex link".into());
                    }
                }
                if !link.is_empty() {
                    cyclic_link(vertex, link, budget)?;
                } else if vertex != removed {
                    return Err("prepared mesh topology contraction removed an unexpected vertex".into());
                }
            }
            if !incident[&removed].is_empty() || updates.iter().filter(|(_, _, new)| new.is_none()).count() != 2 {
                return Err("prepared mesh topology contraction has an invalid endpoint star".into());
            }
            // No fallible operation follows this charge. In particular a failed
            // budget cannot expose half-updated triangles or adjacency.
            budget.charge(updates.len() * 4 + edges.len() + vertices.len() * 2)?;
            let mut removed_faces = Vec::new();
            let mut updated_faces = Vec::new();
            for (row, old, new) in updates {
                self.faces.remove(&unordered(old));
                self.triangles[row] = new;
                if let Some(triangle) = new {
                    updated_faces.push((row, triangle));
                } else {
                    removed_faces.push(row);
                }
            }
            self.faces.extend(new_faces);
            for (key, value) in edges {
                if value.incidence == 0 {
                    self.edges.remove(&key);
                } else {
                    self.edges.insert(key, value);
                }
            }
            for (vertex, adjacent) in incident {
                if adjacent.is_empty() {
                    self.incident.remove(&vertex);
                    self.links.remove(&vertex);
                } else {
                    self.incident.insert(vertex, adjacent);
                    self.links.insert(vertex, links.remove(&vertex).unwrap());
                }
            }
            self.revision = next_revision;
            Ok(CollapseCommitReport { work: budget.used, revision: self.revision, removed_faces, updated_faces })
        })
    }
}
