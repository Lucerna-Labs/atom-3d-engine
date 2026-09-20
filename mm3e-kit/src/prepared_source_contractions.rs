//! Private coordinates and tracked face slots for repeated source admissions.
use super::*;
use std::collections::BTreeSet;
use std::sync::Arc;

#[derive(Clone, Debug)]
struct Node {
    bounds: Option<SourceBounds>,
    parent: Option<usize>,
    children: Option<[usize; 2]>,
    face: Option<usize>,
}
#[derive(Debug)]
pub struct PreparedSourceContractions {
    identity: Arc<()>,
    revision: u64,
    intervals: Vec<SourceBounds>,
    midpoints: Vec<Vector>,
    triangles: Vec<Option<[u32; 3]>>,
    incident: BTreeMap<u32, BTreeSet<usize>>,
    nodes: Vec<Node>,
    leaves: Vec<usize>,
    preparation_work: usize,
}
#[derive(Clone, Debug)]
struct Change {
    face: usize,
    original: [u32; 3],
    mapped: Option<[u32; 3]>,
    bounds: Option<SourceBounds>,
}
/// A direction-specific source proof bound to one private index revision.
/// It does not supply a topology/link, displacement, or final-f32 certificate.
#[derive(Debug)]
pub struct SourceContractionTicket {
    identity: Arc<()>,
    revision: u64,
    changes: Vec<Change>,
    report: SourceContractionReport,
}
impl SourceContractionTicket {
    pub fn report(&self) -> &SourceContractionReport {
        &self.report
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceContractionCommit {
    pub work: usize,
    pub revision: u64,
    /// Ascending stable original face slots, never reused after deletion.
    pub removed_faces: Vec<usize>,
    /// Ascending stable original face slots with their updated triangles.
    pub updated_faces: Vec<(usize, [u32; 3])>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceContractionSnapshot {
    pub work: usize,
    pub revision: u64,
    pub triangles: Vec<[u32; 3]>,
    pub original_face_indices: Vec<usize>,
}
fn counted<T>(
    max_work: usize,
    operation: impl FnOnce(&mut Budget) -> Result<T, String>,
) -> Result<T, SourceContractionFailure> {
    let mut budget = Budget { maximum: max_work, report: IntersectionReport::default(), failed_pair: None };
    operation(&mut budget).map_err(|message| SourceContractionFailure { work: budget.report.work, message })
}
fn union(
    a: Option<SourceBounds>,
    b: Option<SourceBounds>,
    budget: &mut Budget,
) -> Result<Option<SourceBounds>, String> {
    budget.charge(3)?;
    Ok(match (a, b) {
        (None, b) => b,
        (a, None) => a,
        (Some(a), Some(b)) => {
            Some([std::array::from_fn(|i| a[0][i].min(b[0][i])), std::array::from_fn(|i| a[1][i].max(b[1][i]))])
        }
    })
}
fn center(bounds: SourceBounds, axis: usize) -> f64 {
    bounds[0][axis] * 0.5 + bounds[1][axis] * 0.5
}

// Fallible median partition: comparison and swap work is precharged as it is
// performed, rather than hiding unbounded sorting inside an infallible closure.
fn partition(ids: &mut [usize], boxes: &[SourceBounds], axis: usize, budget: &mut Budget) -> Result<(), String> {
    let middle = ids.len() / 2;
    let (mut lo, mut hi) = (0, ids.len());
    let compare = |a: usize, b: usize| center(boxes[a], axis).total_cmp(&center(boxes[b], axis)).then(a.cmp(&b));
    while hi - lo > 1 {
        budget.charge(3)?;
        let mut pivots = [ids[lo], ids[(lo + hi) / 2], ids[hi - 1]];
        if compare(pivots[0], pivots[1]).is_gt() {
            pivots.swap(0, 1);
        }
        if compare(pivots[1], pivots[2]).is_gt() {
            pivots.swap(1, 2);
        }
        if compare(pivots[0], pivots[1]).is_gt() {
            pivots.swap(0, 1);
        }
        let pivot = pivots[1];
        let mut less = lo;
        // Three-way partition avoids needing an uncharged search for pivot.
        let (mut at, mut greater) = (lo, hi);
        while at < greater {
            budget.charge(1)?;
            match compare(ids[at], pivot) {
                std::cmp::Ordering::Less => {
                    budget.charge(1)?;
                    ids.swap(at, less);
                    less += 1;
                    at += 1;
                }
                std::cmp::Ordering::Equal => at += 1,
                std::cmp::Ordering::Greater => {
                    budget.charge(1)?;
                    greater -= 1;
                    ids.swap(at, greater);
                }
            }
        }
        if middle < less {
            hi = less;
        } else if middle >= greater {
            lo = greater;
        } else {
            break;
        }
    }
    Ok(())
}
fn build_tree(
    ids: &mut [usize],
    boxes: &[SourceBounds],
    parent: Option<usize>,
    nodes: &mut Vec<Node>,
    leaves: &mut [usize],
    budget: &mut Budget,
) -> Result<usize, String> {
    budget.charge(1)?;
    let index = nodes.len();
    nodes.push(Node { bounds: None, parent, children: None, face: None });
    if ids.len() == 1 {
        budget.charge(2)?;
        nodes[index].bounds = Some(boxes[ids[0]]);
        nodes[index].face = Some(ids[0]);
        leaves[ids[0]] = index;
        return Ok(index);
    }
    let mut bounds = None;
    for &id in ids.iter() {
        bounds = union(bounds, Some(boxes[id]), budget)?;
    }
    let bounds = bounds.unwrap();
    budget.charge(3)?;
    let extent: [f64; 3] = std::array::from_fn(|i| bounds[1][i] - bounds[0][i]);
    let axis = if extent[0] >= extent[1] && extent[0] >= extent[2] {
        0
    } else if extent[1] >= extent[2] {
        1
    } else {
        2
    };
    partition(ids, boxes, axis, budget)?;
    let middle = ids.len() / 2;
    let (left, right) = ids.split_at_mut(middle);
    let a = build_tree(left, boxes, Some(index), nodes, leaves, budget)?;
    let b = build_tree(right, boxes, Some(index), nodes, leaves, budget)?;
    budget.charge(2)?;
    nodes[index].children = Some([a, b]);
    nodes[index].bounds = Some(bounds);
    Ok(index)
}

impl PreparedSourceContractions {
    /// Validate/own fixed enclosures and current triangles, then build a spatial
    /// tree once. This does not imply that the baseline surface is embedded.
    pub fn new(
        intervals: &[SourceBounds],
        triangles: &[[u32; 3]],
        max_work: usize,
    ) -> Result<Self, SourceContractionFailure> {
        counted(max_work, |budget| {
            budget.charge(1)?;
            if intervals.is_empty()
                || intervals.len() > MAX_MESH_VERTICES
                || triangles.is_empty()
                || triangles.len() > MAX_MESH_TRIANGLES
            {
                return Err("prepared source contraction input exceeds geometry bounds".into());
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
            if triangles.iter().any(|t| {
                t.iter().any(|i| *i as usize >= intervals.len()) || t[0] == t[1] || t[1] == t[2] || t[2] == t[0]
            }) {
                return Err("source contraction source triangle indices are invalid".into());
            }
            budget.charge(intervals.len() * 10 + triangles.len() * 3)?;
            let midpoints = intervals.iter().map(|p| std::array::from_fn(|i| p[0][i] * 0.5 + p[1][i] * 0.5)).collect();
            let boxes = triangles.iter().map(|t| source_box(t, intervals, budget)).collect::<Result<Vec<_>, _>>()?;
            let mut incident = BTreeMap::<u32, BTreeSet<usize>>::new();
            for (face, t) in triangles.iter().enumerate() {
                budget.charge(3)?;
                for &vertex in t {
                    incident.entry(vertex).or_default().insert(face);
                }
            }
            let mut nodes = Vec::new();
            let mut leaves = vec![0; triangles.len()];
            let mut ids = (0..triangles.len()).collect::<Vec<_>>();
            build_tree(&mut ids, &boxes, None, &mut nodes, &mut leaves, budget)?;
            Ok(Self {
                identity: Arc::new(()),
                revision: 0,
                intervals: intervals.to_vec(),
                midpoints,
                triangles: triangles.iter().copied().map(Some).collect(),
                incident,
                nodes,
                leaves,
                preparation_work: budget.report.work,
            })
        })
    }
    pub fn preparation_work(&self) -> usize {
        self.preparation_work
    }
    pub fn revision(&self) -> u64 {
        self.revision
    }
    pub fn snapshot(&self, max_work: usize) -> Result<SourceContractionSnapshot, SourceContractionFailure> {
        counted(max_work, |budget| {
            let mut triangles = Vec::new();
            let mut original_face_indices = Vec::new();
            for (id, &triangle) in self.triangles.iter().enumerate() {
                budget.charge(1)?;
                if let Some(triangle) = triangle {
                    budget.charge(2)?;
                    triangles.push(triangle);
                    original_face_indices.push(id);
                }
            }
            Ok(SourceContractionSnapshot {
                work: budget.report.work,
                revision: self.revision,
                triangles,
                original_face_indices,
            })
        })
    }
    fn query(&self, bounds: SourceBounds, budget: &mut Budget) -> Result<BTreeSet<usize>, String> {
        budget.charge(1)?;
        let mut pending = vec![0];
        let mut faces = BTreeSet::new();
        while let Some(index) = pending.pop() {
            budget.charge(1)?;
            let node = &self.nodes[index];
            let Some(stored) = node.bounds else { continue };
            if boxes_disjoint(bounds, stored, budget)? {
                continue;
            }
            if let Some(children) = node.children {
                budget.charge(2)?;
                pending.extend(children);
            } else {
                budget.charge(1)?;
                faces.insert(node.face.unwrap());
            }
        }
        Ok(faces)
    }
    /// Preserve the uncached gates in the same stable face order. Swept queries
    /// use current original triangles, including rows this proposal will delete.
    /// Final queries use current unchanged faces plus separately mapped changed
    /// faces, so moved geometry is never tested against stale changed boxes.
    pub fn check_contraction(
        &self,
        removed: u32,
        retained: u32,
        max_work: usize,
    ) -> Result<SourceContractionTicket, SourceContractionFailure> {
        counted(max_work, |budget| {
            budget.charge(1)?;
            if removed == retained
                || removed as usize >= self.intervals.len()
                || retained as usize >= self.intervals.len()
            {
                return Err("source contraction interval input exceeds geometry/work bounds".into());
            }
            let changed =
                self.incident.get(&removed).ok_or("source contraction endpoints do not form a source edge")?;
            budget.charge(changed.len())?;
            if !changed.iter().any(|&i| self.triangles[i].unwrap().contains(&retained)) {
                return Err("source contraction endpoints do not form a source edge".into());
            }
            let mut report =
                SourceContractionReport { changed_triangles: changed.len(), ..SourceContractionReport::default() };
            let mut changes = Vec::new();
            for &face in changed {
                budget.charge(2)?;
                let original = self.triangles[face].unwrap();
                let mapped =
                    (!original.contains(&retained)).then(|| original.map(|i| if i == removed { retained } else { i }));
                if let Some(mapped) = mapped {
                    let dot = source_normal_dot(original, mapped, &self.intervals, budget)?;
                    if dot.lo <= 0. || !dot.lo.is_finite() || !dot.hi.is_finite() {
                        return Err(format!("source contraction cannot certify positive source orientation for triangle {face} (normal-dot interval {}..{})",dot.lo,dot.hi));
                    }
                    report.source_orientations += 1;
                }
                changes.push(Change { face, original, mapped, bounds: None });
            }
            for change in &changes {
                budget.charge(3)?;
                let mut hull = change.original.to_vec();
                if !hull.contains(&retained) {
                    budget.charge(1)?;
                    hull.push(retained);
                }
                let bounds = source_box(&hull, &self.intervals, budget)?;
                for other in self.query(bounds, budget)? {
                    budget.charge(1)?;
                    let t = self.triangles[other].unwrap();
                    if t.iter().any(|v| hull.contains(v)) {
                        continue;
                    }
                    report.swept_pairs += 1;
                    if !hulls_separate(
                        &hull,
                        &t,
                        &self.intervals,
                        &self.midpoints,
                        budget,
                        &mut report.separating_axes,
                    )? {
                        return Err(format!("source contraction cannot certify nonadjacent swept separation for triangles {} and {other}",change.face));
                    }
                }
            }
            for change in &mut changes {
                if let Some(mapped) = change.mapped {
                    change.bounds = Some(source_box(&mapped, &self.intervals, budget)?);
                }
            }
            for change in &changes {
                let Some(a) = change.mapped else { continue };
                let ab = change.bounds.unwrap();
                let mut candidates = BTreeMap::new();
                for other in self.query(ab, budget)? {
                    budget.charge(1)?;
                    if !changed.contains(&other) {
                        candidates.insert(other, self.triangles[other].unwrap());
                    }
                }
                // Every changed-vs-changed pair uses BOTH mapped face boxes.
                for other in &changes {
                    budget.charge(1)?;
                    if other.face <= change.face {
                        continue;
                    }
                    let Some(b) = other.mapped else { continue };
                    if !boxes_disjoint(ab, other.bounds.unwrap(), budget)? {
                        candidates.insert(other.face, b);
                    }
                }
                for (other, b) in candidates {
                    report.final_pairs += 1;
                    if !final_source_pair(a, b, &self.intervals, &self.midpoints, budget, &mut report.separating_axes)?
                    {
                        return Err(format!(
                            "source contraction cannot certify final embedding for triangles {} and {other}",
                            change.face
                        ));
                    }
                }
            }
            budget.charge(changes.len() + 1)?;
            report.work = budget.report.work;
            Ok(SourceContractionTicket {
                identity: Arc::clone(&self.identity),
                revision: self.revision,
                changes,
                report,
            })
        })
    }
    /// Stage and precharge all face/adjacency writes and all affected tree
    /// ancestor bounds. A stale/foreign ticket or any exhausted budget leaves
    /// the complete private state unchanged. Deleted leaves become empty.
    pub fn commit_contraction(
        &mut self,
        ticket: SourceContractionTicket,
        max_work: usize,
    ) -> Result<SourceContractionCommit, SourceContractionFailure> {
        counted(max_work, |budget| {
            budget.charge(1)?;
            if !Arc::ptr_eq(&self.identity, &ticket.identity) || self.revision != ticket.revision {
                return Err("prepared source contraction ticket is stale or belongs to another index".into());
            }
            let revision = self.revision.checked_add(1).ok_or("prepared source contraction revision overflow")?;
            let mut vertices = BTreeSet::new();
            let mut affected = BTreeSet::new();
            let mut staged = BTreeMap::new();
            for change in &ticket.changes {
                budget.charge(4)?;
                vertices.extend(change.original);
                let leaf = self.leaves[change.face];
                staged.insert(leaf, change.bounds);
                let mut node = Some(leaf);
                while let Some(index) = node {
                    budget.charge(2)?;
                    affected.insert(index);
                    node = self.nodes[index].parent;
                }
            }
            let mut incident = BTreeMap::new();
            for &vertex in &vertices {
                budget.charge(1 + self.incident[&vertex].len())?;
                incident.insert(vertex, self.incident[&vertex].clone());
            }
            // The retained endpoint is in at least one deleted source edge
            // face, so it is among the collected original-star vertices.
            for change in &ticket.changes {
                budget.charge(3)?;
                for vertex in change.original {
                    incident.get_mut(&vertex).unwrap().remove(&change.face);
                }
                if let Some(t) = change.mapped {
                    budget.charge(3)?;
                    for vertex in t {
                        incident.get_mut(&vertex).unwrap().insert(change.face);
                    }
                }
            }
            // Preorder node IDs put every child after its parent. Reverse IDs
            // therefore refit all affected ancestors bottom-up before mutation.
            for &index in affected.iter().rev() {
                budget.charge(1)?;
                if let Some([a, b]) = self.nodes[index].children {
                    let a = staged.get(&a).copied().unwrap_or(self.nodes[a].bounds);
                    let b = staged.get(&b).copied().unwrap_or(self.nodes[b].bounds);
                    let next = union(a, b, budget)?;
                    staged.insert(index, next);
                }
            }
            budget.charge(ticket.changes.len() * 2 + incident.len() + staged.len())?;
            let mut removed_faces = Vec::new();
            let mut updated_faces = Vec::new();
            for change in ticket.changes {
                self.triangles[change.face] = change.mapped;
                if let Some(t) = change.mapped {
                    updated_faces.push((change.face, t));
                } else {
                    removed_faces.push(change.face);
                }
            }
            for (vertex, faces) in incident {
                if faces.is_empty() {
                    self.incident.remove(&vertex);
                } else {
                    self.incident.insert(vertex, faces);
                }
            }
            for (index, bounds) in staged {
                self.nodes[index].bounds = bounds;
            }
            self.revision = revision;
            Ok(SourceContractionCommit { work: budget.report.work, revision, removed_faces, updated_faces })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const WORK: usize = 20_000_000;
    fn audit(index: &PreparedSourceContractions, probes: &[SourceBounds]) {
        let mut budget = Budget { maximum: WORK, report: IntersectionReport::default(), failed_pair: None };
        for (id, node) in index.nodes.iter().enumerate() {
            if let Some([a, b]) = node.children {
                assert_eq!(index.nodes[a].parent, Some(id));
                assert_eq!(index.nodes[b].parent, Some(id));
                assert_eq!(node.bounds, union(index.nodes[a].bounds, index.nodes[b].bounds, &mut budget).unwrap());
            } else {
                let face = node.face.unwrap();
                assert_eq!(index.leaves[face], id);
                let expected = index.triangles[face].map(|t| source_box(&t, &index.intervals, &mut budget).unwrap());
                assert_eq!(node.bounds, expected);
            }
        }
        for &probe in probes {
            let mut expected = BTreeSet::new();
            for (i, t) in index.triangles.iter().enumerate() {
                if let Some(t) = t {
                    let bounds = source_box(t, &index.intervals, &mut budget).unwrap();
                    if !boxes_disjoint(probe, bounds, &mut budget).unwrap() {
                        expected.insert(i);
                    }
                }
            }
            assert_eq!(index.query(probe, &mut budget).unwrap(), expected);
        }
    }
    #[test]
    fn committed_boxes_expand_outside_old_internal_nodes_and_deleted_leaves_disappear() {
        let mut points = vec![[0., 0., 0.], [10., 0., 0.], [0., 1., 0.], [0., 0., 1.]];
        let mut triangles = vec![[0, 1, 2], [0, 2, 3]];
        for i in 0..8 {
            let x = 2. + f64::from(i) * 0.1;
            let start = points.len() as u32;
            points.extend([[x, -1., 0.], [x + 0.05, -1., 0.], [x, -0.9, 0.]]);
            triangles.push([start, start + 1, start + 2]);
        }
        let bounds: Vec<_> = points.iter().copied().map(|p| [p, p]).collect();
        let mut prepared = PreparedSourceContractions::new(&bounds, &triangles, WORK).unwrap();
        let moved_probe = [[8., 0.05, 0.05], [9., 0.15, 0.15]];
        let probes = [moved_probe, [[-0.1, -1.1, -0.1], [10.1, 1.1, 1.1]], [[2., -1., 0.], [2.8, -0.9, 0.]]];
        audit(&prepared, &probes);
        let mut work = Budget { maximum: WORK, report: IntersectionReport::default(), failed_pair: None };
        assert!(!prepared.query(moved_probe, &mut work).unwrap().contains(&1));
        let mut node = prepared.nodes[prepared.leaves[1]].parent;
        let mut old_excluding_internal = Vec::new();
        while let Some(i) = node {
            if boxes_disjoint(moved_probe, prepared.nodes[i].bounds.unwrap(), &mut work).unwrap() {
                old_excluding_internal.push(i);
            }
            node = prepared.nodes[i].parent;
        }
        assert!(!old_excluding_internal.is_empty(), "fixture must expand past an old internal box, not just a leaf");
        let admission = prepared.check_contraction(0, 1, WORK).unwrap();
        prepared.commit_contraction(admission, WORK).unwrap();
        audit(&prepared, &probes);
        assert!(prepared.query(moved_probe, &mut work).unwrap().contains(&1));
        for i in old_excluding_internal {
            assert!(!boxes_disjoint(moved_probe, prepared.nodes[i].bounds.unwrap(), &mut work).unwrap());
        }
        assert!(prepared.nodes[prepared.leaves[0]].bounds.is_none());
        let admission = prepared.check_contraction(2, 3, WORK).unwrap();
        prepared.commit_contraction(admission, WORK).unwrap();
        audit(&prepared, &probes);
        assert!(prepared.nodes[prepared.leaves[1]].bounds.is_none());
        assert!(prepared.query(moved_probe, &mut work).unwrap().is_empty());
    }
    #[test]
    fn source_only_admission_can_empty_the_tree_without_reviving_removed_faces() {
        // The separate topology gate forbids erasing a closed component. The
        // source index itself must safely track exactly the uncached API's
        // limited source predicates, including this open one-triangle input.
        let bounds = vec![[[0., 0., 0.]; 2], [[1., 0., 0.]; 2], [[0., 1., 0.]; 2]];
        let mut prepared = PreparedSourceContractions::new(&bounds, &[[0, 1, 2]], WORK).unwrap();
        let admission = prepared.check_contraction(0, 1, WORK).unwrap();
        prepared.commit_contraction(admission, WORK).unwrap();
        assert!(prepared.nodes[0].bounds.is_none());
        assert!(prepared.snapshot(WORK).unwrap().triangles.is_empty());
        audit(&prepared, &[[[-10.; 3], [10.; 3]]]);
        assert!(prepared.check_contraction(0, 1, WORK).is_err());
    }
}
