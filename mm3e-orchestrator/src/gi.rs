//! SDF global illumination — a baked irradiance probe volume.
//!
//! Because the entire scene is a single `Fn(Vec3) -> Field`, gathering bounce light is just
//! more marching: from each probe we shoot short rays, march them into the field, and average
//! the one-bounce radiance they return. This is the GI superpower of an SDF engine — Godot must
//! voxelize/distance-field its meshes to do SDFGI; MM3E already *is* a distance field.
//!
//! Each probe stores an **ambient cube**: six axis-aligned irradiance values (±x, ±y, ±z). At
//! shade time the cube is trilinearly interpolated across the eight surrounding probes and
//! combined against the surface normal (the Half-Life-2 ambient-cube weighting). Baking is the
//! `scan` atom over a 3-D probe grid; gathering is `fold` over sample rays; lookup is `combine`.

use mm3e_kit::{atoms, vec::Vec3};

/// A baked irradiance volume covering an axis-aligned region of the scene.
#[derive(Clone)]
pub struct GiVolume {
    min: Vec3,
    size: Vec3,
    dims: (usize, usize, usize),
    /// Per probe: incoming irradiance along +x, -x, +y, -y, +z, -z.
    cubes: Vec<[Vec3; 6]>,
}

pub type GiRaw<'a> = (Vec3, Vec3, (usize, usize, usize), &'a [[Vec3; 6]]);

const AXES: [Vec3; 6] = [
    Vec3 { x: 1.0, y: 0.0, z: 0.0 },
    Vec3 { x: -1.0, y: 0.0, z: 0.0 },
    Vec3 { x: 0.0, y: 1.0, z: 0.0 },
    Vec3 { x: 0.0, y: -1.0, z: 0.0 },
    Vec3 { x: 0.0, y: 0.0, z: 1.0 },
    Vec3 { x: 0.0, y: 0.0, z: -1.0 },
];

/// Build the set of gather directions for one axis: the axis itself plus four 45° tilts, so a
/// probe samples a cone of incoming light rather than a single ray. Weighted by `dot(dir, axis)`.
fn axis_dirs(axis: Vec3, samples: u32) -> Vec<(Vec3, f32)> {
    let up = if axis.y.abs() > 0.9 { Vec3::new(1.0, 0.0, 0.0) } else { Vec3::new(0.0, 1.0, 0.0) };
    let t1 = axis.cross(up).normalize();
    let t2 = axis.cross(t1).normalize();
    let mut dirs = vec![(axis, 1.0f32)];
    if samples > 1 {
        for &t in &[t1, t2, t1.scale(-1.0), t2.scale(-1.0)] {
            let d = (axis.scale(0.72) + t.scale(0.72)).normalize();
            dirs.push((d, d.dot(axis).max(0.0)));
        }
    }
    dirs
}

impl GiVolume {
    /// Bake an irradiance volume over `[min, max]` with `dims` probes per axis. `radiance(origin,
    /// dir)` returns the one-bounce incoming radiance along a ray (marched by the orchestrator).
    pub fn bake(
        min: Vec3,
        max: Vec3,
        dims: (usize, usize, usize),
        samples: u32,
        radiance: &(dyn Fn(Vec3, Vec3) -> Vec3 + Sync),
    ) -> GiVolume {
        // Clamp every axis to at least one probe so allocation and the `nx-1` index math below
        // can never underflow or touch an empty buffer.
        let (nx, ny, nz) = (dims.0.max(1), dims.1.max(1), dims.2.max(1));
        let dims = (nx, ny, nz);
        let size = max - min;
        let cell = Vec3::new(
            size.x / (nx.max(2) - 1) as f32,
            size.y / (ny.max(2) - 1) as f32,
            size.z / (nz.max(2) - 1) as f32,
        );
        let dir_sets: Vec<Vec<(Vec3, f32)>> = AXES.iter().map(|&a| axis_dirs(a, samples)).collect();

        // One probe's ambient cube.
        let probe_cube = |i: usize, j: usize, k: usize| -> [Vec3; 6] {
            let probe = min + Vec3::new(i as f32 * cell.x, j as f32 * cell.y, k as f32 * cell.z);
            let mut cube = [Vec3::ZERO; 6];
            for (a, dirs) in dir_sets.iter().enumerate() {
                let mut acc = Vec3::ZERO;
                let mut wsum = 0.0;
                for &(dir, w) in dirs {
                    acc = acc + radiance(probe, dir).scale(w);
                    wsum += w;
                }
                cube[a] = acc.scale(1.0 / wsum.max(1e-4));
            }
            cube
        };

        // Parallelize the bake across slices along k (the same scoped-threads pattern the
        // renderer uses). The gather closure is shared read-only across threads.
        let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1).max(1);
        let span = nz.div_ceil(threads);
        let slices: Vec<(usize, Vec<[Vec3; 6]>)> = std::thread::scope(|s| {
            let probe_cube = &probe_cube;
            let handles: Vec<_> = (0..threads)
                .map(|ti| {
                    let k0 = ti * span;
                    let k1 = ((ti + 1) * span).min(nz);
                    s.spawn(move || {
                        let mut out = Vec::with_capacity(k1.saturating_sub(k0) * nx * ny);
                        for k in k0..k1 {
                            for j in 0..ny {
                                for i in 0..nx {
                                    out.push(probe_cube(i, j, k));
                                }
                            }
                        }
                        (k0, out)
                    })
                })
                .collect();
            handles.into_iter().map(|hd| hd.join().unwrap()).collect()
        });

        let mut cubes = vec![[Vec3::ZERO; 6]; nx * ny * nz];
        for (k0, slab) in slices {
            if slab.is_empty() {
                continue; // a thread with no slices (threads > nz)
            }
            let base = k0 * ny * nx;
            cubes[base..base + slab.len()].copy_from_slice(&slab);
        }
        GiVolume { min, size, dims, cubes }
    }

    fn cube_at(&self, i: usize, j: usize, k: usize) -> &[Vec3; 6] {
        let (nx, ny, _) = self.dims;
        &self.cubes[(k * ny + j) * nx + i]
    }

    /// Borrow the raw probe layout for GPU upload.
    pub fn raw(&self) -> GiRaw<'_> {
        (self.min, self.size, self.dims, &self.cubes)
    }

    /// Sample the interpolated irradiance arriving at `pos` on a surface with normal `n`.
    pub fn sample(&self, pos: Vec3, n: Vec3) -> Vec3 {
        let (nx, ny, nz) = self.dims;
        // Fractional grid coordinate, clamped into the volume. A degenerate (zero-extent) axis
        // collapses to probe 0 instead of dividing by zero (which would poison shading with NaN).
        let axis = |p: f32, lo: f32, ext: f32, n: usize| {
            if ext.abs() > 1e-6 {
                (p - lo) / ext * (n.max(2) - 1) as f32
            } else {
                0.0
            }
        };
        let g = Vec3::new(
            axis(pos.x, self.min.x, self.size.x, nx),
            axis(pos.y, self.min.y, self.size.y, ny),
            axis(pos.z, self.min.z, self.size.z, nz),
        );
        let clampi = |v: f32, hi: usize| (v.floor().max(0.0) as usize).min(hi.saturating_sub(1));
        let (i0, j0, k0) = (clampi(g.x, nx), clampi(g.y, ny), clampi(g.z, nz));
        let (i1, j1, k1) = ((i0 + 1).min(nx - 1), (j0 + 1).min(ny - 1), (k0 + 1).min(nz - 1));
        let (fx, fy, fz) =
            ((g.x - i0 as f32).clamp(0.0, 1.0), (g.y - j0 as f32).clamp(0.0, 1.0), (g.z - k0 as f32).clamp(0.0, 1.0));

        // Trilinearly interpolate each of the six cube faces.
        let mut face = [Vec3::ZERO; 6];
        for (f, slot) in face.iter_mut().enumerate() {
            let lerp = |a: Vec3, b: Vec3, t: f32| a.mix(b, t);
            let c000 = self.cube_at(i0, j0, k0)[f];
            let c100 = self.cube_at(i1, j0, k0)[f];
            let c010 = self.cube_at(i0, j1, k0)[f];
            let c110 = self.cube_at(i1, j1, k0)[f];
            let c001 = self.cube_at(i0, j0, k1)[f];
            let c101 = self.cube_at(i1, j0, k1)[f];
            let c011 = self.cube_at(i0, j1, k1)[f];
            let c111 = self.cube_at(i1, j1, k1)[f];
            let x00 = lerp(c000, c100, fx);
            let x10 = lerp(c010, c110, fx);
            let x01 = lerp(c001, c101, fx);
            let x11 = lerp(c011, c111, fx);
            let y0 = lerp(x00, x10, fy);
            let y1 = lerp(x01, x11, fy);
            *slot = lerp(y0, y1, fz);
        }

        // Ambient-cube combine: pick the face on the normal's side per axis, weight by n².
        let nn = Vec3::new(n.x * n.x, n.y * n.y, n.z * n.z);
        let fx_face = if n.x >= 0.0 { face[0] } else { face[1] };
        let fy_face = if n.y >= 0.0 { face[2] } else { face[3] };
        let fz_face = if n.z >= 0.0 { face[4] } else { face[5] };
        fx_face.scale(nn.x) + fy_face.scale(nn.y) + fz_face.scale(nn.z)
    }
}

/// Compute conservative world-space bounds enclosing a set of object bounding spheres, padded.
/// Returns `None` if there is nothing finite to bound (so GI is simply skipped).
pub fn bounds_of(spheres: &[(Vec3, f32)], pad: f32) -> Option<(Vec3, Vec3)> {
    if spheres.is_empty() {
        return None;
    }
    let mut lo = Vec3::splat(f32::INFINITY);
    let mut hi = Vec3::splat(f32::NEG_INFINITY);
    atoms::fold(spheres, (), |_, &(c, r)| {
        lo = lo.min(c - Vec3::splat(r));
        hi = hi.max(c + Vec3::splat(r));
    });
    Some((lo - Vec3::splat(pad), hi + Vec3::splat(pad)))
}
