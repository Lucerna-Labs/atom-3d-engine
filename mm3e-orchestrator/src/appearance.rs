//! Object-owned surface appearance, independent of shared material IDs and physics.
use crate::{surface_albedo, Combine, Object, Prim, Scene};
use mm3e_kit::{
    color::Material,
    march::Hit,
    sdf::{self, Field},
    tangent::TangentFrame,
    texture::{CornerUvs, Sampler, TextureImage},
    Vec3,
};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

#[derive(Clone, Debug)]
pub struct SurfaceTexture {
    pub uv: CornerUvs,
    pub maps: SurfaceMaps,
    vertex_count: usize,
    triangles: Arc<Vec<[u32; 3]>>,
}
#[derive(Clone, Copy, Debug)]
pub enum Channel {
    R,
    G,
    B,
    A,
}
impl Channel {
    fn index(self) -> usize {
        match self {
            Self::R => 0,
            Self::G => 1,
            Self::B => 2,
            Self::A => 3,
        }
    }
}
#[derive(Clone, Debug)]
pub struct TextureMap {
    pub image: TextureImage,
    pub sampler: Sampler,
}
#[derive(Clone, Debug)]
pub struct ScalarMap {
    pub map: TextureMap,
    pub channel: Channel,
}
#[derive(Clone, Debug)]
pub struct NormalMap {
    pub map: TextureMap,
    pub strength: f32,
    pub flip_y: bool,
    /// Approximate unresolved slope variation with isotropic GGX roughness.
    pub variance_filter: bool,
}
#[derive(Clone, Debug, Default)]
pub struct SurfaceMaps {
    pub albedo: Option<TextureMap>,
    pub roughness: Option<ScalarMap>,
    pub metallic: Option<ScalarMap>,
    pub emissive: Option<TextureMap>,
    pub normal: Option<NormalMap>,
}
impl SurfaceMaps {
    fn validate(&self) -> Result<(), String> {
        let maps = [
            self.albedo.as_ref(),
            self.roughness.as_ref().map(|m| &m.map),
            self.metallic.as_ref().map(|m| &m.map),
            self.emissive.as_ref(),
            self.normal.as_ref().map(|m| &m.map),
        ];
        if maps.iter().all(|m| m.is_none()) {
            return Err("surface material binding needs at least one map".into());
        }
        for (i, map) in maps.into_iter().enumerate() {
            if let Some(map) = map {
                let data = matches!(i, 1 | 2 | 4);
                if map.image.is_data() != data {
                    return Err("material-map image interpretation does not match color/data role".into());
                }
                if !map.sampler.lod_bias.is_finite() || !(-16.0..=16.0).contains(&map.sampler.lod_bias) {
                    return Err("material-map LOD bias must be in [-16,16]".into());
                }
            }
        }
        if let Some(normal) = &self.normal {
            if !normal.strength.is_finite() || !(0.0..=8.0).contains(&normal.strength) {
                return Err("normal strength must be finite in [0,8]".into());
            }
            if normal.map.image.channel_bounds()[2][0] < 0.5 {
                return Err("normal-map blue must encode the upper tangent hemisphere".into());
            }
        }
        Ok(())
    }
}
#[derive(Clone, Copy)]
pub(crate) struct ShadingSurface {
    pub material: Material,
    pub normal: Vec3,
    pub normal_mapped: bool,
}
#[derive(Debug, Default)]
pub struct Appearance {
    bindings: BTreeMap<usize, SurfaceTexture>,
    failure: Mutex<Option<String>>,
}
impl Clone for Appearance {
    fn clone(&self) -> Self {
        Self { bindings: self.bindings.clone(), failure: Mutex::new(None) }
    }
}
impl Appearance {
    pub fn remove_binding(&mut self, object: usize) {
        self.bindings.remove(&object);
    }
    /// Reindex appearance alongside an object selection, preserving exact binding
    /// ownership rather than deriving it from shared material identifiers.
    pub fn retain_objects(&mut self, original_indices: &[usize]) -> Result<(), String> {
        let mut seen = std::collections::BTreeSet::new();
        let mut bindings = BTreeMap::new();
        for (new, &old) in original_indices.iter().enumerate() {
            if !seen.insert(old) {
                return Err("appearance object selection contains duplicate indices".into());
            }
            if let Some(binding) = self.bindings.get(&old) {
                bindings.insert(new, binding.clone());
            }
        }
        self.bindings = bindings;
        self.failure = Mutex::new(None);
        Ok(())
    }
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }
    pub fn clear(&mut self) {
        *self = Self::default();
    }
    pub fn status(&self) -> Result<(), String> {
        match self.failure.lock() {
            Ok(failure) => failure.as_ref().map_or(Ok(()), |s| Err(s.clone())),
            Err(_) => Err("appearance evaluation state is poisoned".into()),
        }
    }
    fn fail(&self, message: String) {
        if let Ok(mut error) = self.failure.lock() {
            if error.is_none() {
                *error = Some(message);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MaterialSample {
    pub field: Field,
    /// Source of the existing MATERIAL policy; subtraction retains its base owner.
    pub object: Option<usize>,
    pub triangle: Option<u32>,
    pub uv: Option<[f64; 2]>,
    pub lod: Option<f64>,
    pub footprint_lod: Option<f64>,
    pub premultiplied_rgba: Option<[f32; 4]>,
    pub material: Material,
    pub geometric_normal: Option<Vec3>,
    pub shading_normal: Option<Vec3>,
    pub map_lods: [Option<f64>; 5],
    /// Roughness after scalar maps, before normal-variance compensation.
    pub unfiltered_roughness: f32,
    /// Applied slope variance, including normal strength squared. Zero when disabled.
    pub normal_variance: f64,
}

impl Scene {
    pub fn surface_texture(&self, object: usize) -> Option<&SurfaceTexture> {
        self.appearance.bindings.get(&object)
    }

    /// Shared finite query-domain mapping for coordinate transfer. Callers charge
    /// its constant transform/modifier work separately from bounded BVH queries.
    pub fn surface_texture_local_point(&self, object: usize, point: Vec3) -> Result<Vec3, String> {
        let object = self.objects.get(object).ok_or("missing coordinate-transfer object")?;
        let local = object.local_point(point);
        if [local.x, local.y, local.z].iter().any(|value| !value.is_finite()) {
            return Err("coordinate-transfer point is not finite".into());
        }
        Ok(local)
    }
    pub fn bake_gi_checked(&mut self, dims: (usize, usize, usize), samples: u32) -> Result<(), String> {
        self.validate_appearance()?;
        let mut candidate = self.clone();
        candidate.bake_gi_unchecked(dims, samples);
        candidate.appearance.status()?;
        self.gi = candidate.gi;
        Ok(())
    }
    pub fn bind_surface_texture(
        &mut self,
        object: usize,
        uv: CornerUvs,
        image: TextureImage,
        sampler: Sampler,
    ) -> Result<(), String> {
        self.bind_surface_maps(
            object,
            uv,
            SurfaceMaps { albedo: Some(TextureMap { image, sampler }), ..SurfaceMaps::default() },
        )
    }
    pub fn bind_surface_maps(&mut self, object: usize, uv: CornerUvs, maps: SurfaceMaps) -> Result<(), String> {
        maps.validate()?;
        let source = self.objects.get(object).ok_or("texture references missing scene object")?;
        supported(source)?;
        let Prim::Surface { id } = source.prim else {
            return Err("UV textures require a native triangle surface".into());
        };
        let surface = self.surfaces.get(id as usize).ok_or("texture references missing native surface")?;
        if uv.triangle_count() != surface.triangles().len() {
            return Err("UV triangle count differs from bound surface".into());
        }
        if maps.normal.is_some() {
            if source.mods.elongate != Vec3::ZERO {
                return Err("normal maps do not yet support elongate domain mapping".into());
            }
            for i in 0..surface.triangles().len() {
                let c = uv.corners(i as u32).ok_or("missing normal UV triangle")?;
                let determinant = (c[1][0] - c[0][0]) * (c[2][1] - c[0][1]) - (c[2][0] - c[0][0]) * (c[1][1] - c[0][1]);
                if determinant == 0.0 || !determinant.is_finite() {
                    return Err("normal maps require nondegenerate UV triangles".into());
                }
            }
        }
        self.appearance.bindings.insert(
            object,
            SurfaceTexture {
                uv,
                maps,
                vertex_count: surface.vertices().len(),
                triangles: Arc::new(surface.triangles().to_vec()),
            },
        );
        self.gi = None;
        self.appearance.failure = Mutex::new(None);
        Ok(())
    }
    pub fn validate_appearance(&self) -> Result<(), String> {
        if self.materials.is_empty() {
            return Err("scene needs at least one material".into());
        }
        for (&index, binding) in &self.appearance.bindings {
            let object = self.objects.get(index).ok_or("bound appearance object no longer exists")?;
            supported(object)?;
            if binding.maps.normal.is_some() && object.mods.elongate != Vec3::ZERO {
                return Err("normal maps do not yet support elongate domain mapping".into());
            }
            let Prim::Surface { id } = object.prim else {
                return Err("textured object is no longer a native surface".into());
            };
            let surface = self.surfaces.get(id as usize).ok_or("bound appearance surface no longer exists")?;
            if surface.vertices().len() != binding.vertex_count || surface.triangles() != binding.triangles.as_slice() {
                return Err("surface topology changed without updating its UV binding".into());
            }
        }
        self.appearance.status()
    }
    /// Exact authored fold with MATERIAL-owner provenance; no material-index inference.
    pub fn sample_authored_owner(&self, p: Vec3) -> (Field, Option<usize>) {
        let mut accumulated: Option<(Field, usize)> = None;
        for (index, object) in self.objects.iter().enumerate() {
            let next = object.field(p, &self.volumes, &self.csgs, &self.surfaces);
            accumulated = Some(match accumulated {
                None => (next, index),
                Some((previous, owner)) => match object.combine {
                    Combine::Subtract => (sdf::subtract(previous, next), owner),
                    Combine::Union => {
                        (sdf::union(previous, next), if previous.dist <= next.dist { owner } else { index })
                    }
                    Combine::Smooth(k) => {
                        let keep = if k <= 0.0 {
                            previous.dist <= next.dist
                        } else {
                            (0.5 + 0.5 * (next.dist - previous.dist) / k).clamp(0.0, 1.0) > 0.5
                        };
                        (sdf::smooth_union(previous, next, k), if keep { owner } else { index })
                    }
                },
            });
        }
        accumulated.map_or((Field::FAR, None), |(field, owner)| (field, Some(owner)))
    }
    /// A checked observation. Footprint is a nonnegative world-space projected width.
    pub fn sample_material(&self, point: Vec3, footprint: f64) -> Result<MaterialSample, String> {
        self.validate_appearance()?;
        self.material_sample_unchecked(point, footprint, None)
    }
    fn material_sample_unchecked(
        &self,
        point: Vec3,
        footprint: f64,
        geometry_normal: Option<Vec3>,
    ) -> Result<MaterialSample, String> {
        if !point.x.is_finite()
            || !point.y.is_finite()
            || !point.z.is_finite()
            || !footprint.is_finite()
            || footprint < 0.0
        {
            return Err("appearance point and footprint must be finite, with nonnegative footprint".into());
        }
        let (field, owner) = self.sample_authored_owner(point);
        let mut material =
            *self.materials.get(field.mat as usize % self.materials.len()).ok_or("missing appearance material")?;
        material.albedo = surface_albedo(&material, point);
        let mut sample = MaterialSample {
            field,
            object: owner,
            triangle: None,
            uv: None,
            lod: None,
            footprint_lod: None,
            premultiplied_rgba: None,
            material,
            geometric_normal: None,
            shading_normal: None,
            map_lods: [None; 5],
            unfiltered_roughness: material.roughness,
            normal_variance: 0.0,
        };
        let Some(index) = owner else {
            return Ok(sample);
        };
        let Some(binding) = self.appearance.bindings.get(&index) else {
            return Ok(sample);
        };
        let object = &self.objects[index];
        let Prim::Surface { id } = object.prim else {
            return Err("textured object is no longer a native surface".into());
        };
        let surface = self.surfaces.get(id as usize).ok_or("missing texture surface")?;
        let hit = surface.closest_hit_bounded(object.local_point(point), 1_000_000)?;
        let uv = binding.uv.interpolate(hit.triangle, hit.barycentric)?;
        let corners = binding.uv.corners(hit.triangle).ok_or("missing texture UV triangle")?;
        let triangle = surface.triangles()[hit.triangle as usize].map(|i| surface.vertices()[i as usize]);
        sample.triangle = Some(hit.triangle);
        sample.uv = Some(uv);
        let evaluate = |map: &TextureMap| -> Result<([f32; 4], f64, f64), String> {
            let lod =
                texture_lod(triangle, corners, map.image.dimensions(), footprint / f64::from(object.xform.scale))?;
            Ok((map.image.sample(uv, lod, map.sampler)?, map.image.effective_lod(lod, map.sampler)?, lod))
        };
        if let Some(map) = &binding.maps.albedo {
            let (rgba, lod, requested) = evaluate(map)?;
            let factor = Vec3::new(rgba[0] + 1.0 - rgba[3], rgba[1] + 1.0 - rgba[3], rgba[2] + 1.0 - rgba[3]).clamp01();
            sample.material.albedo = sample.material.albedo.cmul(factor);
            sample.lod = Some(lod);
            sample.footprint_lod = Some(requested);
            sample.premultiplied_rgba = Some(rgba);
            sample.map_lods[0] = Some(lod);
        }
        if let Some(map) = &binding.maps.roughness {
            let (rgba, lod, _) = evaluate(&map.map)?;
            sample.material.roughness = (sample.material.roughness * rgba[map.channel.index()]).clamp(0.04, 1.0);
            sample.map_lods[1] = Some(lod);
        }
        if let Some(map) = &binding.maps.metallic {
            let (rgba, lod, _) = evaluate(&map.map)?;
            sample.material.metallic = (sample.material.metallic * rgba[map.channel.index()]).clamp(0.0, 1.0);
            sample.map_lods[2] = Some(lod);
        }
        if let Some(map) = &binding.maps.emissive {
            let (rgba, lod, _) = evaluate(map)?;
            sample.material.emissive = sample.material.emissive.cmul(Vec3::new(rgba[0], rgba[1], rgba[2]));
            sample.map_lods[3] = Some(lod);
        }
        sample.unfiltered_roughness = sample.material.roughness;
        if let Some(map) = &binding.maps.normal {
            let (rgba, lod, requested) = evaluate(&map.map)?;
            sample.map_lods[4] = Some(lod);
            if map.variance_filter && map.strength > 0.0 {
                sample.normal_variance =
                    map.map.image.normal_variance(uv, requested, map.map.sampler)? * f64::from(map.strength).powi(2);
                if sample.normal_variance > 0.0 {
                    // Slope moments motivate this bounded isotropic approximation;
                    // it is not an exact convolution of the GGX distribution.
                    let roughness = f64::from(sample.material.roughness);
                    sample.material.roughness =
                        (roughness.powi(4) + sample.normal_variance).sqrt().sqrt().min(1.0).max(roughness) as f32;
                }
            }
            let raw = object.xform.to_local(point);
            let sign = Vec3::new(
                if object.mods.mirror[0] && raw.x < 0.0 { -1.0 } else { 1.0 },
                if object.mods.mirror[1] && raw.y < 0.0 { -1.0 } else { 1.0 },
                if object.mods.mirror[2] && raw.z < 0.0 { -1.0 } else { 1.0 },
            );
            let transform = |v: Vec3| object.xform.rot.mul_vec(v.cmul(sign));
            let source_normal = transform(triangle_normal(triangle)?);
            let geometric = geometry_normal.unwrap_or_else(|| {
                let normal = self.marcher.normal(&self.field(), point);
                if normal.length() > 0.0 {
                    normal
                } else {
                    transform(hit.normal)
                }
            });
            let edges = [
                Vec3::ZERO,
                transform(triangle[1] - triangle[0]) * object.xform.scale,
                transform(triangle[2] - triangle[0]) * object.xform.scale,
            ];
            let frame = TangentFrame::from_triangle_with_reference(edges, corners, geometric, source_normal)?;
            sample.geometric_normal = Some(geometric);
            sample.shading_normal = Some(if map.strength == 0.0 {
                geometric
            } else {
                frame.perturb([rgba[0], rgba[1], rgba[2]], map.strength, map.flip_y)?
            });
        }
        Ok(sample)
    }
    /// Internal legacy-render bridge. Checked render entry points reject recorded
    /// evaluation failures before exposing an image; no failed result is deliverable.
    pub(crate) fn material_for_hit(&self, hit: &Hit, footprint: f64) -> ShadingSurface {
        let mut base = self.materials[hit.mat as usize % self.materials.len()];
        if self.appearance.is_empty() {
            base.albedo = surface_albedo(&base, hit.pos);
            return ShadingSurface { material: base, normal: hit.normal, normal_mapped: false };
        }
        match self.material_sample_unchecked(hit.pos, footprint, Some(hit.normal)) {
            Ok(sample) => ShadingSurface {
                material: sample.material,
                normal: sample.shading_normal.unwrap_or(hit.normal),
                normal_mapped: sample.shading_normal.is_some_and(|n| n != hit.normal),
            },
            Err(error) => {
                self.appearance.fail(format!(
                    "appearance at {:?} (source object {:?}) failed: {error}",
                    hit.pos,
                    self.sample_authored_owner(hit.pos).1
                ));
                base.albedo = Vec3::ZERO;
                ShadingSurface { material: base, normal: hit.normal, normal_mapped: false }
            }
        }
    }
}
fn triangle_normal(vertices: [Vec3; 3]) -> Result<Vec3, String> {
    let d = |p: Vec3| [f64::from(p.x), f64::from(p.y), f64::from(p.z)];
    let a = d(vertices[0]);
    let b = d(vertices[1]);
    let c = d(vertices[2]);
    let u: [f64; 3] = std::array::from_fn(|i| b[i] - a[i]);
    let v: [f64; 3] = std::array::from_fn(|i| c[i] - a[i]);
    let n = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
    let length = n.iter().map(|v| v * v).sum::<f64>().sqrt();
    if !length.is_finite() || length == 0.0 {
        return Err("normal-map source triangle is degenerate".into());
    }
    Ok(Vec3::new((n[0] / length) as f32, (n[1] / length) as f32, (n[2] / length) as f32))
}
fn supported(object: &Object) -> Result<(), String> {
    let finite = |v: Vec3| v.x.is_finite() && v.y.is_finite() && v.z.is_finite();
    if !finite(object.xform.pos) || object.xform.rot.cols.iter().any(|&v| !finite(v)) {
        return Err("textured object transform must be finite".into());
    }
    for i in 0..3 {
        for j in 0..3 {
            let expected = if i == j { 1.0 } else { 0.0 };
            if (object.xform.rot.cols[i].dot(object.xform.rot.cols[j]) - expected).abs() > 0.001 {
                return Err("textured object placement must be orthogonal with uniform scale".into());
            }
        }
    }
    if !object.xform.scale.is_finite() || object.xform.scale <= 0.0 {
        return Err("textured object scale must be finite and positive".into());
    }
    if object.mods.twist != 0.0 || object.mods.bend != 0.0 {
        return Err("UV texture filtering does not yet support twist or bend domain warps".into());
    }
    Ok(())
}
fn texture_lod(vertices: [Vec3; 3], uv: [[f64; 2]; 3], size: [u32; 2], width: f64) -> Result<f64, String> {
    let d = |v: Vec3| [f64::from(v.x), f64::from(v.y), f64::from(v.z)];
    let sub = |a: [f64; 3], b: [f64; 3]| std::array::from_fn::<_, 3, _>(|i| a[i] - b[i]);
    let cross =
        |a: [f64; 3], b: [f64; 3]| [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
    let norm = |a: [f64; 3]| a.iter().map(|v| v * v).sum::<f64>().sqrt();
    let a = d(vertices[0]);
    let b = sub(d(vertices[1]), a);
    let c = sub(d(vertices[2]), a);
    let n = cross(b, c);
    let square = n.iter().map(|v| v * v).sum::<f64>();
    if square == 0.0 || !square.is_finite() {
        return Err("UV footprint has degenerate geometry".into());
    }
    let gb = cross(c, n).map(|v| v / square);
    let gc = cross(n, b).map(|v| v / square);
    let gradient = |axis: usize| {
        std::array::from_fn(|i| gb[i] * (uv[1][axis] - uv[0][axis]) + gc[i] * (uv[2][axis] - uv[0][axis]))
    };
    let density = (norm(gradient(0)) * f64::from(size[0])).max(norm(gradient(1)) * f64::from(size[1]));
    let rho = width * density;
    if !rho.is_finite() {
        return Err("UV footprint exceeds supported finite precision".into());
    }
    Ok(rho.max(1.0).log2())
}
