//! Per-triangle UV tangent frames for shading-only normal maps. Geometry remains
//! authoritative for visibility, collision and secondary-ray origins.
use crate::Vec3;
type D3 = [f64; 3];
fn d(v: Vec3) -> D3 {
    [f64::from(v.x), f64::from(v.y), f64::from(v.z)]
}
fn sub(a: D3, b: D3) -> D3 {
    std::array::from_fn(|i| a[i] - b[i])
}
fn scale(a: D3, s: f64) -> D3 {
    a.map(|v| v * s)
}
fn dot(a: D3, b: D3) -> f64 {
    (0..3).map(|i| a[i] * b[i]).sum()
}
fn cross(a: D3, b: D3) -> D3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn unit(a: D3) -> Result<D3, String> {
    let n = dot(a, a).sqrt();
    if !n.is_finite() || n == 0.0 {
        return Err("normal-map tangent frame is degenerate".into());
    }
    Ok(scale(a, 1.0 / n))
}
#[derive(Clone, Copy, Debug)]
pub struct TangentFrame {
    pub tangent: Vec3,
    pub bitangent: Vec3,
    pub normal: Vec3,
}
impl TangentFrame {
    /// Positions and geometry normal must share one coordinate frame. UV winding
    /// determines handedness. Back-side frames reverse all three axes relative to
    /// the corresponding front-side frame; no MikkTSpace vertex-normal claim.
    pub fn from_triangle(vertices: [Vec3; 3], uv: [[f64; 2]; 3], geometry_normal: Vec3) -> Result<Self, String> {
        Self::frame(vertices, uv, geometry_normal, None)
    }
    /// Explicit oriented reference normal preserves mirror-domain orientation
    /// when transformed edge winding alone changes sign.
    pub fn from_triangle_with_reference(
        vertices: [Vec3; 3],
        uv: [[f64; 2]; 3],
        geometry_normal: Vec3,
        reference_normal: Vec3,
    ) -> Result<Self, String> {
        Self::frame(vertices, uv, geometry_normal, Some(reference_normal))
    }
    fn frame(
        vertices: [Vec3; 3],
        uv: [[f64; 2]; 3],
        geometry_normal: Vec3,
        reference: Option<Vec3>,
    ) -> Result<Self, String> {
        if vertices.iter().flat_map(|v| d(*v)).any(|v| !v.is_finite()) || uv.iter().flatten().any(|v| !v.is_finite()) {
            return Err("normal-map frame inputs must be finite".into());
        }
        let a = d(vertices[0]);
        let e1 = sub(d(vertices[1]), a);
        let e2 = sub(d(vertices[2]), a);
        let du1 = uv[1][0] - uv[0][0];
        let dv1 = uv[1][1] - uv[0][1];
        let du2 = uv[2][0] - uv[0][0];
        let dv2 = uv[2][1] - uv[0][1];
        let determinant = du1 * dv2 - du2 * dv1;
        if !determinant.is_finite() || determinant == 0.0 {
            return Err("normal maps require nondegenerate UV triangles".into());
        }
        let u = scale(sub(scale(e1, dv2), scale(e2, dv1)), 1.0 / determinant);
        let v = scale(sub(scale(e2, du1), scale(e1, du2)), 1.0 / determinant);
        let source = unit(reference.map(d).unwrap_or_else(|| cross(e1, e2)))?;
        let normal = unit(d(geometry_normal))?;
        let side = if dot(normal, source) < 0.0 { -1.0 } else { 1.0 };
        let hand = if dot(cross(source, u), v) < 0.0 { -1.0 } else { 1.0 };
        let target_hand = hand * side;
        let projected = sub(u, scale(normal, dot(u, normal)));
        let threshold = 64.0 * f64::EPSILON * dot(u, u).sqrt();
        let tangent = if dot(projected, projected).sqrt() > threshold {
            unit(scale(projected, side))?
        } else {
            let bitangent = unit(scale(sub(v, scale(normal, dot(v, normal))), side))?;
            unit(scale(cross(bitangent, normal), target_hand))?
        };
        let bitangent = unit(scale(cross(normal, tangent), target_hand))?;
        let out = |v: D3| Vec3::new(v[0] as f32, v[1] as f32, v[2] as f32);
        Ok(Self { tangent: out(tangent), bitangent: out(bitangent), normal: out(normal) })
    }
    pub fn perturb(self, rgb: [f32; 3], strength: f32, flip_y: bool) -> Result<Vec3, String> {
        if !strength.is_finite()
            || !(0.0..=8.0).contains(&strength)
            || rgb.iter().any(|v| !v.is_finite() || !(0.0..=1.0).contains(v))
        {
            return Err("normal-map samples/strength are invalid".into());
        }
        if strength == 0.0 {
            return Ok(self.normal);
        }
        let x = (f64::from(rgb[0]) * 2.0 - 1.0) * f64::from(strength);
        let y = (f64::from(rgb[1]) * 2.0 - 1.0) * f64::from(strength) * if flip_y { -1.0 } else { 1.0 };
        let z = f64::from(rgb[2]) * 2.0 - 1.0;
        if z < 0.0 {
            return Err("normal-map blue must encode the upper tangent hemisphere".into());
        }
        let local = unit([x, y, z])?;
        let t = d(self.tangent);
        let b = d(self.bitangent);
        let n = d(self.normal);
        let result = unit(std::array::from_fn(|i| t[i] * local[0] + b[i] * local[1] + n[i] * local[2]))?;
        Ok(Vec3::new(result[0] as f32, result[1] as f32, result[2] as f32))
    }
}
