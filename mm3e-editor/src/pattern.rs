//! Native planar outline and hole patterns, deterministic triangulation, explicit seams.
//! Original named controls and loops, final local topology, and world placement are durable.
//! Darts, seam allowances, grading and inferred boundary correspondence are not implemented.
use crate::{
    cloth::{ClothAsset, ClothPin, ClothSolverSettings, MAX_CLOTH_VERTICES},
    model::{
        array, identifier, range, vec, vector, Combination, Document, Entity, Modifiers, Pass, Shape, Surface, V3,
    },
    sewing::{self, BuiltPattern, PanelBoundaries, SeamChain, MAX_SEWN_PANELS},
};
use mm3e_kit::{
    cloth::ClothSeam,
    pattern::{Pattern2D, TriangulationOptions},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub const MAX_PATTERN_WORK: usize = 1_000_000;
pub const TRIANGULATION_ALGORITHM: &str = "planar-outlines-holes-midpoint-v1";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PatternPoint {
    pub id: String,
    /// Metres in the panel's explicit (axis_u, axis_v) plane, relative to origin.
    pub position: [f64; 2],
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PatternLoop {
    pub id: String,
    /// Open array describing a closed polygon; do not repeat the first point at the end.
    pub points: Vec<PatternPoint>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum PatternVertexRef {
    Control {
        id: String,
    },
    /// An explicitly selected final LOCAL vertex from preview_pattern_panel.
    Index {
        index: u32,
    },
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PatternPin {
    pub vertex: PatternVertexRef,
    #[serde(default)]
    pub target_object: Option<String>,
    /// World point when target_object is absent; otherwise that object's local point.
    pub point: V3,
    /// Optional source triangle and normalized barycentric attachment for a
    /// continuously deformed target surface.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_triangle: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub barycentric: Option<[f32; 3]>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PatternPanel {
    pub id: String,
    pub origin: V3,
    pub axis_u: V3,
    pub axis_v: V3,
    pub outer: PatternLoop,
    #[serde(default)]
    pub holes: Vec<PatternLoop>,
    /// Optional deterministic refinement; fails rather than exceeding the vertex/work cap.
    #[serde(default)]
    pub max_edge_m: Option<f64>,
    #[serde(default)]
    pub pins: Vec<PatternPin>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PatternBoundary {
    pub id: String,
    /// Final LOCAL loop vertices, outer CCW and holes CW; no repeated closing vertex.
    /// This orientation is reported; seam chains are still supplied in explicit order.
    pub vertices: Vec<u32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PatternPanelMesh {
    pub panel: String,
    pub points: Vec<[f64; 2]>,
    pub triangles: Vec<[u32; 3]>,
    pub boundary_loops: Vec<PatternBoundary>,
    /// Stable authored control IDs map to retained final LOCAL vertices.
    pub control_vertices: BTreeMap<String, u32>,
    pub vertex_offset: u32,
    pub triangle_offset: u32,
    pub triangulation_work: usize,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PatternAsset {
    pub triangulation_algorithm: String,
    pub panels: Vec<PatternPanel>,
    pub seams: Vec<SeamChain>,
    pub vertex_mass_kg: f32,
    /// Retained deterministic 2D triangulation and identity mapping, not a regenerated guess.
    pub meshes: Vec<PatternPanelMesh>,
    pub triangulation_work: usize,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PatternClothRequest {
    pub id: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub group: String,
    pub panels: Vec<PatternPanel>,
    /// Explicit ordered contiguous FINAL LOCAL boundary indices; never sorted/reversed.
    #[serde(default)]
    pub seams: Vec<SeamChain>,
    pub thickness_m: f32,
    pub vertex_mass_kg: f32,
    #[serde(default)]
    pub collision_object_ids: Vec<String>,
    #[serde(default)]
    pub settings: ClothSolverSettings,
    #[serde(default)]
    pub material: Surface,
}

fn panel_mesh(panel: &PatternPanel, max_vertices: usize, max_work: usize) -> Result<PatternPanelMesh, String> {
    identifier(&panel.id)?;
    vector(panel.origin, "pattern origin")?;
    vector(panel.axis_u, "pattern axis_u")?;
    vector(panel.axis_v, "pattern axis_v")?;
    let (u, v) = (vec(panel.axis_u), vec(panel.axis_v));
    if (u.length() - 1.0).abs() > 1e-5 || (v.length() - 1.0).abs() > 1e-5 || u.dot(v).abs() > 1e-5 {
        return Err("pattern axes must be unit and perpendicular".into());
    }
    if panel.holes.len().saturating_add(1) > max_vertices / 3 || panel.pins.len() > max_vertices {
        return Err("pattern loop or pin count exceeds the remaining garment vertex budget".into());
    }
    let loops: Vec<_> = std::iter::once(&panel.outer).chain(&panel.holes).collect();
    let mut loop_ids = BTreeSet::new();
    let mut point_ids = BTreeSet::new();
    let mut control_count = 0usize;
    for boundary in &loops {
        if boundary.points.len() < 3 {
            return Err("every named pattern loop requires at least three controls".into());
        }
        identifier(&boundary.id)?;
        if !loop_ids.insert(&boundary.id) {
            return Err("pattern boundary loop ids must be distinct".into());
        }
        control_count = control_count.saturating_add(boundary.points.len());
        if control_count > max_vertices {
            return Err("pattern control vertices exceed the remaining 256-vertex garment budget".into());
        }
        for point in &boundary.points {
            identifier(&point.id)?;
            if !point_ids.insert(&point.id) {
                return Err("pattern control point ids must be distinct within a panel".into());
            }
            if point.position.iter().any(|p| !p.is_finite() || p.abs() > 20.0) {
                return Err("pattern coordinates must be finite and within [-20,20] metres".into());
            }
        }
    }
    for pin in &panel.pins {
        match (&pin.target_object, pin.target_triangle, pin.barycentric) {
            (Some(_), Some(_), Some(weights)) => {
                crate::cloth::normalized_barycentric(weights)?;
            }
            (Some(_), None, None) | (None, None, None) => {}
            _ => return Err("pattern pin target_triangle and barycentric must be supplied together with target_object and normalized weights".into()),
        }
    }
    if panel.max_edge_m.is_some_and(|length| !length.is_finite() || !(0.00001..=40.0).contains(&length)) {
        return Err("pattern max_edge_m must be finite and in [0.00001,40] metres".into());
    }
    let input = Pattern2D {
        outer: panel.outer.points.iter().map(|p| p.position).collect(),
        holes: panel.holes.iter().map(|h| h.points.iter().map(|p| p.position).collect()).collect(),
    };
    let mesh = input.triangulate(TriangulationOptions { max_edge_length: panel.max_edge_m, max_vertices, max_work })?;
    if mesh.boundary_loops.len() != loops.len() || mesh.control_vertices.len() != loops.len() {
        return Err("pattern kernel did not preserve loop and control provenance".into());
    }
    let mut controls = BTreeMap::new();
    for (boundary, indices) in loops.iter().zip(&mesh.control_vertices) {
        if indices.len() != boundary.points.len() {
            return Err("pattern kernel changed authored control cardinality".into());
        }
        for (point, &index) in boundary.points.iter().zip(indices) {
            if mesh.points.get(index as usize) != Some(&point.position) {
                return Err("pattern kernel changed an authored control point".into());
            }
            controls.insert(point.id.clone(), index);
        }
    }
    Ok(PatternPanelMesh {
        panel: panel.id.clone(),
        points: mesh.points,
        triangles: mesh.triangles,
        boundary_loops: loops
            .iter()
            .zip(mesh.boundary_loops)
            .map(|(boundary, vertices)| PatternBoundary { id: boundary.id.clone(), vertices })
            .collect(),
        control_vertices: controls,
        vertex_offset: 0,
        triangle_offset: 0,
        triangulation_work: mesh.work,
    })
}

pub fn preview(panel: &PatternPanel) -> Result<Value, String> {
    serde_json::to_value(panel_mesh(panel, MAX_CLOTH_VERTICES, MAX_PATTERN_WORK)?).map_err(|e| e.to_string())
}

fn boundaries(meshes: &[PatternPanelMesh]) -> Result<PanelBoundaries<'_>, String> {
    let mut result = BTreeMap::new();
    for mesh in meshes {
        if mesh.points.len() > MAX_CLOTH_VERTICES
            || mesh.vertex_offset as usize + mesh.points.len() > MAX_CLOTH_VERTICES
        {
            return Err("pattern retained panel exceeds garment vertex budget".into());
        }
        let mut counts = BTreeMap::new();
        for &[a, b, c] in &mesh.triangles {
            if [a, b, c].iter().any(|i| *i as usize >= mesh.points.len()) {
                return Err("pattern retained triangle index is invalid".into());
            }
            for (a, b) in [(a, b), (b, c), (c, a)] {
                *counts.entry((a.min(b), a.max(b))).or_insert(0u32) += 1;
            }
        }
        let edges = counts.into_iter().filter_map(|(pair, count)| (count == 1).then_some(pair)).collect();
        if result.insert(mesh.panel.as_str(), (mesh.vertex_offset, mesh.points.len() as u32, edges)).is_some() {
            return Err("duplicate retained pattern panel id".into());
        }
    }
    Ok(result)
}

pub(crate) fn stitches(asset: &PatternAsset) -> Result<Vec<ClothSeam>, String> {
    sewing::seam_constraints(&boundaries(&asset.meshes)?, &asset.seams)
}

fn assemble(
    panels: &[PatternPanel],
    seams: &[SeamChain],
) -> Result<(BuiltPattern, Vec<PatternPanelMesh>, usize), String> {
    if !(1..=MAX_SEWN_PANELS).contains(&panels.len()) {
        return Err("pattern cloth requires 1..16 panels".into());
    }
    if seams.len() > 256 {
        return Err("pattern cloth supports at most 256 seam chains".into());
    }
    let mut names = BTreeSet::new();
    let mut built = BuiltPattern { vertices: vec![], triangles: vec![], pins: vec![], stitches: vec![] };
    let mut meshes = vec![];
    let mut work = 0usize;
    for panel in panels {
        if !names.insert(&panel.id) {
            return Err("duplicate pattern panel id".into());
        }
        let mut mesh = panel_mesh(panel, MAX_CLOTH_VERTICES - built.vertices.len(), MAX_PATTERN_WORK - work)?;
        work += mesh.triangulation_work;
        mesh.vertex_offset = built.vertices.len() as u32;
        mesh.triangle_offset = built.triangles.len() as u32;
        let mut pinned = BTreeSet::new();
        for pin in &panel.pins {
            let vertex = match &pin.vertex {
                PatternVertexRef::Control { id } => {
                    *mesh.control_vertices.get(id).ok_or_else(|| format!("missing pattern pin control {id}"))?
                }
                PatternVertexRef::Index { index } => *index,
            };
            if vertex as usize >= mesh.points.len() || !pinned.insert(vertex) {
                return Err("pattern pins require distinct existing local vertices".into());
            }
            built.pins.push(ClothPin {
                vertex: vertex + mesh.vertex_offset,
                target_object: pin.target_object.clone(),
                point: pin.point,
                target_triangle: pin.target_triangle,
                barycentric: pin.barycentric,
            });
        }
        for point in &mesh.points {
            let world =
                array(vec(panel.origin) + vec(panel.axis_u) * point[0] as f32 + vec(panel.axis_v) * point[1] as f32);
            vector(world, "pattern world vertex")?;
            built.vertices.push(world);
        }
        built.triangles.extend(mesh.triangles.iter().map(|triangle| triangle.map(|i| i + mesh.vertex_offset)));
        meshes.push(mesh);
    }
    built.stitches = sewing::seam_constraints(&boundaries(&meshes)?, seams)?;
    Ok((built, meshes, work))
}

/// Rebuild deterministically to verify that durable provenance matches actual cloth inputs.
pub(crate) fn build(asset: &PatternAsset) -> Result<BuiltPattern, String> {
    if asset.triangulation_algorithm != TRIANGULATION_ALGORITHM {
        return Err("unsupported retained pattern triangulation algorithm".into());
    }
    range(asset.vertex_mass_kg, 0.0001, 100.0, "pattern vertex_mass_kg")?;
    let (built, meshes, work) = assemble(&asset.panels, &asset.seams)?;
    if meshes != asset.meshes || work != asset.triangulation_work {
        return Err(
            "pattern recipe must reproduce the retained local topology, named loops, controls and work exactly".into(),
        );
    }
    Ok(built)
}

pub fn create(document: &mut Document, request: &PatternClothRequest) -> Result<(), String> {
    identifier(&request.id)?;
    if document.objects.iter().any(|o| o.id == request.id) {
        return Err(format!("object {} already exists", request.id));
    }
    range(request.vertex_mass_kg, 0.0001, 100.0, "pattern vertex_mass_kg")?;
    let (built, meshes, work) = assemble(&request.panels, &request.seams)?;
    let mut inverse_masses = vec![1.0 / request.vertex_mass_kg; built.vertices.len()];
    for pin in &built.pins {
        inverse_masses[pin.vertex as usize] = 0.0;
    }
    let asset = ClothAsset {
        id: request.id.clone(),
        rest_vertices: built.vertices.clone(),
        triangles: built.triangles.clone(),
        thickness_m: request.thickness_m,
        inverse_masses,
        pins: built.pins,
        collision_object_ids: request.collision_object_ids.clone(),
        settings: request.settings.clone(),
        sewing: None,
        pattern: Some(PatternAsset {
            triangulation_algorithm: TRIANGULATION_ALGORITHM.into(),
            panels: request.panels.clone(),
            seams: request.seams.clone(),
            vertex_mass_kg: request.vertex_mass_kg,
            meshes,
            triangulation_work: work,
        }),
        cache: None,
    };
    let mut candidate = document.clone();
    candidate.objects.push(Entity {
        id: request.id.clone(),
        label: request.label.clone(),
        group: request.group.clone(),
        role: "pattern_cloth".into(),
        shape: Shape::Surface {
            vertices: built.vertices,
            triangles: built.triangles,
            thickness_m: request.thickness_m,
        },
        position: [0.0; 3],
        rotation_degrees: [0.0; 3],
        scale: 1.0,
        material: request.material.clone(),
        combine: Combination::Union,
        modifiers: Modifiers::default(),
    });
    candidate.cloths.push(asset);
    candidate.compile(&Pass::Beauty)?;
    *document = candidate;
    Ok(())
}

pub fn update(document: &mut Document, request: &PatternClothRequest) -> Result<(), String> {
    let asset_index = document
        .cloths
        .iter()
        .position(|c| c.id == request.id && c.pattern.is_some())
        .ok_or_else(|| format!("missing outline pattern cloth {}", request.id))?;
    let object_index =
        document.objects.iter().position(|o| o.id == request.id).ok_or("missing pattern cloth object")?;
    let mut candidate = document.clone();
    candidate.cloths.remove(asset_index);
    candidate.objects.remove(object_index);
    create(&mut candidate, request)?;
    let object = candidate.objects.pop().ok_or("missing newly created pattern object")?;
    let asset = candidate.cloths.pop().ok_or("missing newly created pattern asset")?;
    candidate.objects.insert(object_index, object);
    candidate.cloths.insert(asset_index, asset);
    candidate.compile(&Pass::Beauty)?;
    *document = candidate;
    Ok(())
}
