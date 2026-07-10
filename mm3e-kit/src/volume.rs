//! Sampled signed-distance volumes.
//!
//! This is the mechanism that lets baked triangle meshes enter the SDF renderer without turning
//! the engine into a triangle rasterizer. Policy stays in `mm3e-orchestrator::mesh`: this module
//! only owns a regular grid and conservative trilinear sampling.

use crate::vec::Vec3;

const SDFV_MAGIC: &[u8; 8] = b"MM3ESDF1";
const SDFV_HEADER_LEN: usize = 8 + 3 * 4 + 3 * 4 + 3 * 4;

/// A regular grid of signed distances over an axis-aligned local-space box.
#[derive(Clone, Debug)]
pub struct SdfVolume {
    /// Grid resolution per axis. Valid volumes have each dimension >= 2.
    pub dims: (usize, usize, usize),
    /// Local-space position of grid point (0, 0, 0).
    pub min: Vec3,
    /// Spacing between grid points per axis. Valid volumes have positive finite cell sizes.
    pub cell: Vec3,
    /// Distances, row-major: index `(k * ny + j) * nx + i`.
    pub data: Vec<f32>,
}

impl SdfVolume {
    /// Build a volume after validating layout and finite data.
    pub fn new(dims: (usize, usize, usize), min: Vec3, cell: Vec3, data: Vec<f32>) -> Result<SdfVolume, String> {
        let v = SdfVolume { dims, min, cell, data };
        v.validate()?;
        Ok(v)
    }

    /// Validate the complete volume, including every sample value.
    pub fn validate(&self) -> Result<(), String> {
        self.validate_layout()?;
        if let Some((i, _)) = self.data.iter().enumerate().find(|(_, d)| !d.is_finite()) {
            return Err(format!("SdfVolume sample {i} is non-finite"));
        }
        Ok(())
    }

    fn expected_len(&self) -> Option<usize> {
        let (nx, ny, nz) = self.dims;
        nx.checked_mul(ny)?.checked_mul(nz)
    }

    fn has_valid_layout(&self) -> bool {
        let (nx, ny, nz) = self.dims;
        nx >= 2
            && ny >= 2
            && nz >= 2
            && self.min.x.is_finite()
            && self.min.y.is_finite()
            && self.min.z.is_finite()
            && self.cell.x.is_finite()
            && self.cell.y.is_finite()
            && self.cell.z.is_finite()
            && self.cell.x > 0.0
            && self.cell.y > 0.0
            && self.cell.z > 0.0
            && self.expected_len() == Some(self.data.len())
    }
    fn validate_layout(&self) -> Result<(), String> {
        let (nx, ny, nz) = self.dims;
        if nx < 2 || ny < 2 || nz < 2 {
            return Err("SdfVolume dims must all be >= 2".into());
        }
        if !(self.min.x.is_finite() && self.min.y.is_finite() && self.min.z.is_finite()) {
            return Err("SdfVolume min must be finite".into());
        }
        if !(self.cell.x.is_finite() && self.cell.y.is_finite() && self.cell.z.is_finite()) {
            return Err("SdfVolume cell sizes must be positive and finite".into());
        }
        if !(self.cell.x > 0.0 && self.cell.y > 0.0 && self.cell.z > 0.0) {
            return Err("SdfVolume cell sizes must be positive and finite".into());
        }
        let Some(nxy) = nx.checked_mul(ny) else {
            return Err("SdfVolume dimensions overflow".into());
        };
        let Some(expected) = nxy.checked_mul(nz) else {
            return Err("SdfVolume dimensions overflow".into());
        };
        if self.data.len() != expected {
            return Err(format!("SdfVolume data length {} does not match dims {nx}x{ny}x{nz}", self.data.len()));
        }
        Ok(())
    }

    /// Distance at grid point `(i, j, k)` (caller keeps indices in range).
    #[inline]
    pub fn at(&self, i: usize, j: usize, k: usize) -> f32 {
        let (nx, ny, _) = self.dims;
        self.data[(k * ny + j) * nx + i]
    }

    /// The local-space box the grid covers.
    pub fn bounds(&self) -> (Vec3, Vec3) {
        let (nx, ny, nz) = self.dims;
        let sx = nx.saturating_sub(1) as f32 * self.cell.x;
        let sy = ny.saturating_sub(1) as f32 * self.cell.y;
        let sz = nz.saturating_sub(1) as f32 * self.cell.z;
        (self.min, self.min + Vec3::new(sx, sy, sz))
    }

    /// The largest cell spacing.
    pub fn max_cell(&self) -> f32 {
        self.cell.x.max(self.cell.y).max(self.cell.z)
    }

    /// Recommended tetrahedron-normal half-width for this sampled field.
    pub fn recommended_normal_h(&self) -> f32 {
        self.max_cell() * 0.6
    }

    /// Encode this volume to MM3E's tiny std-only `.sdfv` cache format.
    ///
    /// Format: `MM3ESDF1`, `u32 nx,ny,nz`, `f32 min xyz`, `f32 cell xyz`, then little-endian
    /// `f32` samples in the same row-major order as [`SdfVolume::data`].
    pub fn to_sdfv_bytes(&self) -> Result<Vec<u8>, String> {
        self.validate()?;
        let (nx, ny, nz) = self.dims;
        if nx > u32::MAX as usize || ny > u32::MAX as usize || nz > u32::MAX as usize {
            return Err("SdfVolume dimensions exceed sdfv u32 limits".into());
        }
        let mut out = Vec::with_capacity(SDFV_HEADER_LEN + self.data.len() * 4);
        out.extend_from_slice(SDFV_MAGIC);
        for v in [nx as u32, ny as u32, nz as u32] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        for v in [self.min.x, self.min.y, self.min.z, self.cell.x, self.cell.y, self.cell.z] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        for &d in &self.data {
            out.extend_from_slice(&d.to_le_bytes());
        }
        Ok(out)
    }

    /// Decode an `.sdfv` byte buffer produced by [`SdfVolume::to_sdfv_bytes`].
    pub fn from_sdfv_bytes(bytes: &[u8]) -> Result<SdfVolume, String> {
        if bytes.len() < SDFV_HEADER_LEN {
            return Err("sdfv buffer is shorter than the header".into());
        }
        if &bytes[..8] != SDFV_MAGIC {
            return Err("sdfv magic/version mismatch".into());
        }
        let mut off = 8usize;
        let nx = read_u32(bytes, &mut off, "nx")? as usize;
        let ny = read_u32(bytes, &mut off, "ny")? as usize;
        let nz = read_u32(bytes, &mut off, "nz")? as usize;
        let min = Vec3::new(
            read_f32(bytes, &mut off, "min.x")?,
            read_f32(bytes, &mut off, "min.y")?,
            read_f32(bytes, &mut off, "min.z")?,
        );
        let cell = Vec3::new(
            read_f32(bytes, &mut off, "cell.x")?,
            read_f32(bytes, &mut off, "cell.y")?,
            read_f32(bytes, &mut off, "cell.z")?,
        );
        let Some(samples) = nx.checked_mul(ny).and_then(|nxy| nxy.checked_mul(nz)) else {
            return Err("sdfv dimensions overflow".into());
        };
        let Some(payload_bytes) = samples.checked_mul(4) else {
            return Err("sdfv payload length overflows".into());
        };
        let expected = SDFV_HEADER_LEN + payload_bytes;
        if bytes.len() != expected {
            return Err(format!("sdfv byte length {} does not match expected {expected}", bytes.len()));
        }
        let mut data = Vec::with_capacity(samples);
        for i in 0..samples {
            data.push(read_f32(bytes, &mut off, &format!("sample {i}"))?);
        }
        SdfVolume::new((nx, ny, nz), min, cell, data)
    }

    /// Write this volume to disk as `.sdfv`.
    pub fn save_sdfv(&self, path: &std::path::Path) -> Result<(), String> {
        std::fs::write(path, self.to_sdfv_bytes()?).map_err(|e| format!("{}: {e}", path.display()))
    }

    /// Load a volume from disk as `.sdfv`.
    pub fn load_sdfv(path: &std::path::Path) -> Result<SdfVolume, String> {
        let bytes = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
        SdfVolume::from_sdfv_bytes(&bytes).map_err(|e| format!("{}: {e}", path.display()))
    }
    /// Signed distance at local point `p`.
    ///
    /// Inside the grid box this is the trilinear interpolation of the eight surrounding samples.
    /// Outside, it returns a conservative positive underestimate:
    /// `max(box_dist, d(clamped) - box_dist)`. Since the surface lives inside the box and a true
    /// distance field is 1-Lipschitz, this cannot overstep a surface.
    pub fn sample(&self, p: Vec3) -> f32 {
        if !self.has_valid_layout() {
            return f32::INFINITY;
        }

        let (lo, hi) = self.bounds();
        let c = p.clamp_to(lo, hi);
        let outside = (p - c).length();
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
        if !d.is_finite() {
            return f32::INFINITY;
        }

        if outside > 0.0 {
            outside.max(d - outside)
        } else {
            d
        }
    }
}
fn read_u32(bytes: &[u8], off: &mut usize, what: &str) -> Result<u32, String> {
    if bytes.len().saturating_sub(*off) < 4 {
        return Err(format!("sdfv truncated while reading {what}"));
    }
    let mut raw = [0u8; 4];
    raw.copy_from_slice(&bytes[*off..*off + 4]);
    *off += 4;
    Ok(u32::from_le_bytes(raw))
}

fn read_f32(bytes: &[u8], off: &mut usize, what: &str) -> Result<f32, String> {
    let bits = read_u32(bytes, off, what)?;
    let v = f32::from_bits(bits);
    if v.is_finite() {
        Ok(v)
    } else {
        Err(format!("sdfv non-finite f32 while reading {what}"))
    }
}
