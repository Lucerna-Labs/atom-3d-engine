//! Target-based authoring of native joint keys. Solves a current three-joint chain,
//! then verifies the resulting keys through the real forward-kinematics evaluator.
use crate::{
    animation::{self, AnimationSample, Interpolation, MotionTrack, Playback, Target, TransformKey},
    model::{array, identifier, range, vec, vector, Document, Pass, V3},
};
use mm3e_kit::{
    ik as kernel,
    vec::{Mat3, Quat, Transform},
    Vec3,
};
use mm3e_orchestrator::anim::Track;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum UnreachablePolicy {
    #[default]
    Reject,
    Clamp,
}
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LimitPolicy {
    #[default]
    Reject,
    BestFeasible,
}
fn limit_budget() -> u32 {
    4096
}

fn tolerance() -> f32 {
    0.0001
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct IkRequest {
    pub clip: String,
    /// Authored seconds, inside the existing clip. No implicit clamping or looping.
    pub time: f32,
    pub root_joint: String,
    pub middle_joint: String,
    pub tip_joint: String,
    pub target: V3,
    /// World point selecting the bend side/plane.
    pub pole: V3,
    #[serde(default)]
    pub unreachable: UnreachablePolicy,
    /// Optional world joint-frame rotation, in the native Rx*Ry*Rz degree convention.
    /// Without it, the tip's existing local keys remain untouched.
    #[serde(default)]
    pub tip_world_rotation_degrees: Option<V3>,
    /// Only affects newly created tracks; existing track easing remains unchanged.
    #[serde(default)]
    pub new_track_easing: Interpolation,
    #[serde(default = "tolerance")]
    pub tolerance_m: f32,
    #[serde(default)]
    pub limit_policy: LimitPolicy,
    #[serde(default = "limit_budget")]
    pub limit_evaluations: u32,
}

fn length(a: Vec3, b: Vec3) -> f64 {
    let a = array(a);
    let b = array(b);
    a.into_iter().zip(b).map(|(a, b)| (f64::from(a) - f64::from(b)).powi(2)).sum::<f64>().sqrt()
}

fn rotation_error_degrees(a: Quat, b: Quat) -> f64 {
    let a = [a.x, a.y, a.z, a.w].map(f64::from);
    let b = [b.x, b.y, b.z, b.w].map(f64::from);
    let an = a.iter().map(|v| v * v).sum::<f64>().sqrt();
    let bn = b.iter().map(|v| v * v).sum::<f64>().sqrt();
    let difference = a.iter().zip(b).map(|(a, b)| (a / an - b / bn).powi(2)).sum::<f64>().sqrt();
    let sum = a.iter().zip(b).map(|(a, b)| (a / an + b / bn).powi(2)).sum::<f64>().sqrt();
    (4.0 * difference.min(sum).atan2(difference.max(sum))).to_degrees()
}

pub(crate) fn euler(rot: Mat3) -> V3 {
    // Reconstruct a consistent orthogonal matrix in f64. Independently rounded
    // f32 entries otherwise amplify error when two Euler axes nearly coincide.
    let q = Quat::from_mat3(rot);
    let [qx, qy, qz, qw] = [q.x, q.y, q.z, q.w].map(f64::from);
    let norm = (qx * qx + qy * qy + qz * qz + qw * qw).sqrt();
    let (qx, qy, qz, qw) = (qx / norm, qy / norm, qz / norm, qw / norm);
    let m00 = 1.0 - 2.0 * (qy * qy + qz * qz);
    let m01 = 2.0 * (qx * qy - qz * qw);
    let m02 = 2.0 * (qx * qz + qy * qw);
    let m11 = 1.0 - 2.0 * (qx * qx + qz * qz);
    let m12 = 2.0 * (qy * qz - qx * qw);
    let m21 = 2.0 * (qy * qz + qx * qw);
    let m22 = 1.0 - 2.0 * (qx * qx + qy * qy);
    // Rx*Ry*Rz: first row = [cy*cz,-cy*sz,sy]. atan2 keeps the
    // small cosine available near gimbal lock instead of losing it in asin.
    let cy = m00.hypot(m01);
    let y = m02.atan2(cy);
    // A quaternion-derived f32 matrix can retain several rounding ULPs in this
    // row at exact gimbal lock. Dividing that noise between x and z yields an
    // unrelated rotation; collapse the redundant axis within that precision.
    let (x, z) =
        if cy > 8.0 * f64::from(f32::EPSILON) { ((-m12).atan2(m22), (-m01).atan2(m00)) } else { (m21.atan2(m11), 0.0) };
    [x, y, z].map(|angle| angle.to_degrees() as f32)
}

fn translation_scale(document: &Document, clip: &animation::Clip, joint: &str, time: f32) -> Result<(V3, f32), String> {
    let target = Target::Joint { id: joint.into() };
    let values = if !clip.layers.is_empty() {
        let resolved = crate::layering::resolve(document, clip, time)?;
        let delta = resolved.transforms.get(&target).copied().unwrap_or(Transform::IDENTITY);
        let pivot = vec(document.joints.iter().find(|j| j.id == joint).ok_or("missing IK joint")?.pivot);
        (array(delta.to_world(pivot) - pivot), delta.scale)
    } else if let Some(track) = clip.tracks.iter().find(|track| track.target == target) {
        let mut positions = Track::new(track.easing.engine());
        let mut scales = Track::new(track.easing.engine());
        for key in &track.keys {
            positions = positions.key(key.time, vec(key.translation));
            scales = scales.key(key.time, key.scale);
        }
        (
            array(positions.try_sample(time).ok_or("empty IK source track")?),
            scales.try_sample(time).ok_or("empty IK scale track")?,
        )
    } else {
        ([0.0; 3], 1.0)
    };
    Ok(values)
}

fn local_key(
    document: &Document,
    clip: &animation::Clip,
    joint: &str,
    time: f32,
    world_rotation: Mat3,
    parent_rotation: Mat3,
) -> Result<TransformKey, String> {
    let (translation, scale) = translation_scale(document, clip, joint, time)?;
    let rotation = Quat::from_mat3(parent_rotation.transpose() * world_rotation).to_mat3();
    let rotation_degrees = euler(rotation);
    vector(rotation_degrees, "IK key rotation")?;
    Ok(TransformKey { time, translation, rotation_degrees, scale })
}

fn install(clip: &mut animation::Clip, joint: &str, key: TransformKey, easing: Interpolation) {
    let target = Target::Joint { id: joint.into() };
    if let Some(track) = clip.tracks.iter_mut().find(|track| track.target == target) {
        match track.keys.binary_search_by(|other| other.time.total_cmp(&key.time)) {
            Ok(index) => track.keys[index] = key,
            Err(index) => track.keys.insert(index, key),
        }
    } else {
        clip.tracks.push(MotionTrack { target, keys: vec![key], easing, pivot: None });
    }
}

pub fn solve(document: &mut Document, request: &IkRequest) -> Result<Value, String> {
    document.compile(&Pass::Beauty)?;
    for id in [&request.clip, &request.root_joint, &request.middle_joint, &request.tip_joint] {
        identifier(id)?;
    }
    vector(request.target, "IK target")?;
    vector(request.pole, "IK pole")?;
    if let Some(rotation) = request.tip_world_rotation_degrees {
        vector(rotation, "IK tip world rotation")?;
    }
    range(request.tolerance_m, 1e-6, 0.05, "IK verification tolerance")?;
    if request.limit_evaluations == 0 || request.limit_evaluations > mm3e_kit::limited_ik::MAX_EVALUATIONS {
        return Err(format!("IK limit_evaluations must be 1..={}", mm3e_kit::limited_ik::MAX_EVALUATIONS));
    }
    let clip_index = document
        .clips
        .iter()
        .position(|clip| clip.id == request.clip)
        .ok_or_else(|| format!("missing IK clip {}", request.clip))?;
    let clip = &document.clips[clip_index];
    range(request.time, 0.0, clip.duration, "IK authored time")?;
    let indices: BTreeMap<_, _> = document.joints.iter().enumerate().map(|(i, j)| (j.id.as_str(), i)).collect();
    let chain = [&request.root_joint, &request.middle_joint, &request.tip_joint]
        .map(|id| indices.get(id.as_str()).copied().ok_or_else(|| format!("missing IK joint {id}")));
    let [root, middle, tip] = [chain[0].clone()?, chain[1].clone()?, chain[2].clone()?];
    if root == middle
        || root == tip
        || middle == tip
        || document.joints[middle].parent.as_ref() != Some(&request.root_joint)
        || document.joints[tip].parent.as_ref() != Some(&request.middle_joint)
    {
        return Err("IK requires three distinct joints in a direct root -> middle -> tip parent chain".into());
    }
    let sample = AnimationSample { clip: request.clip.clone(), time: request.time, playback: Playback::Clamp };
    let before = animation::evaluate_joints(document, &sample)?;
    let point = |evaluation: &animation::Evaluation, index: usize| {
        evaluation.joint_transforms[index].to_world(vec(document.joints[index].pivot))
    };
    let positions = [root, middle, tip].map(|index| point(&before, index));
    let policy = match request.unreachable {
        UnreachablePolicy::Reject => kernel::UnreachablePolicy::Reject,
        UnreachablePolicy::Clamp => kernel::UnreachablePolicy::Clamp,
    };
    let solution = kernel::solve_two_bone(
        positions[0],
        positions[1],
        positions[2],
        vec(request.target),
        vec(request.pole),
        policy,
    )?;
    let parent = document.joints[root]
        .parent
        .as_ref()
        .map(|id| before.joint_transforms[indices[id.as_str()]])
        .unwrap_or(Transform::IDENTITY);
    let root_world = (solution.root_rotation * Quat::from_mat3(before.joint_transforms[root].rot)).to_mat3();
    let middle_world =
        (solution.middle_rotation * solution.root_rotation * Quat::from_mat3(before.joint_transforms[middle].rot))
            .to_mat3();
    let constrained = [root, middle, tip].iter().any(|&i| document.joints[i].rotation_limit.is_some());
    let limited = if constrained {
        let joint_poses = [root, middle, tip].map(|i| -> Result<mm3e_kit::limited_ik::JointPose, String> {
            let joint = &document.joints[i];
            let (translation, scale) = translation_scale(document, clip, &joint.id, request.time)?;
            let raw = crate::joint_limits::local_delta(document, clip, request.time, joint, before.resolved.as_ref());
            let (applied, _) = crate::joint_limits::apply(joint, raw)?;
            Ok(mm3e_kit::limited_ik::JointPose {
                pivot: vec(joint.pivot),
                translation: vec(translation),
                scale,
                rotation: Quat::from_mat3(applied.rot),
            })
        });
        let [a, b, c] = joint_poses;
        let input = mm3e_kit::limited_ik::Request {
            parent,
            joints: [a?, b?, c?],
            limits: [root, middle, tip].map(|i| document.joints[i].rotation_limit.as_ref().map(|l| l.core())),
            analytic_seed: [
                Quat::from_mat3(parent.rot.transpose() * root_world),
                Quat::from_mat3(root_world.transpose() * middle_world),
            ],
            middle_target: solution.middle,
            tip_target: solution.effective_target,
            tip_orientation: request.tip_world_rotation_degrees.map(|angles| {
                let r = angles.map(f32::to_radians);
                Quat::from_euler(r[0], r[1], r[2])
            }),
            tolerance_m: f64::from(request.tolerance_m),
            orientation_tolerance_rad: 0.1f64.to_radians(),
            max_evaluations: request.limit_evaluations,
        };
        let result = mm3e_kit::limited_ik::solve(&input)?;
        if !result.converged && request.limit_policy == LimitPolicy::Reject {
            return Err(format!("no verified constrained IK target/pole/orientation solution found in {} evaluations: endpoint error {}m, middle error {}m, orientation error {:?}rad; this is not a proof of infeasibility",result.evaluations,result.tip_error,result.middle_error,result.orientation_error));
        }
        Some(result)
    } else {
        None
    };
    let mut keys = vec![
        (
            request.root_joint.clone(),
            local_key(document, clip, &request.root_joint, request.time, root_world, parent.rot)?,
        ),
        (
            request.middle_joint.clone(),
            local_key(document, clip, &request.middle_joint, request.time, middle_world, root_world)?,
        ),
    ];
    if let Some(angles) = request.tip_world_rotation_degrees {
        let r = angles.map(f32::to_radians);
        let desired = Quat::from_euler(r[0], r[1], r[2]).to_mat3();
        keys.push((
            request.tip_joint.clone(),
            local_key(document, clip, &request.tip_joint, request.time, desired, middle_world)?,
        ));
    }
    if let Some(limited) = &limited {
        for ((_, key), rotation) in keys.iter_mut().zip(limited.rotations) {
            key.rotation_degrees = euler(rotation.to_mat3());
        }
    }
    let mut candidate = document.clone();
    for (joint, key) in &keys {
        install(&mut candidate.clips[clip_index], joint, key.clone(), request.new_track_easing);
    }
    candidate.compile(&Pass::Beauty)?;
    let after = animation::evaluate_joints(&candidate, &sample)?;
    let achieved = [root, middle, tip].map(|index| point(&after, index));
    let endpoint_error = length(achieved[2], solution.effective_target);
    let middle_error = length(achieved[1], solution.middle);
    let allow_approximation = limited.is_some() && request.limit_policy == LimitPolicy::BestFeasible;
    let root_error = length(achieved[0], positions[0]);
    let lengths = [length(positions[0], positions[1]), length(positions[1], positions[2])];
    let final_lengths = [length(achieved[0], achieved[1]), length(achieved[1], achieved[2])];
    if (!allow_approximation
        && (endpoint_error > f64::from(request.tolerance_m) || middle_error > f64::from(request.tolerance_m)))
        || root_error > f64::from(request.tolerance_m)
        || lengths.iter().zip(final_lengths).any(|(a, b)| (a - b).abs() > f64::from(request.tolerance_m))
    {
        return Err(format!("IK keys did not meet forward verification: target error {endpoint_error}m, root drift {root_error}m, lengths {lengths:?} -> {final_lengths:?}; tolerance {}m",request.tolerance_m));
    }
    let tip_orientation_error = request.tip_world_rotation_degrees.map(|angles| {
        let radians = angles.map(f32::to_radians);
        let desired = Quat::from_euler(radians[0], radians[1], radians[2]);
        let actual = Quat::from_mat3(after.joint_transforms[tip].rot);
        rotation_error_degrees(desired, actual)
    });
    if !allow_approximation && tip_orientation_error.is_some_and(|angle| angle > 0.1) {
        return Err(format!("IK tip orientation verification failed: {tip_orientation_error:?} degrees"));
    }
    if let Some(limited) = &limited {
        for ((_, key), expected) in keys.iter().zip(limited.rotations) {
            let r = key.rotation_degrees.map(f32::to_radians);
            if rotation_error_degrees(Quat::from_euler(r[0], r[1], r[2]), expected) > 0.1 {
                return Err("limited IK rotation changed beyond tolerance during Euler key serialization".into());
            }
        }
        let reencode_error = length(achieved[1], limited.middle).max(length(achieved[2], limited.tip));
        if reencode_error > f64::from(request.tolerance_m) {
            return Err(format!("limited IK candidate changed by {reencode_error}m after native key serialization"));
        }
        let written: std::collections::BTreeSet<_> = keys.iter().map(|(id, _)| id.as_str()).collect();
        if after
            .limit_observations
            .iter()
            .any(|r| r["joint"].as_str().is_some_and(|id| written.contains(id)) && r["applied"] == true)
        {
            return Err("new IK keys require extra limit projection after native serialization".into());
        }
    }
    // Verify current skin/face/garment geometry without demanding stale cloth caches.
    candidate.compile_at_base(&Pass::Beauty, Some(&sample))?;
    let stale_cloth: Vec<_> = candidate
        .cloths
        .iter()
        .filter(|asset| asset.cache.is_some())
        .filter_map(|asset| match crate::cloth::cache_fresh(&candidate, asset) {
            Ok(true) => None,
            _ => Some(asset.id.clone()),
        })
        .collect();
    let mut result = json!({"clip":request.clip,"time":request.time,"chain":[request.root_joint,request.middle_joint,request.tip_joint],
        "requested_target":request.target,"effective_target":array(solution.effective_target),"achieved_tip":array(achieved[2]),
        "solved_middle":array(achieved[1]),"target_error_m":endpoint_error,"requested_target_error_m":length(achieved[2],vec(request.target)),
        "root_drift_m":root_error,"middle_error_m":middle_error,"desired_middle":array(solution.middle),"segment_lengths_m":lengths,"achieved_segment_lengths_m":final_lengths,
        "clamped":solution.clamped,"pole_fallback":solution.pole_fallback,"target_direction_fallback":solution.target_direction_fallback,
        "tip_orientation_error_degrees":tip_orientation_error,"stale_cloth_caches":stale_cloth,
        "keyframes":keys.iter().map(|(joint,key)|json!({"joint":joint,"key":key})).collect::<Vec<_>>(),
        "semantics":["Current world-space chain; fixed root position and segment lengths; no stretch",
            "Writes/replaces root and middle rotation keys at this time while retaining sampled local translation/scale and other keys",
            "Existing easing is retained; added keys affect interpolation in neighboring intervals",
            "Descendants follow edited parents; tip local orientation is retained unless an explicit world orientation is supplied",
            "Only explicitly configured rotational limits are enforced; no anatomical calibration, self-collision avoidance or retargeting is implied"]});
    if let Some(limited) = limited {
        result["limit_search"] = json!({"policy":request.limit_policy,"evaluations":limited.evaluations,"budget_exhausted":limited.budget_exhausted,
            "within_tolerance":endpoint_error<=f64::from(request.tolerance_m)&&middle_error<=f64::from(request.tolerance_m)&&tip_orientation_error.is_none_or(|e|e<=0.1),
            "approximation_accepted":allow_approximation && (endpoint_error>f64::from(request.tolerance_m)||middle_error>f64::from(request.tolerance_m)||tip_orientation_error.is_some_and(|e|e>0.1)),
            "semantics":"Best finite limit-compliant pose visited by bounded deterministic search; no global nearest-pose or infeasibility claim; clamped/effective_target retain their original radial reach meaning"});
    }
    *document = candidate;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn euler_round_trip_including_gimbal_neighborhoods() {
        let half_pi = std::f32::consts::FRAC_PI_2;
        for x in [-2.0_f32, 0.0, 0.71] {
            for y in [
                -half_pi,
                -half_pi + 1e-7,
                -half_pi + 6e-6,
                -half_pi + 1e-4,
                0.0,
                half_pi - 1e-4,
                half_pi - 6e-6,
                half_pi - 1e-7,
                half_pi,
            ] {
                for z in [-1.8_f32, 0.0, 2.2] {
                    let source = Quat::from_euler(x, y, z).to_mat3();
                    let angles = euler(source).map(f32::to_radians);
                    let result = Quat::from_euler(angles[0], angles[1], angles[2]).to_mat3();
                    let maximum = source
                        .cols
                        .into_iter()
                        .zip(result.cols)
                        .flat_map(|(a, b)| array(a - b))
                        .map(f32::abs)
                        .fold(0.0_f32, f32::max);
                    assert!(maximum < 5e-6, "{x},{y},{z}: {maximum}");
                }
            }
        }
    }
}
