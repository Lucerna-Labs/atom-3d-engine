//! Rotation limits in a joint's world-rest coordinates, relative to identity.
//!
//! The rule factors `q = swing * twist`, limits the swing angle along its
//! existing axis, and chooses the nearest cyclic endpoint of the twist interval.
//! This separated rule is not a global nearest projection on SO(3) for a coupled
//! swing cone and twist interval. Quaternion signs represent the same rotation.
use crate::{Quat, Vec3};
use std::f64::consts::{PI, TAU};

/// Projective angular compliance in radians. Thirty-two f32 machine epsilons
/// accommodate stored quaternion rounding, including ill-conditioned twist
/// coordinates near a perpendicular half-turn. This is an SO(3) angle tolerance,
/// not permission to enlarge each parameter interval by an arbitrary amount.
pub const ANGULAR_TOLERANCE: f64 = 32.0 * f32::EPSILON as f64;
/// Half-angle magnitude below which twist has no reliable f32 decomposition.
/// The deterministic zero-twist gauge changes a near-singular request by less
/// than `ANGULAR_TOLERANCE` before any authored cone restriction is applied.
pub const SINGULAR_TWIST_TOLERANCE: f64 = 4.0 * f32::EPSILON as f64;
/// Maximum additional SO(3) correction allowed while finding a stable f32
/// interior representation. This correction is reported, never folded into the
/// compliance tolerance. Larger/unsuccessful repairs return a representability error.
pub const MAX_QUANTIZATION_ADJUSTMENT: f64 = 0.01;
pub const MAX_QUANTIZATION_REPAIRS: usize = 8;
const UNIT_NORM_TOLERANCE: f64 = 4.0 * f32::EPSILON as f64;

#[derive(Clone, Copy, Debug)]
pub enum RotationLimit {
    Hinge { axis: Vec3, min: f64, max: f64 },
    SwingTwist { axis: Vec3, swing: f64, twist_min: f64, twist_max: f64 },
}

#[derive(Clone, Copy, Debug)]
pub struct Projection {
    pub rotation: Quat,
    pub violated: bool,
    /// Geodesic angle from the requested rotation to this rule's projection,
    /// before the tolerance-preserving no-op. May be nonzero below tolerance
    /// even when `rotation` retains the original quaternion exactly.
    pub angular_error: f64,
    /// Requested swing angle in [0, pi], under the reported singular gauge.
    pub swing: f64,
    /// Requested principal twist in [-pi, pi]; zero at a singular twist.
    pub twist: f64,
    pub singular_twist: bool,
    pub quantization_adjusted: bool,
    /// Additional angle from the ideal separated projection to the stored f32
    /// output, when a conservative interior repair was necessary. Both the
    /// quaternion and its native Quat -> Mat3 -> Quat readback were checked.
    pub quantization_adjustment: f64,
}

impl RotationLimit {
    pub fn validate(&self) -> Result<(), String> {
        self.parameters().map(|_| ())
    }

    fn parameters(&self) -> Result<(V, f64, f64, f64), String> {
        let (axis, swing, min, max) = match *self {
            Self::Hinge { axis, min, max } => (axis, 0.0, min, max),
            Self::SwingTwist { axis, swing, twist_min, twist_max } => (axis, swing, twist_min, twist_max),
        };
        let axis = V::from(axis).unit().ok_or("rotation-limit axis must be finite and nonzero")?;
        if !swing.is_finite() || !(0.0..=PI).contains(&swing) {
            return Err("rotation-limit swing must be finite and in 0..pi radians".into());
        }
        if !min.is_finite() || !max.is_finite() || min < -PI || max > PI || min > 0.0 || max < 0.0 || min > max {
            return Err("rotation-limit twist bounds must be finite, ordered, within [-pi,pi], and contain zero".into());
        }
        Ok((axis, swing, min, max))
    }

    pub fn project(&self, requested: Quat) -> Result<Projection, String> {
        let (axis, swing_max, twist_min, twist_max) = self.parameters()?;
        let (q, norm) = Q::from(requested).unit().ok_or("requested rotation must be finite and nonzero")?;
        let q = q.canonical();
        let requested_angles = decompose(q, axis)?;
        let mut bounded_swing = requested_angles.swing.min(swing_max);
        let mut bounded_twist = nearest_twist(requested_angles.twist, twist_min, twist_max);
        let ideal = compose(requested_angles.direction, bounded_swing, axis, bounded_twist)?;
        let ideal_error = q.angle_to(ideal);
        let initial = if ideal_error <= ANGULAR_TOLERANCE {
            if (norm - 1.0).abs() <= UNIT_NORM_TOLERANCE {
                requested
            } else {
                q.to_quat()
            }
        } else {
            ideal.to_quat()
        };
        for repair in 0..=MAX_QUANTIZATION_REPAIRS {
            let rotation = if repair == 0 {
                initial
            } else {
                compose(requested_angles.direction, bounded_swing, axis, bounded_twist)?.to_quat()
            };
            let stored = Q::from(rotation).unit().ok_or("nonfinite quantized rotation-limit projection")?.0.canonical();
            let adjustment = if repair == 0 { 0.0 } else { ideal.angle_to(stored) };
            if adjustment > MAX_QUANTIZATION_ADJUSTMENT {
                return Err(representability_error());
            }
            let (error, readback) = quantized_compliance(rotation, axis, swing_max, twist_min, twist_max)?;
            if error <= ANGULAR_TOLERANCE {
                let angular_error = if repair == 0 { ideal_error } else { q.angle_to(stored) };
                return Ok(Projection {
                    rotation,
                    violated: angular_error > ANGULAR_TOLERANCE,
                    angular_error,
                    swing: requested_angles.swing,
                    twist: requested_angles.twist,
                    singular_twist: requested_angles.singular,
                    quantization_adjusted: repair != 0,
                    quantization_adjustment: adjustment,
                });
            }
            // Move strictly toward the interval's interior, based on measured
            // output overshoot. Never enlarge a limit or the angular tolerance.
            let twist_error = cyclic_distance(readback.twist, nearest_twist(readback.twist, twist_min, twist_max));
            let middle = 0.5 * (twist_min + twist_max);
            if twist_error > 0.0 && bounded_twist != middle {
                let step = (2.0 * twist_error + 2.0 * ANGULAR_TOLERANCE).min((middle - bounded_twist).abs());
                bounded_twist += (middle - bounded_twist).signum() * step;
            } else {
                // A zero-width/unrepresentable twist interior needs a slightly
                // smaller swing to improve conditioning. This remains bounded;
                // failure is explicit rather than a silent large pose change.
                let step = 2.0 * ANGULAR_TOLERANCE * (1u32 << repair) as f64;
                bounded_swing = (bounded_swing - step).max(0.0);
            }
        }
        Err(representability_error())
    }
}

fn representability_error() -> String {
    format!("rotation-limit projection cannot be represented stably in f32 within {MAX_QUANTIZATION_REPAIRS} conservative repairs and {MAX_QUANTIZATION_ADJUSTMENT} radians additional correction")
}
#[derive(Clone, Copy)]
struct Angles {
    direction: V,
    swing: f64,
    twist: f64,
    singular: bool,
}
fn decompose(q: Q, axis: V) -> Result<Angles, String> {
    let along = q.v.dot(axis);
    let twist_norm = q.w.hypot(along);
    if twist_norm <= SINGULAR_TWIST_TOLERANCE {
        // At a perpendicular half-turn every twist gauge can reconstruct the
        // same rotation. Choose zero twist and a sign-canonical perpendicular
        // requested axis, never a random Cartesian fallback.
        let direction = (q.v - axis * along).unit().ok_or("degenerate singular swing axis")?;
        return Ok(Angles { direction, swing: PI, twist: 0.0, singular: true });
    }
    let twist_q = Q { v: axis * (along / twist_norm), w: q.w / twist_norm };
    let swing_q = q.multiply(twist_q.conjugate());
    let perpendicular = swing_q.v - axis * swing_q.v.dot(axis);
    let length = perpendicular.norm();
    let direction = if length == 0.0 { V::ZERO } else { perpendicular * (1.0 / length) };
    let twist = 2.0 * along.atan2(q.w);
    Ok(Angles {
        direction,
        swing: 2.0 * length.atan2(twist_norm),
        twist: if twist == 0.0 { 0.0 } else { twist },
        singular: false,
    })
}
fn compose(swing_axis: V, swing: f64, twist_axis: V, twist: f64) -> Result<Q, String> {
    Q::axis_angle(swing_axis, swing)
        .multiply(Q::axis_angle(twist_axis, twist))
        .unit()
        .map(|(q, _)| q.canonical())
        .ok_or_else(|| "rotation-limit projection is not finite".into())
}
fn quantized_compliance(
    rotation: Quat,
    axis: V,
    swing_max: f64,
    twist_min: f64,
    twist_max: f64,
) -> Result<(f64, Angles), String> {
    let mut worst = None;
    for readback in [rotation, Quat::from_mat3(rotation.to_mat3())] {
        let q = Q::from(readback).unit().ok_or("rotation-limit native readback is not finite")?.0.canonical();
        let angles = decompose(q, axis)?;
        let bounded = compose(
            angles.direction,
            angles.swing.min(swing_max),
            axis,
            nearest_twist(angles.twist, twist_min, twist_max),
        )?;
        let error = q.angle_to(bounded);
        if worst.as_ref().is_none_or(|(previous, _)| error > *previous) {
            worst = Some((error, angles));
        }
    }
    Ok(worst.expect("two rotation representations were checked"))
}

fn cyclic_distance(a: f64, b: f64) -> f64 {
    let positive = (a - b).rem_euclid(TAU);
    positive.min(TAU - positive)
}
fn nearest_twist(angle: f64, min: f64, max: f64) -> f64 {
    if (min..=max).contains(&angle) {
        angle
    } else if cyclic_distance(angle, min) <= cyclic_distance(angle, max) {
        // Ties choose the lower endpoint; +pi and -pi remain equivalent.
        min
    } else {
        max
    }
}

#[derive(Clone, Copy)]
struct V([f64; 3]);
impl V {
    const ZERO: Self = Self([0.0; 3]);
    fn dot(self, other: Self) -> f64 {
        self.0.iter().zip(other.0).map(|(a, b)| a * b).sum()
    }
    fn norm(self) -> f64 {
        self.0[0].hypot(self.0[1]).hypot(self.0[2])
    }
    fn unit(self) -> Option<Self> {
        let norm = self.norm();
        (self.0.iter().all(|x| x.is_finite()) && norm > 0.0).then(|| self * (1.0 / norm))
    }
    fn cross(self, other: Self) -> Self {
        let [x, y, z] = self.0;
        let [a, b, c] = other.0;
        Self([y * c - z * b, z * a - x * c, x * b - y * a])
    }
}
impl From<Vec3> for V {
    fn from(value: Vec3) -> Self {
        Self([f64::from(value.x), f64::from(value.y), f64::from(value.z)])
    }
}
impl std::ops::Add for V {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(std::array::from_fn(|i| self.0[i] + other.0[i]))
    }
}
impl std::ops::Sub for V {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(std::array::from_fn(|i| self.0[i] - other.0[i]))
    }
}
impl std::ops::Mul<f64> for V {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self(self.0.map(|x| x * scalar))
    }
}

#[derive(Clone, Copy)]
struct Q {
    v: V,
    w: f64,
}
impl From<Quat> for Q {
    fn from(value: Quat) -> Self {
        Self { v: V([f64::from(value.x), f64::from(value.y), f64::from(value.z)]), w: f64::from(value.w) }
    }
}
impl Q {
    const IDENTITY: Self = Self { v: V::ZERO, w: 1.0 };
    fn unit(self) -> Option<(Self, f64)> {
        let norm = self.v.norm().hypot(self.w);
        (self.v.0.iter().all(|x| x.is_finite()) && self.w.is_finite() && norm > 0.0)
            .then(|| (Self { v: self.v * (1.0 / norm), w: self.w / norm }, norm))
    }
    fn canonical(self) -> Self {
        let first_nonzero = std::iter::once(self.w).chain(self.v.0).find(|&x| x != 0.0).unwrap_or(1.0);
        if first_nonzero < 0.0 {
            Self { v: self.v * -1.0, w: -self.w }
        } else {
            self
        }
    }
    fn conjugate(self) -> Self {
        Self { v: self.v * -1.0, w: self.w }
    }
    fn multiply(self, other: Self) -> Self {
        Self {
            v: other.v * self.w + self.v * other.w + self.v.cross(other.v),
            w: self.w * other.w - self.v.dot(other.v),
        }
    }
    fn axis_angle(axis: V, angle: f64) -> Self {
        if angle == 0.0 {
            return Self::IDENTITY;
        }
        let half = 0.5 * angle;
        Self { v: axis * half.sin(), w: half.cos() }
    }
    fn angle_to(self, other: Self) -> f64 {
        // Quaternion chord lengths avoid acos losing all small-angle precision.
        // The shorter of q-r and q+r selects the projective (sign-free) angle.
        let difference = (self.v - other.v).norm().hypot(self.w - other.w);
        let sum = (self.v + other.v).norm().hypot(self.w + other.w);
        4.0 * difference.min(sum).atan2(difference.max(sum))
    }
    fn to_quat(self) -> Quat {
        Quat { x: self.v.0[0] as f32, y: self.v.0[1] as f32, z: self.v.0[2] as f32, w: self.w as f32 }
    }
}
