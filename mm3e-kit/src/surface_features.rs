//! Local convex-feature representations for extracting a triangle surface.
//! A triangle prism plus its three edge capsules has exactly the spherical
//! offset's inside set. Optional capsule or common-cube supporting planes are
//! initial enclosures only. The min/max scalar is NOT a distance field.
use crate::{
    meshing_boolean::BooleanExpr,
    meshing_local::{LocalCellData, LocalFeature},
    surface::TriangleSurface,
    Vec3,
};
use std::collections::{BTreeMap, BTreeSet};

type D3 = [f64; 3];
#[derive(Clone)]
enum Feature {
    Plane { normal: D3, origin: D3, base: f64, offset: f64 },
    BoxPlane { normal: D3, origin: D3, base: f64, offset: f64 },
    Capsule { a: D3, b: D3, radius: f64 },
}
impl Feature {
    fn value(&self, p: D3) -> f64 {
        match self {
            Self::Plane { normal, origin, base, offset } => (dot(*normal, sub(p, *origin)) - *base) - *offset,
            Self::BoxPlane { normal, origin, base, offset } => {
                if *offset >= 0. {
                    let lower = -dot_upper(scale(*normal, -1.), p, *origin);
                    -add_upper(add_upper(-lower, *base), *offset)
                } else {
                    add_upper(add_upper(dot_upper(*normal, p, *origin), -*base), -*offset)
                }
            }
            Self::Capsule { a, b, radius } => {
                let ab = sub(*b, *a);
                let t = (dot(sub(p, *a), ab) / dot(ab, ab)).clamp(0., 1.);
                norm(sub(p, add(*a, scale(ab, t)))) - *radius
            }
        }
    }
    fn cost(&self) -> usize {
        if matches!(self, Self::Plane { .. }) {
            1
        } else {
            8
        }
    }
}
#[derive(Clone)]
enum TriangleFeatures {
    Rounded { planes: [(u64, bool); 5], capsules: [u64; 3] },
    Box(Vec<(u64, bool)>),
}
impl TriangleFeatures {
    fn planes(&self) -> &[(u64, bool)] {
        match self {
            Self::Rounded { planes, .. } => planes,
            Self::Box(planes) => planes,
        }
    }
    fn capsules(&self) -> &[u64] {
        match self {
            Self::Rounded { capsules, .. } => capsules,
            Self::Box(_) => &[],
        }
    }
}
#[derive(Clone)]
pub struct SurfaceFeatures {
    surface: TriangleSurface,
    radius: f64,
    features: Vec<Feature>,
    triangles: Vec<TriangleFeatures>,
    capsule_enclosures: BTreeMap<u64, Vec<(u64, bool)>>,
    pub construction_work: usize,
}
pub struct FeatureProgram {
    pub ids: Vec<u64>,
    pub expression: BooleanExpr,
    pub work: usize,
}
pub struct FeatureValues {
    pub values: Vec<f32>,
    pub work: usize,
}
const INSIDE_ID: u64 = u64::MAX - 1;
struct Budget {
    used: usize,
    max: usize,
}
impl Budget {
    fn charge(&mut self, n: usize) -> Result<(), String> {
        if n > self.max.saturating_sub(self.used) {
            return Err("surface feature work budget exhausted".into());
        }
        self.used += n;
        Ok(())
    }
}
#[derive(Clone)]
enum Expr {
    Leaf(u64, bool),
    Union(Vec<Expr>),
    Intersection(Vec<Expr>),
}

impl SurfaceFeatures {
    pub fn new(surface: TriangleSurface, radius: f64, max_work: usize) -> Result<Self, String> {
        if !radius.is_finite() || radius <= 0. || !(radius as f32).is_finite() || radius as f32 <= 0. {
            return Err("surface feature radius must be positive and finite at native precision".into());
        }
        let mut budget = Budget { used: 0, max: max_work };
        let reference = d(surface.vertices()[0]);
        let mut features = Vec::new();
        let mut planes = BTreeMap::new();
        let mut capsules = BTreeMap::new();
        let mut triangles = Vec::new();
        for triangle in surface.triangles() {
            budget.charge(32)?;
            let p = triangle.map(|i| d(surface.vertices()[i as usize]));
            let normal = canonical_unit(cross(sub(p[1], p[0]), sub(p[2], p[0])))?;
            let winding = dot(normal, cross(sub(p[1], p[0]), sub(p[2], p[0]))).signum();
            let oriented = scale(normal, winding);
            let anchor = p[triangle.iter().enumerate().min_by_key(|(_, i)| *i).unwrap().0];
            let mut bounds = Vec::new();
            bounds.push(insert_plane(oriented, anchor, radius, reference, &mut features, &mut planes)?);
            bounds.push(insert_plane(scale(oriented, -1.), anchor, radius, reference, &mut features, &mut planes)?);
            let mut edges = Vec::new();
            for (a, b) in [(0, 1), (1, 2), (2, 0)] {
                let outside = unit(cross(sub(p[b], p[a]), oriented))?;
                let origin = if triangle[a] < triangle[b] { p[a] } else { p[b] };
                bounds.push(insert_plane(outside, origin, 0., reference, &mut features, &mut planes)?);
                let (a, b) = if bits(p[a]) < bits(p[b]) { (p[a], p[b]) } else { (p[b], p[a]) };
                let key = (bits(a), bits(b));
                let id = *capsules.entry(key).or_insert_with(|| {
                    let id = features.len() as u64;
                    features.push(Feature::Capsule { a, b, radius });
                    id
                });
                edges.push(id);
            }
            triangles.push(TriangleFeatures::Rounded {
                planes: bounds.try_into().unwrap(),
                capsules: edges.try_into().unwrap(),
            });
        }
        Ok(Self {
            surface,
            radius,
            features,
            triangles,
            capsule_enclosures: BTreeMap::new(),
            construction_work: budget.used,
        })
    }
    /// Convex supporting-plane capsules avoid disconnected inside-biased affine
    /// samples of curved distance leaves. They enclose, rather than replace, the
    /// native target; projection and rounded-feature validation remain required.
    pub fn with_supporting_planes(surface: TriangleSurface, radius: f64, max_work: usize) -> Result<Self, String> {
        if !((radius * 3_f64.sqrt()) as f32).is_finite() {
            return Err("capsule enclosure bounds exceed finite native precision".into());
        }
        let mut result = Self::new(surface, radius, max_work)?;
        let mut budget = Budget { used: result.construction_work, max: max_work };
        let reference = d(result.surface.vertices()[0]);
        let mut planes = BTreeMap::new();
        for (id, feature) in result.features.iter().enumerate() {
            if let Feature::Plane { normal, base, offset, .. } = feature {
                planes.insert(
                    [
                        clean(normal[0]).to_bits(),
                        clean(normal[1]).to_bits(),
                        clean(normal[2]).to_bits(),
                        clean(*base).to_bits(),
                        clean(*offset).to_bits(),
                    ],
                    id as u64,
                );
            }
        }
        let capsules: Vec<_> = result
            .features
            .iter()
            .enumerate()
            .filter_map(|(i, f)| match f {
                Feature::Capsule { a, b, .. } => Some((i as u64, *a, *b)),
                _ => None,
            })
            .collect();
        for (id, a, b) in capsules {
            budget.charge(32)?;
            let axis = unit(sub(b, a))?;
            let least = (0..3).min_by(|&i, &j| axis[i].abs().total_cmp(&axis[j].abs())).unwrap();
            let basis = std::array::from_fn(|i| if i == least { 1.0 } else { 0.0 });
            let u = unit(cross(axis, basis))?;
            let v = unit(cross(axis, u))?;
            let radial = [u, v, scale(u, -1.), scale(v, -1.)];
            let mut normals = radial.to_vec();
            normals.extend([axis, scale(axis, -1.)]);
            for radial in radial {
                normals.push(unit(add(axis, radial))?);
                normals.push(unit(sub(radial, axis))?);
            }
            let mut bounds = Vec::new();
            for normal in normals {
                budget.charge(16)?;
                let canonical = canonical_unit(normal)?;
                let oriented = if dot(canonical, normal) < 0. { scale(canonical, -1.) } else { canonical };
                let origin = if dot(oriented, sub(a, reference)) >= dot(oriented, sub(b, reference)) { a } else { b };
                bounds.push(insert_plane(oriented, origin, radius, reference, &mut result.features, &mut planes)?);
            }
            result.capsule_enclosures.insert(id, bounds);
        }
        result.construction_work = budget.used;
        Ok(result)
    }
    /// Initial enclosure using one common axis-aligned cube kernel for every
    /// authored triangle. The triangle-box separating axes give at most thirteen
    /// unoriented axes, hence twenty-six supporting halfspaces per triangle.
    ///
    /// This represents triangle + [-radius,radius]^3, with outward-rounded
    /// support bounds. The cube contains the radius ball; it is only an initial
    /// enclosure of the unchanged spherical native surface. Native projection,
    /// residual, topology, intersection and delivery checks remain necessary.
    pub fn with_common_box_kernel(surface: TriangleSurface, radius: f64, max_work: usize) -> Result<Self, String> {
        if !radius.is_finite() || radius <= 0. || radius as f32 <= 0. || !((radius * 3_f64.sqrt()) as f32).is_finite() {
            return Err("common box kernel radius must be positive and finite at native precision".into());
        }
        let mut budget = Budget { used: 0, max: max_work };
        let reference = d(surface.vertices()[0]);
        let mut features = Vec::new();
        let mut planes = BTreeMap::new();
        let mut triangles = Vec::new();
        const AXES: [D3; 3] = [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]];
        for triangle in surface.triangles() {
            budget.charge(32)?;
            let p = triangle.map(|i| d(surface.vertices()[i as usize]));
            let mut axes = AXES.to_vec();
            axes.push(cross(sub(p[1], p[0]), sub(p[2], p[0])));
            for (a, b) in [(0, 1), (1, 2), (2, 0)] {
                for axis in AXES {
                    let normal = cross(sub(p[b], p[a]), axis);
                    if normal.iter().any(|&x| x != 0.) {
                        axes.push(normal);
                    }
                }
            }
            let mut seen = BTreeSet::new();
            let mut bounds = Vec::new();
            for axis in axes {
                budget.charge(16)?;
                let normal = canonical_unit(axis)?;
                if !seen.insert(bits(normal)) {
                    continue;
                }
                for sign in [1., -1.] {
                    budget.charge(32)?;
                    let support = p
                        .iter()
                        .map(|&p| dot_upper(scale(normal, sign), p, reference))
                        .fold(f64::NEG_INFINITY, f64::max);
                    let l1 = add_upper(add_upper(normal[0].abs(), normal[1].abs()), normal[2].abs());
                    let offset = multiply_upper(radius, l1);
                    let id = insert_canonical_plane(
                        normal,
                        reference,
                        support * sign,
                        offset * sign,
                        true,
                        &mut features,
                        &mut planes,
                    )?;
                    bounds.push((id, sign < 0.));
                }
            }
            triangles.push(TriangleFeatures::Box(bounds));
        }
        Ok(Self {
            surface,
            radius,
            features,
            triangles,
            capsule_enclosures: BTreeMap::new(),
            construction_work: budget.used,
        })
    }
    pub fn feature_count(&self) -> usize {
        self.features.len()
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }
    /// Small unpruned reference program, also useful for direct local/global
    /// comparison. Large sources must use the cell-local interface.
    pub fn global_program(&self, max_work: usize) -> Result<FeatureProgram, String> {
        if self.features.len() + 1 > crate::meshing_boolean::MAX_BOOLEAN_CHANNELS || self.triangles.len() > 100 {
            return Err("surface global reference exceeds bounded Boolean program size; use local cells".into());
        }
        let mut budget = Budget { used: 0, max: max_work };
        budget.charge(16 * self.triangles.len() + self.features.len())?;
        let mut parts = Vec::new();
        let mut capsules = BTreeSet::new();
        for triangle in &self.triangles {
            budget.charge(triangle.planes().len().saturating_sub(5))?;
            parts.push(Expr::Intersection(
                triangle.planes().iter().map(|&(id, negate)| Expr::Leaf(id, negate)).collect(),
            ));
            capsules.extend(triangle.capsules().iter().copied());
        }
        parts.extend(capsules.into_iter().map(|id| {
            if let Some(planes) = self.capsule_enclosures.get(&id) {
                Expr::Intersection(planes.iter().map(|&(id, negate)| Expr::Leaf(id, negate)).collect())
            } else {
                Expr::Leaf(id, false)
            }
        }));
        let expr = Expr::Union(parts);
        let mut ids = BTreeSet::new();
        collect(&expr, &mut ids);
        let mapping = ids.iter().enumerate().map(|(i, &id)| (id, i)).collect();
        Ok(FeatureProgram { ids: ids.into_iter().collect(), expression: convert(expr, &mapping), work: budget.used })
    }
    /// Counted original feature values without changing global identity or sign.
    pub fn sample_features(&self, ids: &[u64], point: Vec3, max_work: usize) -> Result<FeatureValues, String> {
        if ids.len() > crate::meshing_boolean::MAX_BOOLEAN_CHANNELS || d(point).iter().any(|v| !v.is_finite()) {
            return Err("surface feature query requires finite point and at most128 feature IDs".into());
        }
        let mut budget = Budget { used: 0, max: max_work };
        budget.charge(ids.len())?;
        let mut values = Vec::with_capacity(ids.len());
        for &id in ids {
            let value = if id == INSIDE_ID {
                budget.charge(1)?;
                -1.0
            } else {
                let feature = self
                    .features
                    .get(usize::try_from(id).map_err(|_| "unknown surface feature ID")?)
                    .ok_or("unknown surface feature ID")?;
                budget.charge(feature.cost())?;
                feature.value(d(point)) as f32
            };
            if !value.is_finite() {
                return Err("surface feature query exceeded finite native precision".into());
            }
            values.push(value);
        }
        Ok(FeatureValues { values, work: budget.used })
    }
    /// Caller supplies the same eight local points for shared world lattice nodes.
    /// Only features proven irrelevant by conservative bounds/convex halfspaces
    /// are omitted. Values retain globally canonical feature orientation/identity.
    pub fn sample_cell(&self, points: [Vec3; 8], max_work: usize) -> Result<LocalCellData, String> {
        let mut budget = Budget { used: 0, max: max_work };
        budget.charge(16)?;
        if points.iter().any(|p| d(*p).iter().any(|v| !v.is_finite())) {
            return Err("surface feature cell points must be finite".into());
        }
        let lo = std::array::from_fn(|i| points.iter().map(|p| d(*p)[i] as f32).fold(f32::INFINITY, f32::min));
        let hi = std::array::from_fn(|i| points.iter().map(|p| d(*p)[i] as f32).fold(f32::NEG_INFINITY, f32::max));
        self.cell_with_bounds(points, lo, hi, budget)
    }
    /// Avoid eight transform/bounds reductions when the lattice is already in
    /// source coordinates. The endpoint validation proves every generated corner.
    pub fn sample_axis_aligned_cell(&self, min: Vec3, max: Vec3, max_work: usize) -> Result<LocalCellData, String> {
        let mut budget = Budget { used: 0, max: max_work };
        budget.charge(4)?;
        let lo = [min.x, min.y, min.z];
        let hi = [max.x, max.y, max.z];
        if lo.iter().chain(&hi).any(|v| !v.is_finite()) || (0..3).any(|i| lo[i] >= hi[i]) {
            return Err("surface feature cell bounds must be finite and strictly increasing".into());
        }
        let points = std::array::from_fn(|i| {
            Vec3::new(
                if i & 1 == 0 { lo[0] } else { hi[0] },
                if i & 2 == 0 { lo[1] } else { hi[1] },
                if i & 4 == 0 { lo[2] } else { hi[2] },
            )
        });
        self.cell_with_bounds(points, lo, hi, budget)
    }
    fn cell_with_bounds(
        &self,
        points: [Vec3; 8],
        lo: [f32; 3],
        hi: [f32; 3],
        mut budget: Budget,
    ) -> Result<LocalCellData, String> {
        let support_radius = if self.capsule_enclosures.is_empty() { self.radius } else { self.radius * 3_f64.sqrt() };
        let mut padding = support_radius as f32;
        if f64::from(padding) < support_radius {
            padding = padding.next_up();
        }
        let candidates = self.surface.triangle_candidates_bounded(v(lo), v(hi), padding, budget.max - budget.used)?;
        budget.charge(candidates.work)?;
        let points = points.map(d);
        let mut values = BTreeMap::new();
        let mut parts = Vec::new();
        let mut edge_ids = BTreeSet::new();
        for triangle in candidates.triangles {
            budget.charge(1)?;
            let features = &self.triangles[triangle as usize];
            let mut planes = Vec::new();
            let mut outside = false;
            for &(id, negate) in features.planes() {
                let samples = self.samples(id, points, &mut values, &mut budget)?;
                let oriented = samples.map(|x| if negate { -x } else { x });
                let minimum = oriented.iter().copied().fold(f32::INFINITY, f32::min);
                let maximum = oriented.iter().copied().fold(f32::NEG_INFINITY, f32::max);
                if minimum > 0. {
                    outside = true;
                    break;
                }
                if minimum == 0. && maximum == 0. {
                    return Err("surface feature plane lost cell variation at native precision".into());
                }
                if maximum >= 0. {
                    planes.push(Expr::Leaf(id, negate));
                }
            }
            if !outside {
                if planes.is_empty() {
                    return Ok(inside_cell(budget.used));
                }
                parts.push(Expr::Intersection(planes));
            }
            edge_ids.extend(features.capsules().iter().copied());
        }
        for id in edge_ids {
            budget.charge(8)?;
            let Feature::Capsule { a, b, .. } = self.features[id as usize] else { unreachable!() };
            if !segment_box(a, b, lo.map(f64::from), hi.map(f64::from), support_radius) {
                continue;
            }
            if let Some(bounds) = self.capsule_enclosures.get(&id) {
                let mut active = Vec::new();
                let mut outside = false;
                for &(id, negate) in bounds {
                    let samples = self.samples(id, points, &mut values, &mut budget)?;
                    let oriented = samples.map(|x| if negate { -x } else { x });
                    let min = oriented.iter().copied().fold(f32::INFINITY, f32::min);
                    let max = oriented.iter().copied().fold(f32::NEG_INFINITY, f32::max);
                    if min > 0. {
                        outside = true;
                        break;
                    }
                    if min == 0. && max == 0. {
                        return Err("capsule enclosure plane lost cell variation at native precision".into());
                    }
                    if max >= 0. {
                        active.push(Expr::Leaf(id, negate));
                    }
                }
                if !outside {
                    if active.is_empty() {
                        return Ok(inside_cell(budget.used));
                    }
                    parts.push(Expr::Intersection(active));
                }
                continue;
            }
            let samples = self.samples(id, points, &mut values, &mut budget)?;
            if samples.iter().all(|&x| x < 0.) {
                return Ok(inside_cell(budget.used));
            }
            // A convex capsule may be wholly between corner samples. Retain it:
            // its active-plane sampling is the mesher's responsibility.
            parts.push(Expr::Leaf(id, false));
        }
        if parts.is_empty() {
            return Ok(LocalCellData { features: vec![], expression: None, work: budget.used });
        }
        let expression = Expr::Union(parts);
        let mut used = BTreeSet::new();
        collect(&expression, &mut used);
        if used.len() > crate::meshing_boolean::MAX_BOOLEAN_CHANNELS {
            return Err(format!("surface cell has {} feature channels; exceeds128", used.len()));
        }
        let mapping: BTreeMap<_, _> = used.iter().enumerate().map(|(i, &id)| (id, i)).collect();
        let features = used
            .into_iter()
            .map(|id| LocalFeature { id, values: if id == INSIDE_ID { [-1.; 8] } else { values[&id] } })
            .collect();
        Ok(LocalCellData { features, expression: Some(convert(expression, &mapping)), work: budget.used })
    }
    fn samples(
        &self,
        id: u64,
        points: [D3; 8],
        values: &mut BTreeMap<u64, [f32; 8]>,
        budget: &mut Budget,
    ) -> Result<[f32; 8], String> {
        if let Some(v) = values.get(&id) {
            return Ok(*v);
        }
        let feature = &self.features[id as usize];
        budget.charge(8 * feature.cost())?;
        let samples = points.map(|p| feature.value(p) as f32);
        if samples.iter().any(|v| !v.is_finite()) {
            return Err("surface feature evaluation exceeded native finite precision".into());
        }
        values.insert(id, samples);
        Ok(samples)
    }
    /// Explicit set-membership diagnostic, not a distance observation.
    /// Prism/capsule constructors query their native rounded target here;
    /// `with_common_box_kernel` queries its enlarged cube-kernel inside set.
    pub fn contains(&self, point: Vec3) -> bool {
        let p = d(point);
        self.triangles.iter().any(|t| {
            t.planes().iter().all(|&(id, negate)| {
                let x = self.features[id as usize].value(p);
                if negate {
                    x >= 0.
                } else {
                    x <= 0.
                }
            }) || t.capsules().iter().any(|&id| self.features[id as usize].value(p) <= 0.)
        })
    }
}
fn inside_cell(work: usize) -> LocalCellData {
    LocalCellData {
        features: vec![LocalFeature { id: INSIDE_ID, values: [-1.; 8] }],
        expression: Some(BooleanExpr::Leaf(0)),
        work,
    }
}
fn insert_plane(
    normal: D3,
    origin: D3,
    offset: f64,
    reference: D3,
    features: &mut Vec<Feature>,
    planes: &mut BTreeMap<[u64; 5], u64>,
) -> Result<(u64, bool), String> {
    let canonical = canonical_unit(normal)?;
    let sign = if dot(canonical, normal) < 0. { -1. } else { 1. };
    let offset = offset * sign;
    let base = dot(canonical, sub(origin, reference));
    let id = insert_canonical_plane(canonical, reference, base, offset, false, features, planes)?;
    Ok((id, sign < 0.))
}
fn insert_canonical_plane(
    canonical: D3,
    reference: D3,
    base: f64,
    offset: f64,
    box_plane: bool,
    features: &mut Vec<Feature>,
    planes: &mut BTreeMap<[u64; 5], u64>,
) -> Result<u64, String> {
    if !base.is_finite() || !offset.is_finite() {
        return Err("surface supporting plane exceeds finite precision".into());
    }
    let key = [
        clean(canonical[0]).to_bits(),
        clean(canonical[1]).to_bits(),
        clean(canonical[2]).to_bits(),
        clean(base).to_bits(),
        clean(offset).to_bits(),
    ];
    let id = if let Some(&id) = planes.get(&key) {
        id
    } else {
        if features.len() >= crate::meshing_local::MAX_LOCAL_GLOBAL_FEATURES {
            return Err("surface feature dictionary exceeded bounded global feature count".into());
        }
        let id = features.len() as u64;
        features.push(if box_plane {
            Feature::BoxPlane { normal: canonical, origin: reference, base, offset }
        } else {
            Feature::Plane { normal: canonical, origin: reference, base, offset }
        });
        planes.insert(key, id);
        id
    };
    Ok(id)
}
fn sum_error(a: f64, b: f64, sum: f64) -> f64 {
    let virtual_b = sum - a;
    (a - (sum - virtual_b)) + (b - virtual_b)
}
fn add_upper(a: f64, b: f64) -> f64 {
    let sum = a + b;
    if sum_error(a, b, sum) > 0. {
        sum.next_up()
    } else {
        sum
    }
}
fn multiply_upper(a: f64, b: f64) -> f64 {
    let product = a * b;
    if a.mul_add(b, -product) > 0. {
        product.next_up()
    } else {
        product
    }
}
fn dot_upper(normal: D3, point: D3, reference: D3) -> f64 {
    (0..3).fold(0., |sum, i| {
        let delta = point[i] - reference[i];
        let error = sum_error(point[i], -reference[i], delta);
        let directed_delta = if normal[i] >= 0. && error > 0. {
            delta.next_up()
        } else if normal[i] < 0. && error < 0. {
            delta.next_down()
        } else {
            delta
        };
        add_upper(sum, multiply_upper(normal[i], directed_delta))
    })
}
fn collect(expr: &Expr, used: &mut BTreeSet<u64>) {
    match expr {
        Expr::Leaf(id, negate) => {
            used.insert(*id);
            if *negate {
                used.insert(INSIDE_ID);
            }
        }
        Expr::Union(v) | Expr::Intersection(v) => {
            for e in v {
                collect(e, used)
            }
        }
    }
}
fn convert(expr: Expr, mapping: &BTreeMap<u64, usize>) -> BooleanExpr {
    match expr {
        Expr::Leaf(id, false) => BooleanExpr::Leaf(mapping[&id]),
        Expr::Leaf(id, true) => BooleanExpr::Subtract(
            Box::new(BooleanExpr::Leaf(mapping[&INSIDE_ID])),
            Box::new(BooleanExpr::Leaf(mapping[&id])),
        ),
        Expr::Union(v) => combine(v.into_iter().map(|e| convert(e, mapping)).collect(), true),
        Expr::Intersection(v) => combine(v.into_iter().map(|e| convert(e, mapping)).collect(), false),
    }
}
fn combine(mut v: Vec<BooleanExpr>, union: bool) -> BooleanExpr {
    if v.len() == 1 {
        return v.pop().unwrap();
    }
    let right = v.split_off(v.len() / 2);
    let a = Box::new(combine(v, union));
    let b = Box::new(combine(right, union));
    if union {
        BooleanExpr::Union(a, b)
    } else {
        BooleanExpr::Intersection(a, b)
    }
}
fn segment_box(a: D3, b: D3, lo: D3, hi: D3, radius: f64) -> bool {
    let mut t0 = 0_f64;
    let mut t1 = 1_f64;
    for axis in 0..3 {
        let lo = (lo[axis] - radius).next_down();
        let hi = (hi[axis] + radius).next_up();
        let delta = b[axis] - a[axis];
        if delta == 0. {
            if a[axis] < lo || a[axis] > hi {
                return false;
            }
        } else {
            let x = (lo - a[axis]) / delta;
            let y = (hi - a[axis]) / delta;
            t0 = t0.max(x.min(y).next_down());
            t1 = t1.min(x.max(y).next_up());
            if t0 > t1 {
                return false;
            }
        }
    }
    true
}
fn canonical_unit(n: D3) -> Result<D3, String> {
    let i = (0..3).max_by(|&a, &b| n[a].abs().total_cmp(&n[b].abs())).unwrap();
    if n[i] == 0. || !n[i].is_finite() {
        return Err("surface feature has degenerate plane".into());
    }
    unit(scale(n, 1. / n[i]))
}
fn unit(p: D3) -> Result<D3, String> {
    let n = norm(p);
    if n == 0. || !n.is_finite() {
        return Err("surface feature has invalid normal".into());
    }
    Ok(scale(p, 1. / n))
}
fn clean(x: f64) -> f64 {
    if x == 0. {
        0.
    } else {
        x
    }
}
fn bits(p: D3) -> [u64; 3] {
    p.map(|x| clean(x).to_bits())
}
fn d(p: Vec3) -> D3 {
    [p.x as f64, p.y as f64, p.z as f64]
}
fn v(p: [f32; 3]) -> Vec3 {
    Vec3::new(p[0], p[1], p[2])
}
fn add(a: D3, b: D3) -> D3 {
    std::array::from_fn(|i| a[i] + b[i])
}
fn sub(a: D3, b: D3) -> D3 {
    std::array::from_fn(|i| a[i] - b[i])
}
fn scale(a: D3, s: f64) -> D3 {
    a.map(|x| x * s)
}
fn dot(a: D3, b: D3) -> f64 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}
fn norm(a: D3) -> f64 {
    a[0].hypot(a[1]).hypot(a[2])
}
fn cross(a: D3, b: D3) -> D3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
