//! Deterministic, fixed-step deformable triangle cloth.
//!
//! Stretch and signed dihedral bending use XPBD: the compliance contribution is
//! `compliance / substep_dt²`, with accumulated multipliers reset each substep.
//! See Macklin, Müller and Chentanez, https://mmacklin.com/xpbd.pdf (2016).
//! This is an elastic triangle-network model, not a measured woven-fabric material.
//! Contact projects vertices against a caller-provided world-space signed distance
//! and outward normal. External contact remains discrete and one-way. Optional
//! self-contact uses thickness-aware vertex-triangle and edge-edge constraints,
//! with conservative advancement along linear substep trajectories. It excludes
//! the local one-ring topology and directly sewn vertex links. Neither external triangle-interior contact nor
//! rigid-body reaction is provided. Starting intersections, pinned conflicts,
//! finite solver convergence and nonlinear between-substep motion remain limits.

use crate::Vec3;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

#[path = "cloth_contact.rs"]
mod self_contact;

const MAX_VERTICES: usize = 100_000;
const MAX_TRIANGLES: usize = 200_000;
const MAX_PROJECTIONS_PER_STEP: u64 = 50_000_000;
const MIN_EDGE: f64 = 1e-8;
/// Hard allocation bound; seam projections also spend the normal step budget.
pub const MAX_CLOTH_SEAMS: usize = 100_000;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClothSettings {
    /// Seconds advanced by every successful `step`; independent of wall time.
    pub fixed_dt: f32,
    pub substeps: u32,
    pub iterations: u32,
    pub gravity: Vec3,
    /// Velocity attenuation is exp(-damping_per_second * substep_dt).
    pub damping_per_second: f32,
    /// Inverse edge spring stiffness (metres / newton); zero is inextensible.
    pub stretch_compliance: f32,
    /// Inverse angular stiffness (radians / newton-metre); zero resists bending.
    pub bend_compliance: f32,
    /// Distance of each simulated vertex from an external signed surface.
    pub collision_thickness: f32,
    /// Re-query and re-project the contact surface this many times per iteration.
    pub contact_iterations: u32,
    /// Enable nonlocal vertex-triangle and edge-edge cloth self-contact.
    pub self_collision: bool,
    /// Minimum distance between two cloth layers, in world units.
    pub self_collision_thickness: f32,
    /// Coulomb coefficient for external and self contact; zero preserves sliding.
    /// External geometry is assumed stationary for tangential relative motion.
    pub friction_coefficient: f32,
    /// Cumulative broadphase candidates per fixed step, including final audit.
    pub self_collision_max_candidates: u32,
    /// Maximum counted broadphase comparisons, narrowphase samples and contact
    /// projections per fixed step. Mesh sizes separately bound allocation/sorting.
    pub self_collision_max_work: u64,
}

impl Default for ClothSettings {
    fn default() -> Self {
        Self {
            fixed_dt: 1.0 / 60.0,
            substeps: 4,
            iterations: 12,
            gravity: Vec3::new(0.0, -9.81, 0.0),
            damping_per_second: 0.8,
            stretch_compliance: 1e-7,
            bend_compliance: 1e-3,
            collision_thickness: 0.003,
            contact_iterations: 2,
            self_collision: false,
            self_collision_thickness: 0.006,
            friction_coefficient: 0.0,
            self_collision_max_candidates: 200_000,
            self_collision_max_work: 5_000_000,
        }
    }
}

impl ClothSettings {
    pub fn validate(&self) -> Result<(), ClothError> {
        if !self.fixed_dt.is_finite() || self.fixed_dt <= 0.0 || self.fixed_dt > 1.0 {
            return Err(error("fixed_dt must be finite, greater than zero and at most one second"));
        }
        if !(1..=128).contains(&self.substeps)
            || !(1..=256).contains(&self.iterations)
            || !(1..=32).contains(&self.contact_iterations)
        {
            return Err(error("substeps, iterations and contact_iterations must be in 1..=128, 1..=256 and 1..=32"));
        }
        if !finite(self.gravity)
            || [
                self.damping_per_second,
                self.stretch_compliance,
                self.bend_compliance,
                self.collision_thickness,
                self.self_collision_thickness,
                self.friction_coefficient,
            ]
            .iter()
            .any(|x| !x.is_finite() || *x < 0.0)
        {
            return Err(error(
                "gravity must be finite; damping, compliance, friction and thickness must be finite and nonnegative",
            ));
        }
        if self.self_collision && self.self_collision_thickness < 1e-8 {
            return Err(error("enabled self collision requires thickness of at least 1e-8 world units"));
        }
        if !(1..=2_000_000).contains(&self.self_collision_max_candidates)
            || !(1..=MAX_PROJECTIONS_PER_STEP).contains(&self.self_collision_max_work)
        {
            return Err(error("self collision candidate/work budgets must be in 1..=2000000 and 1..=50000000"));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClothContact {
    /// Signed world-space distance: positive outside, negative inside.
    pub distance: f32,
    /// Finite, nonzero outward normal; normalized by the solver.
    pub normal: Vec3,
}

/// An authored sewing distance between two distinct cloth vertices. Zero
/// target distance joins coincident panel boundaries. Compliance has the same
/// metres/newton convention as edge stretch compliance. This constraint does
/// not model thread geometry, progressive sewing, stitch breakage or weave.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClothSeam {
    pub vertices: [u32; 2],
    pub rest_length: f32,
    pub compliance: f32,
}

/// Complete dynamic checkpoint for the same topology, seams, masses and settings.
/// Constraint multipliers are local to a substep and are not persistent state.
#[derive(Clone, Debug, PartialEq)]
pub struct ClothState {
    pub positions: Vec<Vec3>,
    pub velocities: Vec<Vec3>,
    /// Indexed like positions. Only entries with zero inverse mass are applied.
    pub pin_targets: Vec<Vec3>,
    pub completed_steps: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClothStepReport {
    pub completed_steps: u64,
    pub max_relative_edge_error: f32,
    pub max_speed: f32,
    /// Number of vertex projections actually made against an external surface.
    pub contact_projections: u64,
    /// Collapsed triangles/hinges skipped because no bending gradient exists.
    pub degenerate_bend_projections: u64,
    /// Signed contact queries include pins; pinned penetrations are reported,
    /// never silently moved away from an authored attachment.
    pub max_contact_penetration: f32,
    pub self_contact_projections: u64,
    pub self_vertex_triangle_projections: u64,
    pub self_edge_edge_projections: u64,
    pub self_contact_candidates: u64,
    pub self_contact_work: u64,
    /// Final unsigned thickness deficit, including unsatisfiable pinned pairs.
    /// This is a local clearance diagnostic, not proof of globally untangled cloth.
    pub max_self_contact_penetration: f32,
    /// Number of nonzero sewing distance corrections applied during this step.
    pub seam_projections: u64,
    /// Maximum absolute difference from an authored seam target length, metres.
    /// Includes unresolvable pairs of kinematic pins.
    pub max_seam_length_error: f32,
}

/// Read-only static clearance result for nonlocal vertex-triangle and edge-edge
/// features. Includes pinned geometry because masses do not affect distance.
/// A zero deficit is not proof that the triangle surface is globally untangled:
/// a preexisting edge-through-face intersection can avoid all sampled feature
/// proximities. This query performs no swept test between cached frames.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClothSelfContactReport {
    /// Largest positive `self_collision_thickness - feature_distance`.
    pub max_penetration: f32,
    /// Broadphase candidates examined, under `self_collision_max_candidates`.
    pub candidates: u64,
    /// Collision work counted under `self_collision_max_work`.
    pub work: u64,
}

/// Measures static self-contact clearance on an arbitrary supplied pose and
/// indexed triangle topology, without changing the inputs or simulating motion.
/// Uses the solver's own/one-ring exclusions, primitive distance queries and
/// candidate/work limits. Every call has its own bounded query budget.
///
/// The explicit query runs even when `settings.self_collision` is false; it
/// requires positive self-collision thickness. Topology must satisfy the same
/// index, duplicate-face, winding, manifold-edge and isolated-vertex checks as
/// cloth construction. Current triangles may be collapsed. No mass/pin data is
/// needed. See [`ClothSelfContactReport`] for the surface-intersection limitation.
pub fn measure_cloth_self_contact(
    positions: &[Vec3],
    triangles: &[[u32; 3]],
    settings: ClothSettings,
) -> Result<ClothSelfContactReport, ClothError> {
    ClothSettings { self_collision: true, ..settings }.validate()?;
    self_contact::Topology::from_triangles(positions.len(), triangles)?.measure(positions, &settings)
}

/// Static clearance query with the solver's directly sewn vertex exclusions.
/// Only each explicit seam link is added to local adjacency; exclusions do not
/// expand transitively across the seam. A zero-length sewn pair intentionally
/// meets despite layer thickness; neighboring unsewn feature contacts remain
/// active and may conflict with tightly spaced or inconsistent authored seams.
pub fn measure_cloth_self_contact_with_seams(
    positions: &[Vec3],
    triangles: &[[u32; 3]],
    seams: &[ClothSeam],
    settings: ClothSettings,
) -> Result<ClothSelfContactReport, ClothError> {
    ClothSettings { self_collision: true, ..settings }.validate()?;
    let seams = validated_seams(seams, positions.len())?;
    let mut topology = self_contact::Topology::from_triangles(positions.len(), triangles)?;
    topology.add_seams(&seams);
    topology.measure(positions, &settings)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClothError(pub String);
impl fmt::Display for ClothError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for ClothError {}
fn error(message: &str) -> ClothError {
    ClothError(message.to_owned())
}

fn validated_seams(seams: &[ClothSeam], vertex_count: usize) -> Result<Vec<ClothSeam>, ClothError> {
    if seams.len() > MAX_CLOTH_SEAMS {
        return Err(error("cloth exceeds the 100000 sewing-constraint limit"));
    }
    let mut ordered = Vec::with_capacity(seams.len());
    for seam in seams {
        let [a, b] = seam.vertices;
        if a == b || a as usize >= vertex_count || b as usize >= vertex_count {
            return Err(error("seam vertices must be distinct existing cloth vertices"));
        }
        if !seam.rest_length.is_finite()
            || seam.rest_length < 0.0
            || !seam.compliance.is_finite()
            || seam.compliance < 0.0
        {
            return Err(error("seam target length and compliance must be finite and nonnegative"));
        }
        ordered.push(ClothSeam { vertices: [a.min(b), a.max(b)], ..*seam });
    }
    ordered.sort_unstable_by_key(|seam| seam.vertices);
    if ordered.windows(2).any(|pair| pair[0].vertices == pair[1].vertices) {
        return Err(error("duplicate seam vertex pair, including reversed pairs"));
    }
    Ok(ordered)
}

fn validate_projection_work(
    edges: usize,
    bends: usize,
    seams: usize,
    vertices: usize,
    settings: ClothSettings,
) -> Result<(), ClothError> {
    let projections =
        (edges as u64 + bends as u64 + seams as u64 + vertices as u64 * u64::from(settings.contact_iterations))
            * u64::from(settings.iterations)
            * u64::from(settings.substeps);
    if projections > MAX_PROJECTIONS_PER_STEP {
        return Err(error("cloth step exceeds the 50000000 projection work budget; reduce mesh resolution, seam count or solver iterations"));
    }
    Ok(())
}

#[derive(Clone, Debug)]
struct Edge {
    vertices: [usize; 2],
    rest_length: f64,
    rest_direction: DVec,
}
#[derive(Clone, Debug)]
struct Bend {
    /// Shared edge first, followed by the opposite vertex of each triangle.
    vertices: [usize; 4],
    rest_angle: f64,
}

#[derive(Clone, Debug)]
pub struct Cloth {
    rest_positions: Vec<Vec3>,
    triangles: Vec<[u32; 3]>,
    inverse_masses: Vec<f32>,
    settings: ClothSettings,
    state: ClothState,
    edges: Vec<Edge>,
    bends: Vec<Bend>,
    seams: Vec<ClothSeam>,
    seam_rest_directions: Vec<DVec>,
    contact_topology: Option<self_contact::Topology>,
}

impl Cloth {
    /// Builds stretch constraints on all edges and signed dihedral constraints
    /// across manifold interior edges. Inputs must be consistently wound,
    /// nondegenerate indexed triangles, with no duplicate faces or isolated vertices.
    /// An inverse mass of zero means an exact kinematic pin; positive values are 1/kg.
    pub fn new(
        rest_positions: Vec<Vec3>,
        triangles: Vec<[u32; 3]>,
        inverse_masses: Vec<f32>,
        settings: ClothSettings,
    ) -> Result<Self, ClothError> {
        settings.validate()?;
        if !(3..=MAX_VERTICES).contains(&rest_positions.len())
            || triangles.is_empty()
            || triangles.len() > MAX_TRIANGLES
        {
            return Err(error("cloth requires 3..=100000 vertices and 1..=200000 triangles"));
        }
        if rest_positions.len() != inverse_masses.len()
            || rest_positions.iter().any(|p| !finite(*p))
            || inverse_masses.iter().any(|w| !w.is_finite() || *w < 0.0)
        {
            return Err(error("one finite nonnegative inverse mass is required per finite rest position"));
        }
        let mut incidence = BTreeMap::<(usize, usize), Vec<(usize, bool)>>::new();
        let mut seen = BTreeSet::new();
        let mut used = vec![false; rest_positions.len()];
        for triangle in &triangles {
            let [a, b, c] = triangle.map(|v| v as usize);
            if [a, b, c].iter().any(|v| *v >= rest_positions.len()) || a == b || b == c || a == c {
                return Err(error("triangle index is out of bounds or repeated"));
            }
            let mut key = [a, b, c];
            key.sort_unstable();
            if !seen.insert(key) {
                return Err(error("duplicate triangle"));
            }
            let ab = DVec::from(rest_positions[b]) - DVec::from(rest_positions[a]);
            let ac = DVec::from(rest_positions[c]) - DVec::from(rest_positions[a]);
            // Relative criterion rejects needles/collinearity without imposing a
            // fixed world-space area floor on otherwise valid small triangles.
            if ab.length() < MIN_EDGE
                || ac.length() < MIN_EDGE
                || (ab - ac).length() < MIN_EDGE
                || ab.cross(ac).length() <= 1e-10 * ab.length() * ac.length()
            {
                return Err(error("degenerate triangle or rest edge shorter than 1e-8 world units"));
            }
            for (i, j, opposite) in [(a, b, c), (b, c, a), (c, a, b)] {
                used[i] = true;
                incidence.entry((i.min(j), i.max(j))).or_default().push((opposite, i < j));
            }
        }
        if used.iter().any(|v| !v) {
            return Err(error("isolated cloth vertex is not part of any triangle"));
        }
        let mut edges = Vec::with_capacity(incidence.len());
        let mut bends = Vec::new();
        for ((a, b), adjacent) in incidence {
            if adjacent.len() > 2 {
                return Err(error("nonmanifold cloth edge has more than two incident triangles"));
            }
            let difference = DVec::from(rest_positions[a]) - DVec::from(rest_positions[b]);
            let rest_length = difference.length();
            if !rest_length.is_finite() || rest_length < MIN_EDGE {
                return Err(error("invalid rest edge length"));
            }
            edges.push(Edge { vertices: [a, b], rest_length, rest_direction: difference * (1.0 / rest_length) });
            if adjacent.len() == 2 {
                if adjacent[0].1 == adjacent[1].1 {
                    return Err(error("neighboring cloth triangles have inconsistent winding"));
                }
                let vertices = [a, b, adjacent[0].0, adjacent[1].0];
                let points = vertices.map(|i| DVec::from(rest_positions[i]));
                let (rest_angle, _) = dihedral(points).ok_or_else(|| error("degenerate rest hinge"))?;
                bends.push(Bend { vertices, rest_angle });
            }
        }
        validate_projection_work(edges.len(), bends.len(), 0, rest_positions.len(), settings)?;
        let state = ClothState {
            positions: rest_positions.clone(),
            velocities: vec![Vec3::ZERO; rest_positions.len()],
            pin_targets: rest_positions.clone(),
            completed_steps: 0,
        };
        let contact_topology =
            settings.self_collision.then(|| self_contact::Topology::new(rest_positions.len(), &triangles, &edges));
        Ok(Self {
            rest_positions,
            triangles,
            inverse_masses,
            settings,
            state,
            edges,
            bends,
            seams: Vec::new(),
            seam_rest_directions: Vec::new(),
            contact_topology,
        })
    }

    /// Builds the same triangle-network material as [`Cloth::new`] and adds
    /// authored sewing constraints without replacing any original rest edges.
    pub fn new_with_seams(
        rest_positions: Vec<Vec3>,
        triangles: Vec<[u32; 3]>,
        inverse_masses: Vec<f32>,
        settings: ClothSettings,
        seams: Vec<ClothSeam>,
    ) -> Result<Self, ClothError> {
        let mut cloth = Self::new(rest_positions, triangles, inverse_masses, settings)?;
        cloth.set_seams(seams)?;
        Ok(cloth)
    }

    /// Atomically replaces sewing definitions without changing rest fabric,
    /// dynamic state or pins. Pairs are canonicalized/sorted; reversed duplicate
    /// pairs are rejected. Contradictory pinned targets are diagnosed during
    /// simulation, not silently moved or rejected as malformed authoring.
    pub fn set_seams(&mut self, seams: Vec<ClothSeam>) -> Result<(), ClothError> {
        let seams = validated_seams(&seams, self.rest_positions.len())?;
        validate_projection_work(
            self.edges.len(),
            self.bends.len(),
            seams.len(),
            self.rest_positions.len(),
            self.settings,
        )?;
        let directions = seams
            .iter()
            .map(|seam| {
                let [a, b] = seam.vertices.map(|i| i as usize);
                let delta = DVec::from(self.rest_positions[a]) - DVec::from(self.rest_positions[b]);
                let length = delta.length();
                if length > 0.0 {
                    delta * (1.0 / length)
                } else {
                    DVec([1.0, 0.0, 0.0])
                }
            })
            .collect();
        let contact_topology = self.settings.self_collision.then(|| {
            let mut topology = self_contact::Topology::new(self.rest_positions.len(), &self.triangles, &self.edges);
            topology.add_seams(&seams);
            topology
        });
        self.seams = seams;
        self.seam_rest_directions = directions;
        self.contact_topology = contact_topology;
        Ok(())
    }

    pub fn seams(&self) -> &[ClothSeam] {
        &self.seams
    }

    pub fn positions(&self) -> &[Vec3] {
        &self.state.positions
    }
    pub fn velocities(&self) -> &[Vec3] {
        &self.state.velocities
    }
    pub fn triangles(&self) -> &[[u32; 3]] {
        &self.triangles
    }
    pub fn rest_positions(&self) -> &[Vec3] {
        &self.rest_positions
    }
    pub fn inverse_masses(&self) -> &[f32] {
        &self.inverse_masses
    }
    pub fn settings(&self) -> ClothSettings {
        self.settings
    }
    pub fn state(&self) -> &ClothState {
        &self.state
    }
    pub fn constraint_counts(&self) -> (usize, usize) {
        (self.edges.len(), self.bends.len())
    }

    /// Read-only static clearance query using this cloth's already-validated
    /// topology/settings and arbitrary supplied positions. Never changes the
    /// dynamic checkpoint or pin targets. Includes pinned contacts and operates
    /// even if simulation self-collision is disabled. This is a local feature
    /// distance query, not a proof of globally intersection-free geometry.
    pub fn measure_self_contact(&self, positions: &[Vec3]) -> Result<ClothSelfContactReport, ClothError> {
        if let Some(topology) = &self.contact_topology {
            topology.measure(positions, &self.settings)
        } else {
            let mut topology = self_contact::Topology::new(self.rest_positions.len(), &self.triangles, &self.edges);
            topology.add_seams(&self.seams);
            topology.measure(positions, &self.settings)
        }
    }

    /// Validated atomic checkpoint restore. Pinned positions may differ from
    /// targets to represent an attachment move pending the next fixed step.
    pub fn restore_state(&mut self, state: ClothState) -> Result<(), ClothError> {
        let n = self.rest_positions.len();
        if state.positions.len() != n
            || state.velocities.len() != n
            || state.pin_targets.len() != n
            || state.positions.iter().chain(&state.velocities).chain(&state.pin_targets).any(|v| !finite(*v))
        {
            return Err(error("checkpoint must contain one finite position, velocity and pin target per vertex"));
        }
        self.state = state;
        Ok(())
    }

    /// Moves an existing zero-inverse-mass attachment. The next `step` linearly
    /// interpolates its old position to this target over all substeps, so cloth
    /// receives the attachment motion rather than teleporting after simulation.
    pub fn set_pin_position(&mut self, vertex: u32, target: Vec3) -> Result<(), ClothError> {
        let i = vertex as usize;
        if i >= self.inverse_masses.len() || self.inverse_masses[i] != 0.0 || !finite(target) {
            return Err(error("pin target requires an existing pinned vertex and a finite position"));
        }
        self.state.pin_targets[i] = target;
        Ok(())
    }

    /// Advances exactly one configured fixed step. A callback returns the closest
    /// relevant signed surface (or None). Invalid samples/nonfinite integration
    /// fail atomically: no position, velocity or step counter is committed.
    /// Callback side effects cannot be rolled back; use a pure geometry query.
    pub fn step<F>(&mut self, mut contact: F) -> Result<ClothStepReport, ClothError>
    where
        F: FnMut(Vec3) -> Option<ClothContact>,
    {
        let next_step =
            self.state.completed_steps.checked_add(1).ok_or_else(|| error("cloth step counter overflow"))?;
        let mut next = self.state.clone();
        let mut positions: Vec<DVec> = next.positions.iter().copied().map(DVec::from).collect();
        let mut velocities: Vec<DVec> = next.velocities.iter().copied().map(DVec::from).collect();
        let original_positions = positions.clone();
        let h = f64::from(self.settings.fixed_dt) / f64::from(self.settings.substeps);
        let alpha_stretch = f64::from(self.settings.stretch_compliance) / (h * h);
        let alpha_bend = f64::from(self.settings.bend_compliance) / (h * h);
        let attenuation = (-f64::from(self.settings.damping_per_second) * h).exp();
        let gravity = DVec::from(self.settings.gravity);
        let mut contact_projections = 0;
        let mut degenerate_bend_projections = 0;
        let mut self_stats = self_contact::Stats::default();
        let mut seam_projections = 0;
        for substep in 0..self.settings.substeps {
            let previous = positions.clone();
            let t = f64::from(substep + 1) / f64::from(self.settings.substeps);
            for i in 0..positions.len() {
                if self.inverse_masses[i] == 0.0 {
                    // Exact endpoint avoids interpolation rounding of the pin.
                    positions[i] = if substep + 1 == self.settings.substeps {
                        DVec::from(next.pin_targets[i])
                    } else {
                        original_positions[i] * (1.0 - t) + DVec::from(next.pin_targets[i]) * t
                    };
                } else {
                    velocities[i] = velocities[i] * attenuation + gravity * h;
                    positions[i] = positions[i] + velocities[i] * h;
                }
            }
            let mut stretch_lambda = vec![0.0; self.edges.len()];
            let mut bend_lambda = vec![0.0; self.bends.len()];
            let mut seam_lambda = vec![0.0; self.seams.len()];
            let mut seam_directions = self.seam_rest_directions.clone();
            for (seam, direction) in self.seams.iter().zip(&mut seam_directions) {
                let [a, b] = seam.vertices.map(|i| i as usize);
                let delta = previous[a] - previous[b];
                let length = delta.length();
                if length > 0.0 {
                    *direction = delta * (1.0 / length);
                }
            }
            for _ in 0..self.settings.iterations {
                for (edge, lambda) in self.edges.iter().zip(&mut stretch_lambda) {
                    let [a, b] = edge.vertices;
                    let wa = f64::from(self.inverse_masses[a]);
                    let wb = f64::from(self.inverse_masses[b]);
                    if wa + wb == 0.0 {
                        continue;
                    }
                    let delta = positions[a] - positions[b];
                    let length = delta.length();
                    let gradient = if length > MIN_EDGE * 1e-4 { delta * (1.0 / length) } else { edge.rest_direction };
                    let dl = (-(length - edge.rest_length) - alpha_stretch * *lambda) / (wa + wb + alpha_stretch);
                    *lambda += dl;
                    positions[a] = positions[a] + gradient * (wa * dl);
                    positions[b] = positions[b] - gradient * (wb * dl);
                }
                for (bend, lambda) in self.bends.iter().zip(&mut bend_lambda) {
                    let Some((angle, gradients)) = dihedral(bend.vertices.map(|i| positions[i])) else {
                        degenerate_bend_projections += 1;
                        continue;
                    };
                    let weights = bend.vertices.map(|i| f64::from(self.inverse_masses[i]));
                    let denominator = weights.iter().zip(&gradients).map(|(w, g)| w * g.length_sq()).sum::<f64>();
                    if denominator == 0.0 {
                        continue;
                    }
                    let constraint = wrap_angle(angle - bend.rest_angle);
                    let dl = (-constraint - alpha_bend * *lambda) / (denominator + alpha_bend);
                    *lambda += dl;
                    for j in 0..4 {
                        let index = bend.vertices[j];
                        positions[index] = positions[index] + gradients[j] * (weights[j] * dl);
                    }
                }
                for ((seam, lambda), direction) in self.seams.iter().zip(&mut seam_lambda).zip(&mut seam_directions) {
                    let [a, b] = seam.vertices.map(|i| i as usize);
                    let wa = f64::from(self.inverse_masses[a]);
                    let wb = f64::from(self.inverse_masses[b]);
                    if wa + wb == 0.0 {
                        continue;
                    }
                    let delta = positions[a] - positions[b];
                    let length = delta.length();
                    if length > 0.0 {
                        *direction = delta * (1.0 / length);
                    }
                    let alpha = f64::from(seam.compliance) / (h * h);
                    let dl = (-(length - f64::from(seam.rest_length)) - alpha * *lambda) / (wa + wb + alpha);
                    *lambda += dl;
                    if dl != 0.0 {
                        if wa > 0.0 {
                            positions[a] = positions[a] + *direction * (wa * dl);
                        }
                        if wb > 0.0 {
                            positions[b] = positions[b] - *direction * (wb * dl);
                        }
                        seam_projections += 1;
                    }
                }
                for (i, position) in positions.iter_mut().enumerate() {
                    if self.inverse_masses[i] == 0.0 {
                        continue;
                    }
                    for _ in 0..self.settings.contact_iterations {
                        let p = position.to_vec()?;
                        let Some(sample) = contact(p) else { break };
                        let normal = validate_contact(sample)?;
                        let gap = f64::from(sample.distance) - f64::from(self.settings.collision_thickness);
                        if gap >= 0.0 {
                            break;
                        }
                        *position = *position - normal * gap;
                        let displacement = *position - previous[i];
                        let tangent = displacement - normal * displacement.dot(normal);
                        let tangent_length = tangent.length();
                        if tangent_length > 0.0 && self.settings.friction_coefficient > 0.0 {
                            let correction = (f64::from(self.settings.friction_coefficient) * -gap).min(tangent_length);
                            *position = *position - tangent * (correction / tangent_length);
                        }
                        contact_projections += 1;
                    }
                }
                if let Some(topology) = &self.contact_topology {
                    topology.project(
                        &mut positions,
                        &previous,
                        &self.inverse_masses,
                        &self.settings,
                        &mut self_stats,
                    )?;
                }
            }
            for i in 0..positions.len() {
                velocities[i] = (positions[i] - previous[i]) * (1.0 / h);
                // Validate even when a caller supplies no collision geometry.
                positions[i].to_vec()?;
                velocities[i].to_vec()?;
            }
        }
        next.positions = positions.iter().map(|v| v.to_vec()).collect::<Result<_, _>>()?;
        next.velocities = velocities.iter().map(|v| v.to_vec()).collect::<Result<_, _>>()?;
        next.completed_steps = next_step;
        let max_relative_edge_error = self
            .edges
            .iter()
            .map(|edge| {
                let [a, b] = edge.vertices;
                (((DVec::from(next.positions[a]) - DVec::from(next.positions[b])).length() / edge.rest_length) - 1.0)
                    .abs()
            })
            .fold(0.0_f64, f64::max);
        let max_speed = next.velocities.iter().copied().map(|v| DVec::from(v).length()).fold(0.0_f64, f64::max);
        let max_seam_length_error = self
            .seams
            .iter()
            .map(|seam| {
                let [a, b] = seam.vertices.map(|i| i as usize);
                ((DVec::from(next.positions[a]) - DVec::from(next.positions[b])).length() - f64::from(seam.rest_length))
                    .abs()
            })
            .fold(0.0_f64, f64::max);
        let mut max_contact_penetration = 0.0_f32;
        let max_self_contact_penetration = if let Some(topology) = &self.contact_topology {
            topology.penetration(&positions, &self.settings, &mut self_stats)? as f32
        } else {
            0.0
        };
        for &position in &next.positions {
            if let Some(sample) = contact(position) {
                validate_contact(sample)?;
                max_contact_penetration =
                    max_contact_penetration.max(self.settings.collision_thickness - sample.distance);
            }
        }
        if !max_contact_penetration.is_finite()
            || max_relative_edge_error > f64::from(f32::MAX)
            || max_speed > f64::from(f32::MAX)
            || max_seam_length_error > f64::from(f32::MAX)
        {
            return Err(error("cloth diagnostics exceeded finite representable range"));
        }
        self.state = next;
        Ok(ClothStepReport {
            completed_steps: next_step,
            max_relative_edge_error: max_relative_edge_error as f32,
            max_speed: max_speed as f32,
            contact_projections,
            degenerate_bend_projections,
            max_contact_penetration,
            self_contact_projections: self_stats.vertex_triangle + self_stats.edge_edge,
            self_vertex_triangle_projections: self_stats.vertex_triangle,
            self_edge_edge_projections: self_stats.edge_edge,
            self_contact_candidates: self_stats.candidates,
            self_contact_work: self_stats.work,
            max_self_contact_penetration,
            seam_projections,
            max_seam_length_error: max_seam_length_error as f32,
        })
    }
}

fn finite(v: Vec3) -> bool {
    v.x.is_finite() && v.y.is_finite() && v.z.is_finite()
}
fn validate_contact(sample: ClothContact) -> Result<DVec, ClothError> {
    if !sample.distance.is_finite() || !finite(sample.normal) {
        return Err(error("contact callback returned a nonfinite distance or normal"));
    }
    let normal = DVec::from(sample.normal);
    let length = normal.length();
    if length <= 1e-12 {
        return Err(error("contact callback returned a zero or numerically vanishing normal"));
    }
    Ok(normal * (1.0 / length))
}
fn wrap_angle(angle: f64) -> f64 {
    (angle + std::f64::consts::PI).rem_euclid(std::f64::consts::TAU) - std::f64::consts::PI
}

/// Signed hinge angle and analytic gradients. n0 = edge × opposite0,
/// n1 = opposite1 × edge; the cross/dot atan2 avoids acos singularity at
/// coplanarity. Degenerate current hinges have no unique bending gradient.
fn dihedral(p: [DVec; 4]) -> Option<(f64, [DVec; 4])> {
    let edge = p[1] - p[0];
    let a = p[2] - p[0];
    let b = p[3] - p[0];
    let edge_sq = edge.length_sq();
    let n0 = edge.cross(a);
    let n1 = b.cross(edge);
    let n0_sq = n0.length_sq();
    let n1_sq = n1.length_sq();
    if edge_sq < MIN_EDGE * MIN_EDGE * 1e-8
        || n0_sq <= 1e-20 * edge_sq * a.length_sq()
        || n1_sq <= 1e-20 * edge_sq * b.length_sq()
    {
        return None;
    }
    let edge_length = edge_sq.sqrt();
    let normal0 = n0 * (1.0 / n0_sq.sqrt());
    let normal1 = n1 * (1.0 / n1_sq.sqrt());
    let sin = (edge * (1.0 / edge_length)).dot(normal0.cross(normal1));
    let cos = normal0.dot(normal1);
    let g2 = n0 * (-edge_length / n0_sq);
    let g3 = n1 * (-edge_length / n1_sq);
    let ta = a.dot(edge) / edge_sq;
    let tb = b.dot(edge) / edge_sq;
    let g0 = g2 * (ta - 1.0) + g3 * (tb - 1.0);
    let g1 = g2 * -ta + g3 * -tb;
    Some((sin.atan2(cos), [g0, g1, g2, g3]))
}

// Local f64 working coordinates prevent f32 dot/cross overflow on otherwise
// finite authored positions and preserve small compliance terms in the solver.
#[derive(Clone, Copy, Debug, Default)]
struct DVec([f64; 3]);
impl From<Vec3> for DVec {
    fn from(v: Vec3) -> Self {
        Self([f64::from(v.x), f64::from(v.y), f64::from(v.z)])
    }
}
impl DVec {
    fn to_vec(self) -> Result<Vec3, ClothError> {
        let v = Vec3::new(self.0[0] as f32, self.0[1] as f32, self.0[2] as f32);
        if !finite(v) {
            return Err(error("cloth integration produced nonfinite or out-of-range positions or velocities"));
        }
        Ok(v)
    }
    fn dot(self, rhs: Self) -> f64 {
        self.0.iter().zip(rhs.0).map(|(a, b)| a * b).sum()
    }
    fn length_sq(self) -> f64 {
        self.dot(self)
    }
    fn length(self) -> f64 {
        self.length_sq().sqrt()
    }
    fn cross(self, rhs: Self) -> Self {
        let [a, b, c] = self.0;
        let [x, y, z] = rhs.0;
        Self([b * z - c * y, c * x - a * z, a * y - b * x])
    }
}
impl std::ops::Add for DVec {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(std::array::from_fn(|i| self.0[i] + rhs.0[i]))
    }
}
impl std::ops::Sub for DVec {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(std::array::from_fn(|i| self.0[i] - rhs.0[i]))
    }
}
impl std::ops::Mul<f64> for DVec {
    type Output = Self;
    fn mul(self, s: f64) -> Self {
        Self(self.0.map(|v| v * s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn analytic_dihedral_gradients_match_independent_central_difference() {
        for points in [
            [Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.2, 1.0, 0.0), Vec3::new(0.7, -1.0, 0.0)],
            [
                Vec3::new(0.1, 0.2, 0.3),
                Vec3::new(1.3, 0.1, -0.2),
                Vec3::new(-0.2, 1.1, 0.7),
                Vec3::new(0.8, -0.5, -0.8),
            ],
        ] {
            let p = points.map(DVec::from);
            let (_, gradients) = dihedral(p).unwrap();
            for i in 0..4 {
                for axis in 0..3 {
                    let mut left = p;
                    let mut right = p;
                    left[i].0[axis] -= 1e-6;
                    right[i].0[axis] += 1e-6;
                    let measured = wrap_angle(dihedral(right).unwrap().0 - dihedral(left).unwrap().0) / 2e-6;
                    assert!(
                        (measured - gradients[i].0[axis]).abs() < 1e-7,
                        "{i} {axis}: {measured} != {}",
                        gradients[i].0[axis]
                    );
                }
            }
        }
    }
}
