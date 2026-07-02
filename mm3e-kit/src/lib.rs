//! mm3e-kit — the single primitive kit for the mechanical math 3-D engine.
//!
//! All dumb mechanism, no policy. The eight root atoms
//! (`scan · hash · fold · project · scale · compare · combine · order`) plus the 3-D
//! rendering primitives wired from them. This is the exact 3-D elevation of MMPE's 2-D kit:
//! where the 2-D engine rasterized 2-D signed-distance fields with a `scan`-convert, this one
//! sphere-traces 3-D signed-distance fields. Same atoms, same dependency-free std-only spirit,
//! same self-rolled framebuffer + BMP encoder.
//!
//! Every decision — which primitives are in the scene, how they combine, where the lights and
//! camera are, how many bounces — lives in `mm3e-orchestrator`, never here. If a primitive in
//! this crate grows an `if` that makes a value judgement, that `if` belongs in the orchestrator.

pub mod camera;
pub mod color;
pub mod dual;
pub mod font;
pub mod framebuffer;
pub mod march;
pub mod sdf;
pub mod shade;
pub mod vec;

pub use camera::Camera;
pub use color::{Material, Rgba};
pub use framebuffer::Framebuffer;
pub use march::{Hit, Marcher, Ray};
pub use sdf::Field;
pub use vec::{Mat3, Quat, Transform, Vec3};

/// The eight root atoms — the canonical vocabulary the whole kit specializes from.
/// Each does exactly one thing and makes no decisions. The 3-D code specializes them:
/// `Vec3::dot` *is* `project`, every SDF *is* `compare` (a distance), sphere-tracing *is*
/// `fold` (reduce ray steps to a hit), lighting *is* `combine` (a weighted sum).
pub mod atoms {
    /// `scan` — stream a region into discrete units (here: the pixel grid into ray coordinates).
    pub fn scan(width: u32, height: u32) -> impl Iterator<Item = (u32, u32)> {
        (0..height).flat_map(move |y| (0..width).map(move |x| (x, y)))
    }

    /// `hash` — a unit → a stable integer (FNV-1a). Drives procedural texture / dithering.
    pub fn hash(bytes: &[u8]) -> u64 {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        for &b in bytes {
            h ^= b as u64;
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
        h
    }

    /// `hash` specialized to a lattice cell → a unit float in [0, 1).
    pub fn hash_cell(ix: i32, iy: i32) -> f32 {
        let mut buf = [0u8; 8];
        buf[0..4].copy_from_slice(&ix.to_le_bytes());
        buf[4..8].copy_from_slice(&iy.to_le_bytes());
        (hash(&buf) >> 40) as f32 / (1u64 << 24) as f32
    }

    /// `fold` — reduce a stream to an accumulator.
    pub fn fold<T, A, F: FnMut(A, &T) -> A>(items: &[T], init: A, mut f: F) -> A {
        let mut acc = init;
        for it in items {
            acc = f(acc, it);
        }
        acc
    }

    /// `project` — a vector through a basis: the dot product. (`Vec3::dot` is this, in 3-D.)
    pub fn project(v: &[f32], basis: &[f32]) -> f32 {
        v.iter().zip(basis).map(|(a, b)| a * b).sum()
    }

    /// `scale` — divide by a reference.
    pub fn scale(v: f32, reference: f32) -> f32 {
        v / reference
    }

    /// `compare` — a distance over a pair (Euclidean). Every SDF is this atom, specialized.
    pub fn compare(a: &[f32], b: &[f32]) -> f32 {
        a.iter().zip(b).map(|(x, y)| (x - y) * (x - y)).sum::<f32>().sqrt()
    }

    /// `combine` — a weighted sum of `(weight, signal)` terms. Lighting is this atom.
    pub fn combine(terms: &[(f32, f32)]) -> f32 {
        terms.iter().map(|(w, s)| w * s).sum()
    }

    /// `order` — indices of `items` sorted by `key`, descending.
    pub fn order<T, K: Fn(&T) -> f32>(items: &[T], key: K) -> Vec<usize> {
        let mut idx: Vec<usize> = (0..items.len()).collect();
        idx.sort_by(|&i, &j| key(&items[j]).partial_cmp(&key(&items[i])).unwrap_or(std::cmp::Ordering::Equal));
        idx
    }
}
