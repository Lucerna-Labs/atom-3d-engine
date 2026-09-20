//! Stateless deformation of a fixed, vertex-indexed mesh in rest-world coordinates.
//!
//! Blend-shape position deltas are added first, then the supplied rest-world joint
//! deltas skin those positions. The caller retains the original triangle indices;
//! this operation never remeshes, reorders, or edits its inputs. It implements linear
//! blend skinning and rigid dual-quaternion skinning, not anatomical tissue simulation.

use crate::{surface::MAX_SURFACE_VERTICES, Mat3, Transform, Vec3};

pub const MAX_DEFORM_VERTICES: usize = MAX_SURFACE_VERTICES;
pub const MAX_DEFORM_JOINTS: usize = 256;
pub const MAX_VERTEX_INFLUENCES: usize = 8;
pub const MAX_BLEND_SHAPES: usize = 64;
/// Only f32 authoring roundoff within this absolute tolerance is normalized. Invalid
/// weight sums, zero/negative weights and duplicate joint indices are rejected.
pub const WEIGHT_SUM_TOLERANCE: f64 = 1e-6;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SkinningMethod {
    LinearBlend,
    /// Rigid skinning: every supplied joint must have exactly unit uniform scale.
    /// Nonunit scale is an error; there is no implicit change to linear skinning.
    DualQuaternion,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Influence {
    /// Index into the supplied joint-delta palette, not a hierarchy parent index.
    pub joint: u32,
    pub weight: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlendShape {
    /// Dense rest-world position deltas in exactly the rest mesh's vertex order.
    pub deltas: Vec<Vec3>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Deformation {
    pub positions: Vec<Vec3>,
    /// Bounds of the returned vertices, without a surface shell's thickness.
    pub bounds: (Vec3, Vec3),
    /// Maximum distance from a returned vertex to its corresponding rest vertex.
    /// f64 keeps this diagnostic finite across the entire finite f32 position range.
    pub max_displacement: f64,
}

/// Evaluate a mesh pose from immutable source data, with f64 arithmetic internally.
///
/// Joint transforms map rest-world positions directly to posed-world positions. Do
/// not pass joint local transforms or absolute posed transforms without their inverse
/// bind transform. Vertices and morphs must already include the object's rest placement;
/// transform local morph deltas by its rest rotation and scale, without translation.
///
/// There must be 3..=65,536 vertices, 1..=256 joints, 1..=8 positive influences per
/// vertex summing to one (within `WEIGHT_SUM_TOLERANCE`), and at most 64 blend shapes.
/// For morph-only evaluation, pass empty joint and influence slices together.
/// Morph weights can be negative and need not sum to one, but must be finite. Every
/// morph, including inactive ones, must have the exact source vertex cardinality and
/// finite deltas. Rotations must be finite, orthonormal and proper within f32 roundoff.
/// Linear skinning accepts positive finite uniform scales. DQS rejects nonunit scales.
///
/// Unmoved vertices retain their original f32 bits, including signed zero. A failure
/// returns no partial result and cannot mutate the source. Triangle validity and any
/// pose-dependent collapsed faces remain the responsibility of the surface consumer.
pub fn deform(
    rest_positions: &[Vec3],
    influences: &[Vec<Influence>],
    blend_shapes: &[BlendShape],
    blend_weights: &[f32],
    joint_deltas: &[Transform],
    method: SkinningMethod,
) -> Result<Deformation, String> {
    if !(3..=MAX_DEFORM_VERTICES).contains(&rest_positions.len()) {
        return Err(format!("deformation requires 3..={MAX_DEFORM_VERTICES} rest vertices"));
    }
    let morph_only = influences.is_empty() && joint_deltas.is_empty();
    if !morph_only && influences.len() != rest_positions.len() {
        return Err("deformation influence rows must match rest vertex count and order".into());
    }
    if !morph_only && !(1..=MAX_DEFORM_JOINTS).contains(&joint_deltas.len()) {
        return Err(format!("deformation requires 1..={MAX_DEFORM_JOINTS} joint deltas"));
    }
    if blend_shapes.len() > MAX_BLEND_SHAPES || blend_shapes.len() != blend_weights.len() {
        return Err(format!("deformation requires matching blend shapes/weights, at most {MAX_BLEND_SHAPES}"));
    }
    for (index, point) in rest_positions.iter().enumerate() {
        if !finite(*point) {
            return Err(format!("deformation rest vertex {index} must be finite"));
        }
    }
    for (index, (shape, weight)) in blend_shapes.iter().zip(blend_weights).enumerate() {
        if shape.deltas.len() != rest_positions.len() {
            return Err(format!("blend shape {index} must match rest vertex count and order"));
        }
        if !weight.is_finite() || shape.deltas.iter().any(|point| !finite(*point)) {
            return Err(format!("blend shape {index} deltas and weight must be finite"));
        }
    }
    for (index, transform) in joint_deltas.iter().enumerate() {
        validate_transform(*transform, method).map_err(|error| format!("deformation joint {index}: {error}"))?;
    }
    let mut sums = Vec::with_capacity(influences.len());
    for (vertex, row) in influences.iter().enumerate() {
        if !(1..=MAX_VERTEX_INFLUENCES).contains(&row.len()) {
            return Err(format!("vertex {vertex} requires 1..={MAX_VERTEX_INFLUENCES} influences"));
        }
        let mut sum = 0.0;
        for (index, influence) in row.iter().enumerate() {
            if influence.joint as usize >= joint_deltas.len() {
                return Err(format!("vertex {vertex} influence has an out-of-range joint"));
            }
            if !influence.weight.is_finite() || influence.weight <= 0.0 {
                return Err(format!("vertex {vertex} influence weight must be finite and positive"));
            }
            if row[..index].iter().any(|previous| previous.joint == influence.joint) {
                return Err(format!("vertex {vertex} has a duplicate influence joint"));
            }
            sum += f64::from(influence.weight);
        }
        if (sum - 1.0).abs() > WEIGHT_SUM_TOLERANCE {
            return Err(format!("vertex {vertex} influence weights must sum to one (received {sum})"));
        }
        sums.push(sum);
    }
    let duals: Vec<_> = if method == SkinningMethod::DualQuaternion {
        joint_deltas.iter().copied().map(DualQuat::from_transform).collect()
    } else {
        Vec::new()
    };
    let mut positions = Vec::with_capacity(rest_positions.len());
    let mut lo = Vec3::splat(f32::INFINITY);
    let mut hi = Vec3::splat(f32::NEG_INFINITY);
    let mut max_displacement: f64 = 0.0;
    for (index, &rest) in rest_positions.iter().enumerate() {
        let mut delta = D3::ZERO;
        for (shape, &weight) in blend_shapes.iter().zip(blend_weights) {
            if weight != 0.0 {
                delta = delta + D3::from(shape.deltas[index]) * f64::from(weight);
            }
        }
        let point = D3::from(rest) + delta;
        let row = if morph_only { &[][..] } else { influences[index].as_slice() };
        let unmoved = row.iter().all(|influence| identity(joint_deltas[influence.joint as usize]));
        let output = if unmoved && delta == D3::ZERO {
            rest
        } else if unmoved {
            point.to_vec()
        } else if row.len() == 1 {
            // Both methods are exactly the same matrix operation for one influence.
            transform_point(joint_deltas[row[0].joint as usize], point).to_vec()
        } else {
            match method {
                SkinningMethod::LinearBlend => {
                    let mut posed = D3::ZERO;
                    for influence in row {
                        let weight = f64::from(influence.weight) / sums[index];
                        posed = posed + transform_point(joint_deltas[influence.joint as usize], point) * weight;
                    }
                    posed.to_vec()
                }
                SkinningMethod::DualQuaternion => {
                    blend_dual_quaternions(row, sums[index], &duals)?.transform_point(point).to_vec()
                }
            }
        };
        if !finite(output) {
            return Err(format!("deformed vertex {index} exceeds finite f32 position range"));
        }
        max_displacement = max_displacement.max((D3::from(output) - D3::from(rest)).length());
        lo = lo.min(output);
        hi = hi.max(output);
        positions.push(output);
    }
    Ok(Deformation { positions, bounds: (lo, hi), max_displacement })
}

fn finite(point: Vec3) -> bool {
    point.x.is_finite() && point.y.is_finite() && point.z.is_finite()
}

fn identity(transform: Transform) -> bool {
    transform.pos == Vec3::ZERO && transform.scale == 1.0 && transform.rot.cols == Mat3::IDENTITY.cols
}

fn validate_transform(transform: Transform, method: SkinningMethod) -> Result<(), String> {
    if !finite(transform.pos) || !transform.scale.is_finite() || transform.scale <= 0.0 {
        return Err("translation must be finite and uniform scale finite and positive".into());
    }
    if method == SkinningMethod::DualQuaternion && transform.scale != 1.0 {
        return Err("dual-quaternion skinning requires exactly unit scale on every joint".into());
    }
    if transform.rot.cols.iter().any(|column| !finite(*column)) {
        return Err("rotation must be finite".into());
    }
    let cols = transform.rot.cols.map(D3::from);
    for i in 0..3 {
        for j in i..3 {
            let target = if i == j { 1.0 } else { 0.0 };
            if (cols[i].dot(cols[j]) - target).abs() > 2e-5 {
                return Err("rotation must be orthonormal (no shear or matrix scale)".into());
            }
        }
    }
    if (cols[0].cross(cols[1]).dot(cols[2]) - 1.0).abs() > 3e-5 {
        return Err("rotation must be proper (no reflection)".into());
    }
    Ok(())
}

fn transform_point(transform: Transform, point: D3) -> D3 {
    let cols = transform.rot.cols.map(D3::from);
    let scaled = point * f64::from(transform.scale);
    D3::from(transform.pos) + cols[0] * scaled.0[0] + cols[1] * scaled.0[1] + cols[2] * scaled.0[2]
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct D3([f64; 3]);
impl D3 {
    const ZERO: Self = Self([0.0; 3]);
    fn to_vec(self) -> Vec3 {
        Vec3::new(self.0[0] as f32, self.0[1] as f32, self.0[2] as f32)
    }
    fn dot(self, other: Self) -> f64 {
        self.0.iter().zip(other.0).map(|(a, b)| a * b).sum()
    }
    fn cross(self, other: Self) -> Self {
        let [a, b, c] = self.0;
        let [x, y, z] = other.0;
        Self([b * z - c * y, c * x - a * z, a * y - b * x])
    }
    fn length(self) -> f64 {
        self.dot(self).sqrt()
    }
}
impl From<Vec3> for D3 {
    fn from(value: Vec3) -> Self {
        Self([f64::from(value.x), f64::from(value.y), f64::from(value.z)])
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
    fn mul(self, scale: f64) -> Self {
        Self(self.0.map(|component| component * scale))
    }
}

/// Unnormalized Hamilton arithmetic is essential here: the engine's public quaternion
/// product normalizes its inputs, which would destroy a dual part's translation length.
#[derive(Clone, Copy, Debug)]
struct Q([f64; 4]);
impl Q {
    const ZERO: Self = Self([0.0; 4]);
    fn dot(self, other: Self) -> f64 {
        self.0.iter().zip(other.0).map(|(a, b)| a * b).sum()
    }
    fn conjugate(self) -> Self {
        let [x, y, z, w] = self.0;
        Self([-x, -y, -z, w])
    }
    fn canonical_sign(self) -> f64 {
        // At an exact half-turn from the reference, dot=0 cannot select a hemisphere.
        // Break that tie by a fixed quaternion component order, independent of q/-q.
        for index in [3, 0, 1, 2] {
            if self.0[index] != 0.0 {
                return if self.0[index] < 0.0 { -1.0 } else { 1.0 };
            }
        }
        1.0
    }
    fn from_rotation(rotation: Mat3) -> Self {
        let [a, b, c] = rotation.cols.map(D3::from).map(|column| column.0);
        let trace = a[0] + b[1] + c[2];
        let q = if trace > 0.0 {
            let s = (trace + 1.0).sqrt() * 2.0;
            Self([(b[2] - c[1]) / s, (c[0] - a[2]) / s, (a[1] - b[0]) / s, 0.25 * s])
        } else if a[0] > b[1] && a[0] > c[2] {
            let s = (1.0 + a[0] - b[1] - c[2]).sqrt() * 2.0;
            Self([0.25 * s, (b[0] + a[1]) / s, (c[0] + a[2]) / s, (b[2] - c[1]) / s])
        } else if b[1] > c[2] {
            let s = (1.0 + b[1] - a[0] - c[2]).sqrt() * 2.0;
            Self([(b[0] + a[1]) / s, 0.25 * s, (c[1] + b[2]) / s, (c[0] - a[2]) / s])
        } else {
            let s = (1.0 + c[2] - a[0] - b[1]).sqrt() * 2.0;
            Self([(c[0] + a[2]) / s, (c[1] + b[2]) / s, 0.25 * s, (a[1] - b[0]) / s])
        };
        q * (1.0 / q.dot(q).sqrt())
    }
}
impl std::ops::Add for Q {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(std::array::from_fn(|i| self.0[i] + other.0[i]))
    }
}
impl std::ops::Mul<f64> for Q {
    type Output = Self;
    fn mul(self, scale: f64) -> Self {
        Self(self.0.map(|component| component * scale))
    }
}
impl std::ops::Mul for Q {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let [x, y, z, w] = self.0;
        let [a, b, c, d] = other.0;
        Self([
            w * a + x * d + y * c - z * b,
            w * b - x * c + y * d + z * a,
            w * c + x * b - y * a + z * d,
            w * d - x * a - y * b - z * c,
        ])
    }
}

#[derive(Clone, Copy, Debug)]
struct DualQuat {
    real: Q,
    dual: Q,
}
impl DualQuat {
    fn from_transform(transform: Transform) -> Self {
        let real = Q::from_rotation(transform.rot);
        let [x, y, z] = D3::from(transform.pos).0;
        Self { real, dual: (Q([x, y, z, 0.0]) * real) * 0.5 }
    }
    fn transform_point(self, point: D3) -> D3 {
        let [x, y, z] = point.0;
        let rotated = self.real * Q([x, y, z, 0.0]) * self.real.conjugate();
        let translation = (self.dual * self.real.conjugate()) * 2.0;
        D3(std::array::from_fn(|i| rotated.0[i] + translation.0[i]))
    }
}

fn blend_dual_quaternions(row: &[Influence], sum: f64, palette: &[DualQuat]) -> Result<DualQuat, String> {
    // Largest weight is the hemisphere reference; equal weights resolve by joint index.
    let reference = row.iter().max_by(|a, b| a.weight.total_cmp(&b.weight).then(b.joint.cmp(&a.joint))).unwrap();
    let reference = palette[reference.joint as usize].real;
    let mut real = Q::ZERO;
    let mut dual = Q::ZERO;
    for influence in row {
        let joint = palette[influence.joint as usize];
        let dot = reference.dot(joint.real);
        let sign = if dot == 0.0 {
            reference.canonical_sign() * joint.real.canonical_sign()
        } else if dot < 0.0 {
            -1.0
        } else {
            1.0
        };
        let weight = sign * f64::from(influence.weight) / sum;
        real = real + joint.real * weight;
        dual = dual + joint.dual * weight;
    }
    let norm = real.dot(real).sqrt();
    if !norm.is_finite() || norm <= 1e-12 {
        return Err("dual-quaternion blend has an undefined rotation".into());
    }
    real = real * (1.0 / norm);
    dual = dual * (1.0 / norm);
    // Unit dual quaternion: real.dot(real)=1 and real.dot(dual)=0 (Study condition).
    dual = dual + real * -real.dot(dual);
    Ok(DualQuat { real, dual })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dual_quaternion_hemisphere_alignment_is_invariant_to_sign() {
        let transform = Transform::new(Vec3::new(9.0, -4.0, 2.0), Mat3::from_euler(0.4, 1.7, -2.5), 1.0);
        let q = DualQuat::from_transform(transform);
        let negative = DualQuat { real: q.real * -1.0, dual: q.dual * -1.0 };
        let row = [Influence { joint: 0, weight: 0.5 }, Influence { joint: 1, weight: 0.5 }];
        let expected = blend_dual_quaternions(&row, 1.0, &[q, q]).unwrap();
        let actual = blend_dual_quaternions(&row, 1.0, &[q, negative]).unwrap();
        let flipped = blend_dual_quaternions(&row, 1.0, &[negative, q]).unwrap();
        for point in [D3([2.0, 3.0, 4.0]), D3::ZERO] {
            assert!((expected.transform_point(point) - actual.transform_point(point)).length() < 1e-12);
            assert!((expected.transform_point(point) - flipped.transform_point(point)).length() < 1e-12);
            assert!((actual.transform_point(point) - transform_point(transform, point)).length() < 2e-6);
        }
    }

    #[test]
    fn exact_half_turn_hemisphere_tie_is_invariant_to_quaternion_sign() {
        let a = DualQuat { real: Q([0.0, 0.0, 0.0, 1.0]), dual: Q([1.0, 2.0, 3.0, 0.0]) };
        let b = DualQuat { real: Q([1.0, 0.0, 0.0, 0.0]), dual: Q([0.0, 2.0, -3.0, 4.0]) };
        let row = [Influence { joint: 0, weight: 0.5 }, Influence { joint: 1, weight: 0.5 }];
        let point = D3([2.0, 1.0, 0.0]);
        let expected = blend_dual_quaternions(&row, 1.0, &[a, b]).unwrap().transform_point(point);
        for sign_a in [-1.0, 1.0] {
            for sign_b in [-1.0, 1.0] {
                let palette = [
                    DualQuat { real: a.real * sign_a, dual: a.dual * sign_a },
                    DualQuat { real: b.real * sign_b, dual: b.dual * sign_b },
                ];
                let actual = blend_dual_quaternions(&row, 1.0, &palette).unwrap().transform_point(point);
                assert!((expected - actual).length() < 1e-12);
            }
        }
    }
}
