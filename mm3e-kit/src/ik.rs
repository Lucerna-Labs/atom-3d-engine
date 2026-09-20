//! Constant-work, two-segment inverse kinematics in world coordinates.
//!
//! This geometric solver preserves the two measured segment lengths. It supplies
//! world-space rotation corrections, not an authored skeleton, joint limits, twist
//! limits, collision avoidance, or a general constraint solver. No inputs are edited.

use crate::{Quat, Vec3};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnreachablePolicy {
    Reject,
    /// Project the requested target onto the exact radial reach interval. Segments
    /// keep their lengths; `clamped` and `residual` explicitly report this projection.
    Clamp,
}

#[derive(Clone, Copy, Debug)]
pub struct TwoBoneIk {
    pub middle: Vec3,
    pub tip: Vec3,
    /// Rotate the current chain about the original root by this correction first.
    pub root_rotation: Quat,
    /// After the root correction, rotate the distal segment about the solved middle.
    /// This correction is in WORLD axes, not middle-joint local axes.
    pub middle_rotation: Quat,
    pub requested_target: Vec3,
    pub effective_target: Vec3,
    /// Distance between requested target and the returned effective endpoint.
    pub residual: f64,
    pub clamped: bool,
    /// A collinear/coincident pole used the existing bend, or a deterministic axis.
    /// At an equal-length full fold, a coincident pole instead retains the first bone.
    pub pole_fallback: bool,
    /// The requested target was exactly at the root. Clamping then uses the current
    /// root-to-tip direction, or the first segment if the current chain is fully folded.
    pub target_direction_fallback: bool,
    pub segment_lengths: [f64; 2],
}

/// Solve a two-bone chain from its currently evaluated world-space points.
///
/// `pole` is a world-space POINT defining the side toward which the middle bends.
/// A collinear pole falls back to the current projected bend; a straight chain then
/// uses a deterministic perpendicular to the target axis. Every fallback is reported.
/// For equal lengths and a target exactly at the root, the middle points directly
/// toward the pole; a coincident pole preserves the current first-segment direction.
///
/// All five points must be finite, and each current segment must have nonzero length.
/// No absolute minimum length is imposed. Calculations use f64 until returning the
/// engine's f32 vectors/quaternions. Unrepresentable positions and rounded results
/// that would collapse or change a segment length by more than 64 f32 epsilons are
/// rejected explicitly, rather than returning a stretched chain at a bad coordinate
/// scale. The current tip is always recognized as reachable despite norm roundoff.
///
/// There is no history, iteration, allocation, hidden target offset, or stretching.
pub fn solve_two_bone(
    root: Vec3,
    middle: Vec3,
    tip: Vec3,
    target: Vec3,
    pole: Vec3,
    policy: UnreachablePolicy,
) -> Result<TwoBoneIk, String> {
    for (name, point) in [("root", root), ("middle", middle), ("tip", tip), ("target", target), ("pole", pole)] {
        if !finite(point) {
            return Err(format!("two-bone IK {name} must be finite"));
        }
    }
    let a = D3::from(root);
    let first = D3::from(middle) - a;
    let second = D3::from(tip) - D3::from(middle);
    let [l1, l2] = [first.length(), second.length()];
    if l1 == 0.0 || l2 == 0.0 {
        return Err("two-bone IK requires two nonzero current segment lengths".into());
    }
    let sum = l1 + l2;
    let difference = l1 - l2;
    let minimum = difference.abs();
    let requested = D3::from(target) - a;
    let distance = requested.length();
    let current_boundary = distance > 0.0 && first.cross(second).length() == 0.0;
    if target == tip && (pole == middle || current_boundary) {
        // The authored current bend is itself an exact solution. Preserve it without
        // re-solving a numerically ill-conditioned straight/folded triangle. At a
        // nonzero-span collinear reach boundary the middle is unique, so a different
        // pole cannot move it. Keep the measured tiny distal segment instead of losing
        // it by subtracting two huge root-relative vectors.
        let pole_offset = D3::from(pole) - a;
        let pole_fallback = distance > 0.0
            && (pole_offset - requested * (pole_offset.dot(requested) / (distance * distance))).length()
                <= 64.0 * f64::EPSILON * sum.max(pole_offset.length());
        return Ok(TwoBoneIk {
            middle,
            tip,
            root_rotation: Quat::IDENTITY,
            middle_rotation: Quat::IDENTITY,
            requested_target: target,
            effective_target: tip,
            residual: 0.0,
            clamped: false,
            pole_fallback,
            target_direction_fallback: distance == 0.0,
            segment_lengths: [l1, l2],
        });
    }
    // The supplied current tip is a constructive reachability witness. Floating-point
    // norms can otherwise put a perfectly straight current chain one f64 ulp outside.
    let outside = target != tip && (distance < minimum || distance > sum);
    if outside && policy == UnreachablePolicy::Reject {
        return Err(format!("two-bone IK target distance {distance} is outside reach [{minimum}, {sum}]"));
    }
    let effective_distance = distance.clamp(minimum, sum);
    let target_direction_fallback = distance == 0.0;
    let direction = if distance > 0.0 {
        requested * (1.0 / distance)
    } else {
        let current_target = D3::from(tip) - a;
        if current_target.length() > 0.0 {
            current_target.unit()
        } else {
            first * (1.0 / l1)
        }
    };
    let pole_offset = D3::from(pole) - a;
    let (first_solved, pole_fallback) = if effective_distance == 0.0 {
        // Equal-length fold: there is no target axis and hence no distinguished plane.
        // The pole itself provides an unambiguous first-segment direction.
        if pole_offset.length() > 0.0 {
            (pole_offset.unit() * l1, false)
        } else {
            (first, true)
        }
    } else {
        let projected_pole = pole_offset - direction * pole_offset.dot(direction);
        let numeric_guard = 64.0 * f64::EPSILON * sum.max(pole_offset.length());
        let (bend, fallback) = if projected_pole.length() > numeric_guard {
            (projected_pole.unit(), false)
        } else {
            let current_bend = first - direction * first.dot(direction);
            if current_bend.length() > 64.0 * f64::EPSILON * l1 {
                (current_bend.unit(), true)
            } else {
                (direction.perpendicular(), true)
            }
        };
        let short = l1.min(l2);
        let long = l1.max(l2);
        let solved = if short < 0.5 * long {
            // Measuring from the shorter bone avoids losing it in (long +/- short).
            // Here d >= long/2, so both factors remain well conditioned even for a
            // tiny first bone and a very long second bone.
            let along_short = 0.5
                * ((effective_distance - long) * (1.0 + long / effective_distance)
                    + short * (short / effective_distance));
            let along_short = along_short.clamp(-short, short);
            let height = ((short - along_short) * (short + along_short)).max(0.0).sqrt();
            if l1 <= l2 {
                direction * along_short + bend * height
            } else {
                direction * (effective_distance - along_short) + bend * height
            }
        } else {
            // Factored cosine law avoids subtracting two large, nearly equal squares.
            let along = 0.5 * (effective_distance + (difference / effective_distance) * sum);
            // Factored triangle area retains a small bend close to either reach boundary.
            // The ratios are in [0, 2], including arbitrarily small equal-length folds.
            let height_squared = 0.25
                * (sum + effective_distance)
                * (sum - effective_distance)
                * ((effective_distance + difference) / effective_distance)
                * ((effective_distance - difference) / effective_distance);
            direction * along + bend * height_squared.max(0.0).sqrt()
        };
        (solved, fallback)
    };
    let effective_offset = if outside { direction * effective_distance } else { requested };
    let second_solved = effective_offset - first_solved;
    let solved_middle = (a + first_solved).to_vec();
    let solved_tip = if outside { (a + effective_offset).to_vec() } else { target };
    if !finite(solved_middle) || !finite(solved_tip) {
        return Err("two-bone IK solution exceeds finite f32 position range".into());
    }
    for (actual, expected) in
        [((D3::from(solved_middle) - a).length(), l1), ((D3::from(solved_tip) - D3::from(solved_middle)).length(), l2)]
    {
        if actual == 0.0 || (actual - expected).abs() > 64.0 * f64::from(f32::EPSILON) * expected {
            return Err("two-bone IK solution cannot preserve segment lengths at this f32 coordinate scale".into());
        }
    }
    let root_rotation = Q::from_to(first, first_solved);
    let rotated_second = root_rotation.rotate(second);
    let middle_rotation = Q::from_to(rotated_second, second_solved);
    Ok(TwoBoneIk {
        middle: solved_middle,
        tip: solved_tip,
        root_rotation: root_rotation.to_quat(),
        middle_rotation: middle_rotation.to_quat(),
        requested_target: target,
        effective_target: solved_tip,
        residual: (D3::from(solved_tip) - D3::from(target)).length(),
        clamped: outside,
        pole_fallback,
        target_direction_fallback,
        segment_lengths: [l1, l2],
    })
}

fn finite(point: Vec3) -> bool {
    point.x.is_finite() && point.y.is_finite() && point.z.is_finite()
}

#[derive(Clone, Copy, Debug)]
struct D3([f64; 3]);
impl D3 {
    fn length(self) -> f64 {
        self.dot(self).sqrt()
    }
    fn dot(self, other: Self) -> f64 {
        self.0.iter().zip(other.0).map(|(a, b)| a * b).sum()
    }
    fn cross(self, other: Self) -> Self {
        let [a, b, c] = self.0;
        let [x, y, z] = other.0;
        Self([b * z - c * y, c * x - a * z, a * y - b * x])
    }
    fn unit(self) -> Self {
        self * (1.0 / self.length())
    }
    fn perpendicular(self) -> Self {
        // Use the least-aligned Cartesian axis; ties are x, then y, then z.
        // Projection rather than a fixed cross axis remains stable at coordinate poles.
        let index = (0..3).min_by(|&a, &b| self.0[a].abs().total_cmp(&self.0[b].abs())).unwrap();
        let axis = Self(std::array::from_fn(|i| if i == index { 1.0 } else { 0.0 }));
        (axis - self * self.dot(axis)).unit()
    }
    fn to_vec(self) -> Vec3 {
        Vec3::new(self.0[0] as f32, self.0[1] as f32, self.0[2] as f32)
    }
}
impl From<Vec3> for D3 {
    fn from(point: Vec3) -> Self {
        Self([f64::from(point.x), f64::from(point.y), f64::from(point.z)])
    }
}
impl std::ops::Add for D3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(std::array::from_fn(|i| self.0[i] + other.0[i]))
    }
}
impl std::ops::Sub for D3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(std::array::from_fn(|i| self.0[i] - other.0[i]))
    }
}
impl std::ops::Mul<f64> for D3 {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self(self.0.map(|value| value * scalar))
    }
}

#[derive(Clone, Copy, Debug)]
struct Q {
    vector: D3,
    scalar: f64,
}
impl Q {
    fn from_to(from: D3, to: D3) -> Self {
        let a = from.unit();
        let b = to.unit();
        let dot = a.dot(b).clamp(-1.0, 1.0);
        let crossed = a.cross(b);
        let sine = crossed.length();
        if dot < 0.0 && sine <= 64.0 * f64::EPSILON {
            return Self { vector: a.perpendicular(), scalar: 0.0 };
        }
        if sine == 0.0 {
            return Self { vector: D3([0.0; 3]), scalar: 1.0 };
        }
        let half = 0.5 * sine.atan2(dot);
        Self { vector: crossed * (half.sin() / sine), scalar: half.cos() }
    }
    fn rotate(self, point: D3) -> D3 {
        let twice = self.vector.cross(point) * 2.0;
        point + twice * self.scalar + self.vector.cross(twice)
    }
    fn to_quat(self) -> Quat {
        Quat {
            x: self.vector.0[0] as f32,
            y: self.vector.0[1] as f32,
            z: self.vector.0[2] as f32,
            w: self.scalar as f32,
        }
        .normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn antipodal_and_nearly_antipodal_from_to_remain_finite_and_correct() {
        for perturbation in [0.0, 1e-16, 1e-12, 1e-8, 1e-4] {
            let from = D3([1.0, 0.0, 0.0]);
            let to = D3([-1.0, perturbation, 0.0]).unit();
            let q = Q::from_to(from, to);
            assert!((q.rotate(from) - to).length() < 2e-14);
            assert!((q.vector.dot(q.vector) + q.scalar * q.scalar - 1.0).abs() < 1e-14);
        }
    }

    #[test]
    fn parallel_and_tiny_from_to_rotations_are_not_replaced_by_an_angular_dead_zone() {
        for perturbation in [0.0, 1e-20, 1e-12, 1e-6] {
            let from = D3([1.0, 0.0, 0.0]);
            let to = D3([1.0, perturbation, 0.0]).unit();
            let q = Q::from_to(from, to);
            assert!((q.rotate(from) - to).length() < 1e-15);
            if perturbation > 0.0 {
                assert!(q.vector.length() > 0.0);
            }
        }
    }
}
