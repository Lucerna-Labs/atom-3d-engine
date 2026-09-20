//! Two-sided displacement bound for a frozen triangulation and its simplicial image.
//!
//! Callers must supply certified enclosures of original coordinates after exact
//! coalescing/retriangulation. Every original triangle must map to a final
//! oriented triangle, edge or vertex, and every final triangle must have an
//! original predecessor. Barycentric correspondence then bounds both surface
//! directions by maximum direct original-to-final vertex displacement. This does
//! not certify topology, embedded face orientation, intersections or native/UV error.
use crate::{
    meshing::{MAX_MESH_TRIANGLES, MAX_MESH_VERTICES},
    Vec3,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug)]
pub struct Options {
    pub max_error_m: f64,
    pub max_work: usize,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CorrespondenceReport {
    pub work: usize,
    pub vertex_witnesses: usize,
    pub original_triangles: usize,
    pub final_triangles: usize,
    pub triangle_images: usize,
    pub edge_images: usize,
    pub vertex_images: usize,
    pub max_displacement_m: f64,
    pub worst_vertex: usize,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CorrespondenceFailure {
    /// Work already spent before rejection; retrying callers must charge it.
    pub work: usize,
    pub message: String,
}
struct Budget {
    maximum: usize,
    used: usize,
}
impl Budget {
    fn charge(&mut self, amount: usize) -> Result<(), String> {
        if amount > self.maximum.saturating_sub(self.used) {
            return Err("mesh correspondence exhausted work budget".into());
        }
        self.used += amount;
        Ok(())
    }
}
fn oriented_key([a, b, c]: [u32; 3]) -> [u32; 3] {
    if a <= b && a <= c {
        [a, b, c]
    } else if b <= c {
        [b, c, a]
    } else {
        [c, a, b]
    }
}
fn outward_norm(components: [f64; 3]) -> Result<f64, String> {
    let maximum = components.into_iter().fold(0.0_f64, f64::max);
    if maximum == 0. {
        return Ok(0.);
    }
    if !maximum.is_finite() {
        return Err("mesh correspondence displacement exceeds finite bounds".into());
    }
    // Scale before squaring to avoid a tiny nonzero displacement becoming zero.
    // Every nonnegative operation rounds outward, including underflow. Exact
    // zero axes contribute zero instead of supplying borrowed error allowance.
    let mut sum = 0.0_f64;
    for component in components {
        if component != 0. {
            let scaled = (component / maximum).next_up();
            let squared = (scaled * scaled).next_up();
            sum = (sum + squared).next_up();
        }
    }
    // sqrt has a correctly rounded result (unlike unspecified-precision
    // transcendental functions): https://doc.rust-lang.org/std/primitive.f64.html#method.sqrt
    let result = (maximum * sum.sqrt().next_up()).next_up();
    if result.is_finite() {
        Ok(result)
    } else {
        Err("mesh correspondence displacement exceeds finite bounds".into())
    }
}
/// Enclosures are `[lower_xyz,upper_xyz]`. True aliases may be appended as extra
/// witnesses, but each must map to a vertex used by the final surface. Exclude
/// interior partition points already removed by exact coalescing before freezing
/// this input. Triangle images require cyclic orientation; edge/vertex images do
/// not. Work counts bounded storage scans, set operations and coordinate bounds.
/// Use `certify_counted` if rejection can be followed by another candidate attempt.
pub fn certify(
    original_enclosures: &[[[f64; 3]; 2]],
    original_triangles: &[[u32; 3]],
    original_to_final: &[u32],
    final_positions: &[Vec3],
    final_triangles: &[[u32; 3]],
    options: Options,
) -> Result<CorrespondenceReport, String> {
    certify_counted(
        original_enclosures,
        original_triangles,
        original_to_final,
        final_positions,
        final_triangles,
        options,
    )
    .map_err(|failure| failure.message)
}

/// Same certificate, retaining spent work on rejection for bounded searches.
pub fn certify_counted(
    original_enclosures: &[[[f64; 3]; 2]],
    original_triangles: &[[u32; 3]],
    original_to_final: &[u32],
    final_positions: &[Vec3],
    final_triangles: &[[u32; 3]],
    options: Options,
) -> Result<CorrespondenceReport, CorrespondenceFailure> {
    let mut budget = Budget { maximum: options.max_work, used: 0 };
    certify_inner(
        original_enclosures,
        original_triangles,
        original_to_final,
        final_positions,
        final_triangles,
        options,
        &mut budget,
    )
    .map_err(|message| CorrespondenceFailure { work: budget.used, message })
}

fn certify_inner(
    original_enclosures: &[[[f64; 3]; 2]],
    original_triangles: &[[u32; 3]],
    original_to_final: &[u32],
    final_positions: &[Vec3],
    final_triangles: &[[u32; 3]],
    options: Options,
    budget: &mut Budget,
) -> Result<CorrespondenceReport, String> {
    if original_enclosures.is_empty()
        || original_enclosures.len() > MAX_MESH_VERTICES
        || original_triangles.is_empty()
        || original_triangles.len() > MAX_MESH_TRIANGLES
        || original_to_final.len() != original_enclosures.len()
        || final_positions.is_empty()
        || final_positions.len() > MAX_MESH_VERTICES
        || final_triangles.is_empty()
        || final_triangles.len() > MAX_MESH_TRIANGLES
    {
        return Err("mesh correspondence requires bounded nonempty geometry and complete vertex ancestry".into());
    }
    if !options.max_error_m.is_finite() || options.max_error_m < 0. {
        return Err("mesh correspondence requires a finite nonnegative displacement limit".into());
    }
    budget
        .charge(original_enclosures.len() + original_triangles.len() + final_positions.len() + final_triangles.len())?;
    if final_positions.iter().any(|p| [p.x, p.y, p.z].into_iter().any(|v| !v.is_finite())) {
        return Err("mesh correspondence final positions must be finite".into());
    }
    let mut vertices = vec![false; final_positions.len()];
    let mut edges = BTreeSet::new();
    let mut faces = BTreeMap::new();
    for &triangle in final_triangles {
        budget.charge(7)?;
        if triangle.iter().any(|&i| i as usize >= final_positions.len()) {
            return Err("mesh correspondence final triangle index is invalid".into());
        }
        let [a, b, c] = triangle;
        if a == b || b == c || c == a {
            return Err("mesh correspondence final triangle repeats a vertex index".into());
        }
        if faces.insert(oriented_key(triangle), false).is_some() {
            return Err("mesh correspondence final geometry contains a duplicate oriented face".into());
        }
        for (a, b) in [(a, b), (b, c), (c, a)] {
            vertices[a as usize] = true;
            edges.insert((a.min(b), a.max(b)));
        }
    }
    let mut report = CorrespondenceReport {
        work: 0,
        vertex_witnesses: original_enclosures.len(),
        original_triangles: original_triangles.len(),
        final_triangles: final_triangles.len(),
        triangle_images: 0,
        edge_images: 0,
        vertex_images: 0,
        max_displacement_m: 0.,
        worst_vertex: 0,
    };
    for (index, (&enclosure, &destination)) in original_enclosures.iter().zip(original_to_final).enumerate() {
        budget.charge(24)?;
        if destination as usize >= vertices.len() || !vertices[destination as usize] {
            return Err(format!("mesh correspondence original vertex {index} has no image in the final surface"));
        }
        if (0..3).any(|axis| {
            !enclosure[0][axis].is_finite()
                || !enclosure[1][axis].is_finite()
                || enclosure[0][axis] > enclosure[1][axis]
        }) {
            return Err("mesh correspondence requires finite ordered coordinate enclosures".into());
        }
        let p = final_positions[destination as usize];
        let p = [p.x, p.y, p.z].map(f64::from);
        let components = std::array::from_fn(|axis| {
            let error = (enclosure[0][axis] - p[axis]).abs().max((enclosure[1][axis] - p[axis]).abs());
            if error == 0. {
                0.
            } else {
                error.next_up()
            }
        });
        let displacement = outward_norm(components)?;
        if displacement > report.max_displacement_m {
            report.max_displacement_m = displacement;
            report.worst_vertex = index;
        }
    }
    for &triangle in original_triangles {
        budget.charge(5)?;
        if triangle.iter().any(|&i| i as usize >= original_to_final.len()) {
            return Err("mesh correspondence original triangle index is invalid".into());
        }
        let [a, b, c] = triangle;
        if a == b || b == c || c == a {
            return Err("mesh correspondence original triangle repeats a vertex index".into());
        }
        let image = triangle.map(|i| original_to_final[i as usize]);
        let [a, b, c] = image;
        if a == b && b == c {
            report.vertex_images += 1;
        } else if a == b || b == c || c == a {
            let lo = a.min(b).min(c);
            let hi = a.max(b).max(c);
            if !edges.contains(&(lo, hi)) {
                return Err("mesh correspondence original face image is not a final edge".into());
            }
            report.edge_images += 1;
        } else {
            let covered = faces
                .get_mut(&oriented_key(image))
                .ok_or("mesh correspondence original face image is not a final oriented face")?;
            *covered = true;
            report.triangle_images += 1;
        }
    }
    budget.charge(faces.len())?;
    if faces.values().any(|&covered| !covered) {
        return Err("mesh correspondence final face has no original predecessor".into());
    }
    if report.max_displacement_m > options.max_error_m {
        return Err(format!(
            "mesh correspondence direct source-to-stored displacement {}m exceeds {}m at original vertex {}",
            report.max_displacement_m, options.max_error_m, report.worst_vertex
        ));
    }
    report.work = budget.used;
    Ok(report)
}
