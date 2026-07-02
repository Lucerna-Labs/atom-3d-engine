//! A sampled signed-distance volume — the mechanism that lets *baked* geometry (triangle meshes
//! re-expressed as distances, see `mm3e_orchestrator::mesh`) flow through the same sphere tracer,
//! CSG, GI, and physics as the analytic primitives. This is the doctrine's bridge move: triangles
//! are the one representation the engine deliberately does not march, so content is converted
//! *into the engine's basis* (a distance field on a grid) once, offline, and from then on it IS
//! a `Field` like any other.
//!
//! Pure mechanism: a grid of distances plus trilinear sampling. What gets baked, at what
//! resolution, from which mesh — all policy, all in the orchestrator.

use crate::vec::Vec3;

/// A regular grid of signed distances over an axis-aligned local-space box.
///
/// Samples hold the *true* signed distance at grid points (the bake computes them exactly);
/// between points the trilinear interpolant deviates from the true field by O(cell²·curvature),
/// which the marcher's unconditional overlap guard absorbs (an over-step retreats rather than
/// tunnels — see `Marcher::march_with`).
#[derive(Clone, Debug)]
pub struct SdfVolume {
    /// Grid resolution per axis (each ≥ 2).
    pub dims: (usize, usize, usize),
    /// Local-space position of grid point (0, 0, 0).
    pub min: Vec3,
    /// Spacing between grid points per axis (> 0).
    pub cell: Vec3,
    /// Distances, row-major: index `(k·ny + j)·nx + i`.
    pub data: Vec<f32>,
}

impl SdfVolume {
    /// Distance at grid point `(i, j, k)` (caller keeps indices in range).
    #[inline]
    pub fn at(&self, i: usize, j: usize, k: usize) -> f32 {
        let (nx, ny, _) = self.dims;
        self.data[(k * ny + j) * nx + i]
    }

    /// The local-space box the grid covers.
    pub fn bounds(&self) -> (Vec3, Vec3) {
        let (nx, ny, nz) = self.dims;
        let size =
            Vec3::new((nx - 1) as f32 * self.cell.x, (ny - 1) as f32 * self.cell.y, (nz - 1) as f32 * self.cell.z);
        (self.min, self.min + size)
    }

    /// Signed distance at local point `p`.
    ///
    /// Inside the grid box: the trilinear interpolation of the eight surrounding samples.
    /// Outside: a **provably conservative** underestimate, `max(box_dist, d(c) − box_dist)`
    /// where `c` is `p` clamped to the box — the true distance satisfies both
    /// `d(p) ≥ box_dist` (the surface lies in the box; projection onto a convex set minimizes
    /// distance) and `d(p) ≥ d(c) − |p−c|` (the field is 1-Lipschitz), so the max of the two
    /// never overshoots and the sphere tracer stays safe without help from the guard.
    pub fn sample(&self, p: Vec3) -> f32 {
        let (lo, hi) = self.bounds();
        let c = p.clamp_to(lo, hi);
        let outside = (p - c).length();

        // Fractional grid coordinates of the clamped point.
        let g = Vec3::new(
            (c.x - self.min.x) / self.cell.x,
            (c.y - self.min.y) / self.cell.y,
            (c.z - self.min.z) / self.cell.z,
        );
        let (nx, ny, nz) = self.dims;
        let clampi = |v: f32, n: usize| (v.floor().max(0.0) as usize).min(n - 2);
        let (i0, j0, k0) = (clampi(g.x, nx), clampi(g.y, ny), clampi(g.z, nz));
        let (fx, fy, fz) =
            ((g.x - i0 as f32).clamp(0.0, 1.0), (g.y - j0 as f32).clamp(0.0, 1.0), (g.z - k0 as f32).clamp(0.0, 1.0));

        let lerp = |a: f32, b: f32, t: f32| a + (b - a) * t;
        let x00 = lerp(self.at(i0, j0, k0), self.at(i0 + 1, j0, k0), fx);
        let x10 = lerp(self.at(i0, j0 + 1, k0), self.at(i0 + 1, j0 + 1, k0), fx);
        let x01 = lerp(self.at(i0, j0, k0 + 1), self.at(i0 + 1, j0, k0 + 1), fx);
        let x11 = lerp(self.at(i0, j0 + 1, k0 + 1), self.at(i0 + 1, j0 + 1, k0 + 1), fx);
        let d = lerp(lerp(x00, x10, fy), lerp(x01, x11, fy), fz);

        if outside > 0.0 {
            outside.max(d - outside)
        } else {
            d
        }
    }
}
