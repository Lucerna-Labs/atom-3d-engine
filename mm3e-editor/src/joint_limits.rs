//! Explicit rotational constraints on final local joint deltas.
use crate::{
    animation::{self, AnimationSample, Clip, Joint, Target},
    model::{array, vec, Document, V3},
};
use mm3e_kit::{
    rotation_limit::{Projection, RotationLimit},
    Quat, Transform,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LimitMode {
    #[default]
    Reject,
    Project,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum JointRotationLimit {
    Hinge {
        axis: V3,
        min_degrees: f64,
        max_degrees: f64,
        #[serde(default)]
        mode: LimitMode,
    },
    SwingTwist {
        axis: V3,
        swing_degrees: f64,
        twist_min_degrees: f64,
        twist_max_degrees: f64,
        #[serde(default)]
        mode: LimitMode,
    },
}
impl JointRotationLimit {
    pub(crate) fn core(&self) -> RotationLimit {
        match self {
            Self::Hinge { axis, min_degrees, max_degrees, .. } => {
                RotationLimit::Hinge { axis: vec(*axis), min: min_degrees.to_radians(), max: max_degrees.to_radians() }
            }
            Self::SwingTwist { axis, swing_degrees, twist_min_degrees, twist_max_degrees, .. } => {
                RotationLimit::SwingTwist {
                    axis: vec(*axis),
                    swing: swing_degrees.to_radians(),
                    twist_min: twist_min_degrees.to_radians(),
                    twist_max: twist_max_degrees.to_radians(),
                }
            }
        }
    }
    pub fn validate(&self) -> Result<(), String> {
        self.core().validate()
    }
    pub(crate) fn mode(&self) -> LimitMode {
        match self {
            Self::Hinge { mode, .. } | Self::SwingTwist { mode, .. } => *mode,
        }
    }
}
pub(crate) fn observation(joint: &Joint, local: Transform, projection: &Projection) -> Value {
    let mode = joint.rotation_limit.as_ref().expect("limited joint").mode();
    json!({"joint":joint.id,"mode":mode,"violated":projection.violated,"would_reject":projection.violated&&mode==LimitMode::Reject,"would_project":projection.violated&&mode==LimitMode::Project,"limit":joint.rotation_limit,
        "requested_swing_degrees":projection.swing.to_degrees(),"requested_twist_degrees":projection.twist.to_degrees(),"angular_error_degrees":projection.angular_error.to_degrees(),"singular_twist":projection.singular_twist,"quantization_adjusted":projection.quantization_adjusted,"quantization_adjustment_degrees":projection.quantization_adjustment.to_degrees(),
        "requested_basis":local.rot.cols.map(array),"projected_rotation_degrees":crate::ik::euler(projection.rotation.to_mat3()),"projected_basis":projection.rotation.to_mat3().cols.map(array)})
}
/// Rebuild the delta around its existing pivot, retaining the sampled translation
/// and uniform scale. Altering only the basis would move an offset joint pivot.
pub(crate) fn with_rotation(local: Transform, pivot: V3, rotation: Quat) -> Transform {
    let old = local.rot;
    let next = rotation.to_mat3();
    // Evaluate the correction in f64 so large canceling pivot offsets do not add
    // an avoidable f32 subtraction before the final engine representation.
    let offset = |matrix: mm3e_kit::Mat3, axis: usize| -> f64 {
        matrix
            .cols
            .iter()
            .zip(pivot)
            .map(|(column, x)| f64::from(array(*column)[axis]) * f64::from(x) * f64::from(local.scale))
            .sum()
    };
    let pos = std::array::from_fn(|i| (f64::from(array(local.pos)[i]) + offset(old, i) - offset(next, i)) as f32);
    Transform::new(vec(pos), next, local.scale)
}
pub(crate) fn apply(joint: &Joint, local: Transform) -> Result<(Transform, Option<Value>), String> {
    let Some(limit) = &joint.rotation_limit else {
        return Ok((local, None));
    };
    let projection = limit
        .core()
        .project(Quat::from_mat3(local.rot))
        .map_err(|e| format!("joint {} rotation limit: {e}", joint.id))?;
    let mut report = observation(joint, local, &projection);
    if projection.violated && limit.mode() == LimitMode::Reject {
        return Err(format!("joint {} violates its rotation limit by {} degrees; inspect joint_limit_state or explicitly choose project mode",joint.id,projection.angular_error.to_degrees()));
    }
    let applied = if projection.violated { with_rotation(local, joint.pivot, projection.rotation) } else { local };
    report["applied"] = json!(projection.violated);
    let before = local.to_world(vec(joint.pivot));
    let after = applied.to_world(vec(joint.pivot));
    report["local_pivot_drift_m"] = json!(array(before)
        .into_iter()
        .zip(array(after))
        .map(|(a, b)| (f64::from(a) - f64::from(b)).powi(2))
        .sum::<f64>()
        .sqrt());
    Ok((applied, Some(report)))
}
pub(crate) fn local_delta(
    document: &Document,
    clip: &Clip,
    time: f32,
    joint: &Joint,
    resolved: Option<&crate::layering::Resolved>,
) -> Transform {
    let target = Target::Joint { id: joint.id.clone() };
    if let Some(resolved) = resolved {
        return resolved.transforms.get(&target).copied().unwrap_or(Transform::IDENTITY);
    }
    let _ = document;
    clip.tracks
        .iter()
        .find(|t| t.target == target)
        .map(|t| animation::delta(t, time, joint.pivot))
        .unwrap_or(Transform::IDENTITY)
}
pub fn inspect(document: &Document, sample: &AnimationSample) -> Result<Value, String> {
    animation::validate(document)?;
    let (clip, time) = animation::sample_time(document, sample)?;
    let resolved = (!clip.layers.is_empty()).then(|| crate::layering::resolve(document, clip, time)).transpose()?;
    let mut reports = vec![];
    for joint in &document.joints {
        if let Some(limit) = &joint.rotation_limit {
            let local = local_delta(document, clip, time, joint, resolved.as_ref());
            let projected = limit
                .core()
                .project(Quat::from_mat3(local.rot))
                .map_err(|e| format!("joint {} rotation limit: {e}", joint.id))?;
            reports.push(observation(joint, local, &projected));
        }
    }
    Ok(
        json!({"clip":sample.clip,"requested_time":sample.time,"time":time,"joints":reports,"semantics":"Final layered local rotations measured before parent FK; project mode affects evaluated geometry but never rewrites authored keys; limits constrain rotation only, not translation, scale, collisions or anatomical suitability"}),
    )
}
