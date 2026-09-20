//! Bounded, deterministic rotation search for a constrained two-segment chain.
//! This returns the best feasible pose visited, not a proof of global optimality
//! or infeasibility. Translation, scale, parent transform and rest pivots are fixed.
use crate::{
    rotation_limit::{RotationLimit, ANGULAR_TOLERANCE},
    Quat, Transform, Vec3,
};

pub const MAX_EVALUATIONS: u32 = 32_768;

#[derive(Clone, Copy, Debug)]
pub struct JointPose {
    pub pivot: Vec3,
    pub translation: Vec3,
    pub scale: f32,
    pub rotation: Quat,
}

#[derive(Clone, Debug)]
pub struct Request {
    pub parent: Transform,
    pub joints: [JointPose; 3],
    pub limits: [Option<RotationLimit>; 3],
    /// Local root/middle rotations from the unconstrained geometric solve.
    pub analytic_seed: [Quat; 2],
    pub middle_target: Vec3,
    pub tip_target: Vec3,
    pub tip_orientation: Option<Quat>,
    pub tolerance_m: f64,
    pub orientation_tolerance_rad: f64,
    pub max_evaluations: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct Solution {
    pub rotations: [Quat; 3],
    pub root: Vec3,
    pub middle: Vec3,
    pub tip: Vec3,
    pub middle_error: f64,
    pub tip_error: f64,
    pub orientation_error: Option<f64>,
    pub evaluations: u32,
    pub converged: bool,
    pub budget_exhausted: bool,
}

fn finite(v: Vec3) -> bool {
    [v.x, v.y, v.z].into_iter().all(f32::is_finite)
}
fn distance(a: Vec3, b: Vec3) -> f64 {
    [a.x, a.y, a.z]
        .into_iter()
        .zip([b.x, b.y, b.z])
        .map(|(a, b)| (f64::from(a) - f64::from(b)).powi(2))
        .sum::<f64>()
        .sqrt()
}
fn valid_quaternion(q: Quat) -> bool {
    let values = [q.x, q.y, q.z, q.w].map(f64::from);
    values.into_iter().all(f64::is_finite) && (values.into_iter().map(|x| x * x).sum::<f64>() - 1.0).abs() <= 1e-4
}
fn conjugate(q: Quat) -> Quat {
    Quat { x: -q.x, y: -q.y, z: -q.z, w: q.w }
}
fn angle(a: Quat, b: Quat) -> f64 {
    // Difference of normalized quaternions, using vector/scalar atan2 instead of
    // an f32 dot/acos dead zone near zero and at equivalent opposite signs.
    let a = [a.x, a.y, a.z, a.w].map(f64::from);
    let b = [b.x, b.y, b.z, b.w].map(f64::from);
    let vector = [
        a[3] * b[0] - a[0] * b[3] - a[1] * b[2] + a[2] * b[1],
        a[3] * b[1] + a[0] * b[2] - a[1] * b[3] - a[2] * b[0],
        a[3] * b[2] - a[0] * b[1] + a[1] * b[0] - a[2] * b[3],
    ];
    let scalar = a.into_iter().zip(b).map(|(a, b)| a * b).sum::<f64>();
    2.0 * vector.into_iter().map(|x| x * x).sum::<f64>().sqrt().atan2(scalar.abs())
}
fn unit(v: Vec3) -> Result<Vec3, String> {
    let length = distance(v, Vec3::ZERO);
    if !finite(v) || length == 0.0 {
        return Err("limited IK requires nonzero representable rest segment directions".into());
    }
    Ok(Vec3::new((f64::from(v.x) / length) as f32, (f64::from(v.y) / length) as f32, (f64::from(v.z) / length) as f32))
}
fn project(limit: Option<RotationLimit>, q: Quat) -> Result<Quat, String> {
    match limit {
        Some(limit) => Ok(limit.project(q)?.rotation),
        None => Ok(q),
    }
}
fn valid_transform(transform: Transform) -> bool {
    if !finite(transform.pos) || !transform.scale.is_finite() || transform.scale <= 0.0 {
        return false;
    }
    let columns = transform.rot.cols;
    if !columns.into_iter().all(finite) {
        return false;
    }
    let dot = |a: Vec3, b: Vec3| {
        f64::from(a.x) * f64::from(b.x) + f64::from(a.y) * f64::from(b.y) + f64::from(a.z) * f64::from(b.z)
    };
    let epsilon = 64.0 * f64::from(f32::EPSILON);
    columns.into_iter().all(|column| (dot(column, column) - 1.0).abs() <= epsilon)
        && dot(columns[0], columns[1]).abs() <= epsilon
        && dot(columns[0], columns[2]).abs() <= epsilon
        && dot(columns[1], columns[2]).abs() <= epsilon
        && dot(columns[0].cross(columns[1]), columns[2]) > 0.0
}
fn fk(request: &Request, rotations: [Quat; 3]) -> Result<([Transform; 3], [Vec3; 3]), String> {
    let mut parent = request.parent;
    let mut transforms = [Transform::IDENTITY; 3];
    let mut positions = [Vec3::ZERO; 3];
    for i in 0..3 {
        let joint = request.joints[i];
        let local = Transform::around_pivot(joint.pivot, rotations[i], joint.scale, joint.translation);
        parent = parent.compose(local);
        positions[i] = parent.to_world(joint.pivot);
        if !valid_transform(parent) || !finite(positions[i]) {
            return Err("limited IK pose is not representable as finite native transforms".into());
        }
        transforms[i] = parent;
    }
    Ok((transforms, positions))
}

#[derive(Clone, Copy)]
struct Candidate {
    solution: Solution,
    score: f64,
}
struct Search<'a> {
    request: &'a Request,
    initial: [Vec3; 3],
    lengths: [f64; 2],
    evaluations: u32,
    best: Option<Candidate>,
}
impl Search<'_> {
    fn evaluate(&mut self, root_middle: [Quat; 2]) -> Option<Candidate> {
        if self.evaluations >= self.request.max_evaluations {
            return None;
        }
        self.evaluations += 1;
        let mut rotations = [
            project(self.request.limits[0], root_middle[0]).ok()?,
            project(self.request.limits[1], root_middle[1]).ok()?,
            self.request.joints[2].rotation,
        ];
        let (mut transforms, mut positions) = fk(self.request, rotations).ok()?;
        if let Some(desired) = self.request.tip_orientation {
            let parent = Quat::from_mat3(transforms[1].rot);
            rotations[2] = project(self.request.limits[2], conjugate(parent) * desired).ok()?;
            (transforms, positions) = fk(self.request, rotations).ok()?;
        }
        let tolerance = self.request.tolerance_m;
        if distance(positions[0], self.initial[0]) > tolerance
            || (distance(positions[0], positions[1]) - self.lengths[0]).abs() > tolerance
            || (distance(positions[1], positions[2]) - self.lengths[1]).abs() > tolerance
        {
            return None;
        }
        let middle_error = distance(positions[1], self.request.middle_target);
        let tip_error = distance(positions[2], self.request.tip_target);
        let orientation_error =
            self.request.tip_orientation.map(|wanted| angle(Quat::from_mat3(transforms[2].rot), wanted));
        let orientation_distance =
            orientation_error.unwrap_or(0.0) * tolerance / self.request.orientation_tolerance_rad;
        let score = middle_error * middle_error + tip_error * tip_error + orientation_distance * orientation_distance;
        if !score.is_finite() {
            return None;
        }
        let converged = middle_error <= tolerance
            && tip_error <= tolerance
            && orientation_error.is_none_or(|e| e <= self.request.orientation_tolerance_rad);
        let candidate = Candidate {
            solution: Solution {
                rotations,
                root: positions[0],
                middle: positions[1],
                tip: positions[2],
                middle_error,
                tip_error,
                orientation_error,
                evaluations: self.evaluations,
                converged,
                budget_exhausted: false,
            },
            score,
        };
        if self.best.is_none_or(|best| converged && !best.solution.converged || score < best.score) {
            self.best = Some(candidate);
        }
        Some(candidate)
    }
}

/// Every visited candidate obeys the supplied rotation rules. Search exhaustion
/// returns residuals; only `converged` certifies the requested tolerances. The tip
/// stays exactly at its input local rotation when no world orientation is requested.
pub fn solve(request: &Request) -> Result<Solution, String> {
    if request.max_evaluations == 0 || request.max_evaluations > MAX_EVALUATIONS {
        return Err(format!("limited IK evaluation budget must be 1..{MAX_EVALUATIONS}"));
    }
    if !request.tolerance_m.is_finite()
        || !(1e-12..=1e6).contains(&request.tolerance_m)
        || !request.orientation_tolerance_rad.is_finite()
        || !(1e-12..=std::f64::consts::PI).contains(&request.orientation_tolerance_rad)
        || !finite(request.middle_target)
        || !finite(request.tip_target)
        || !valid_transform(request.parent)
    {
        return Err("limited IK targets, tolerances, or parent transform are invalid".into());
    }
    for joint in request.joints {
        if !finite(joint.pivot)
            || !finite(joint.translation)
            || !joint.scale.is_finite()
            || joint.scale <= 0.0
            || !valid_quaternion(joint.rotation)
        {
            return Err(
                "limited IK joint poses require finite pivots/translations, positive scales, and unit rotations".into(),
            );
        }
    }
    for limit in request.limits.into_iter().flatten() {
        limit.validate()?;
    }
    for q in request.analytic_seed.into_iter().chain(request.tip_orientation) {
        if !valid_quaternion(q) {
            return Err("limited IK seed and target orientations must be finite unit quaternions".into());
        }
    }
    if request.tip_orientation.is_none() {
        let tip = request.joints[2].rotation;
        if angle(tip, project(request.limits[2], tip)?) > ANGULAR_TOLERANCE {
            return Err(
                "limited IK cannot retain an input tip rotation outside its limit without a requested orientation"
                    .into(),
            );
        }
    }
    let current = request.joints.map(|joint| joint.rotation);
    let (_, initial) = fk(request, current)?;
    let lengths = [distance(initial[0], initial[1]), distance(initial[1], initial[2])];
    if lengths.into_iter().any(|length| length == 0.0 || !length.is_finite()) {
        return Err("limited IK requires two nonzero representable current segments".into());
    }
    let axes = [
        unit(request.joints[1].pivot + request.joints[1].translation - request.joints[0].pivot)?,
        unit(request.joints[2].pivot + request.joints[2].translation - request.joints[1].pivot)?,
    ];
    let mut search = Search { request, initial, lengths, evaluations: 0, best: None };
    let current_seed = [current[0], current[1]];
    let mut seeds = vec![current_seed, request.analytic_seed];
    // Explore null-space twists from the constructive analytic pose as well as
    // ordinary coordinates. Projection can otherwise make a feasible twist
    // inaccessible through individually improving Cartesian rotation trials.
    for turn in [std::f32::consts::FRAC_PI_2, -std::f32::consts::FRAC_PI_2, std::f32::consts::PI] {
        let root_twist = Quat::from_axis_angle(axes[0], turn);
        seeds.push([request.analytic_seed[0] * root_twist, conjugate(root_twist) * request.analytic_seed[1]]);
        seeds.push([request.analytic_seed[0], request.analytic_seed[1] * Quat::from_axis_angle(axes[1], turn)]);
    }
    let mut starting = Vec::new();
    for seed in seeds {
        if let Some(candidate) = search.evaluate(seed) {
            if candidate.solution.converged {
                break;
            }
            starting.push(candidate);
        }
    }
    let minimum_step = (request.tolerance_m / (8.0 * (lengths[0] + lengths[1])))
        .min(request.orientation_tolerance_rad / 8.0)
        .clamp(1e-12, 0.01);
    let mut active = starting;
    let mut steps = vec![std::f64::consts::FRAC_PI_2; active.len()];
    // Round-robin seeds prevent the first local basin consuming the entire budget.
    while search.evaluations < request.max_evaluations && !search.best.is_some_and(|c| c.solution.converged) {
        let mut unfinished = false;
        for i in 0..active.len() {
            if steps[i] < minimum_step {
                continue;
            }
            unfinished = true;
            let before = active[i];
            let base = [before.solution.rotations[0], before.solution.rotations[1]];
            let mut best = before;
            for kind in 0..9 {
                for sign in [-1.0, 1.0] {
                    let amount = (steps[i] * sign) as f32;
                    let mut trial = base;
                    if kind < 6 {
                        let axis = match kind % 3 {
                            0 => Vec3::new(1.0, 0.0, 0.0),
                            1 => Vec3::new(0.0, 1.0, 0.0),
                            _ => Vec3::new(0.0, 0.0, 1.0),
                        };
                        trial[kind / 3] = Quat::from_axis_angle(axis, amount) * trial[kind / 3];
                    } else if kind < 8 {
                        let joint = kind - 6;
                        trial[joint] = trial[joint] * Quat::from_axis_angle(axes[joint], amount);
                    } else {
                        let twist = Quat::from_axis_angle(axes[0], amount);
                        trial = [trial[0] * twist, conjugate(twist) * trial[1]];
                    }
                    if let Some(candidate) = search.evaluate(trial) {
                        if candidate.solution.converged || candidate.score < best.score {
                            best = candidate;
                        }
                    }
                    if search.evaluations >= request.max_evaluations
                        || search.best.is_some_and(|c| c.solution.converged)
                    {
                        break;
                    }
                }
                if search.evaluations >= request.max_evaluations || search.best.is_some_and(|c| c.solution.converged) {
                    break;
                }
            }
            active[i] = best;
            if best.score >= before.score {
                steps[i] *= 0.5;
            }
            if search.evaluations >= request.max_evaluations || search.best.is_some_and(|c| c.solution.converged) {
                break;
            }
        }
        if !unfinished {
            break;
        }
    }
    let mut result =
        search.best.ok_or("limited IK found no finite projected pose preserving root and segment lengths")?.solution;
    result.evaluations = search.evaluations;
    result.budget_exhausted = !result.converged && search.evaluations >= request.max_evaluations;
    Ok(result)
}
