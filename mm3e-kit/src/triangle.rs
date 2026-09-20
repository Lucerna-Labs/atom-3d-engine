//! Triangle crossing mechanism for sampled-field construction.
use crate::vec::Vec3;

fn edge(a: Vec3, b: Vec3, y: f64, z: f64) -> f64 {
    (f64::from(b.y) - f64::from(a.y)) * (z - f64::from(a.z)) - (f64::from(b.z) - f64::from(a.z)) * (y - f64::from(a.y))
}

fn owns_boundary(a: Vec3, b: Vec3) -> bool {
    b.z > a.z || (b.z == a.z && b.y < a.y)
}

/// X coordinate where the infinite X-directed line at `(y,z)` crosses a triangle.
///
/// Project onto YZ, normalize winding, and apply a half-open edge rule. Adjacent projected
/// triangles sharing an edge own that edge once, avoiding double parity crossings on face
/// diagonals. Computation uses f64 on f32 inputs; this is not an exact-predicate mesh validator.
/// Degenerate projections (including triangles parallel to the line) have no crossing.
pub fn x_crossing(a: Vec3, b: Vec3, c: Vec3, y: f32, z: f32) -> Option<f32> {
    let area = edge(a, b, f64::from(c.y), f64::from(c.z));
    if area == 0.0 || !area.is_finite() || !y.is_finite() || !z.is_finite() {
        return None;
    }
    let (b, c, area) = if area < 0.0 { (c, b, -area) } else { (b, c, area) };
    let y = f64::from(y);
    let z = f64::from(z);
    let weights = [edge(b, c, y, z), edge(c, a, y, z), edge(a, b, y, z)];
    for (weight, (start, end)) in weights.iter().zip([(b, c), (c, a), (a, b)]) {
        if *weight < 0.0 || (*weight == 0.0 && !owns_boundary(start, end)) {
            return None;
        }
    }
    let x = ((weights[0] * f64::from(a.x) + weights[1] * f64::from(b.x) + weights[2] * f64::from(c.x)) / area) as f32;
    x.is_finite().then_some(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_shared_diagonal_is_counted_once_independent_of_winding() {
        let a = Vec3::new(2.0, -1.0, -1.0);
        let b = Vec3::new(2.0, 1.0, -1.0);
        let c = Vec3::new(2.0, 1.0, 1.0);
        let d = Vec3::new(2.0, -1.0, 1.0);
        for triangles in [[[a, b, c], [a, c, d]], [[c, b, a], [d, c, a]], [[a, c, b], [a, c, d]]] {
            for y in [-0.7, 0.0, 0.4, 0.75] {
                let hits: Vec<_> = triangles.iter().filter_map(|t| x_crossing(t[0], t[1], t[2], y, y)).collect();
                assert_eq!(hits, vec![2.0]);
            }
        }
    }

    #[test]
    fn projected_vertices_and_thin_faces_have_consistent_ownership() {
        let o = Vec3::new(1.0, 0.0, 0.0);
        let p = [
            Vec3::new(1.0, -1.0, -1.0),
            Vec3::new(1.0, 1.0, -1.0),
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(1.0, -1.0, 1.0),
        ];
        let hits = (0..4).filter_map(|i| x_crossing(o, p[i], p[(i + 1) % 4], 0.0, 0.0)).count();
        assert_eq!(hits, 1);
        // The former absolute determinant threshold discarded small, valid projections.
        assert_eq!(x_crossing(o, Vec3::new(1.0, 1e-7, 0.0), Vec3::new(1.0, 0.0, 1e-7), 2e-8, 2e-8), Some(1.0));
    }
}
