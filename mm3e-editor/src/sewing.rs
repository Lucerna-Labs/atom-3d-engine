//! Native rectangular pattern panels and explicit ordered stitch correspondence.
//! This is sewn-panel assembly, not arbitrary outline drafting, darts, cutting,
//! seam allowances, grading, or a general pattern-CAD system.
use crate::{
    cloth::{self, ClothAsset, ClothPin, ClothSolverSettings, MAX_CLOTH_VERTICES},
    model::{identifier, range, Combination, Document, Entity, Modifiers, Pass, Shape, Surface, V3},
};
use mm3e_kit::cloth::ClothSeam;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const MAX_SEWN_PANELS: usize = 16;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SewingPanel {
    /// Stable identity within this garment. Local vertex indices use v rows/u columns.
    pub id: String,
    pub origin: V3,
    pub axis_u: V3,
    pub axis_v: V3,
    pub segments: [u32; 2],
    pub width_m: f32,
    pub height_m: f32,
    #[serde(default)]
    pub pins: Vec<ClothPin>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SeamChain {
    pub panel_a: String,
    /// Ordered contiguous boundary vertices in panel A's LOCAL vertex numbering.
    pub chain_a: Vec<u32>,
    pub panel_b: String,
    /// Explicit partner order; reverse this array yourself when correspondence requires it.
    /// The editor never sorts, reverses, welds, or guesses a chain's orientation.
    pub chain_b: Vec<u32>,
    #[serde(default)]
    pub rest_length_m: f32,
    #[serde(default)]
    pub compliance: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SewnPattern {
    pub panels: Vec<SewingPanel>,
    pub seams: Vec<SeamChain>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SewnClothRequest {
    pub id: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub group: String,
    pub panels: Vec<SewingPanel>,
    /// Empty seams provide an explicit disconnected-panel negative control.
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

pub(crate) struct BuiltPattern {
    pub vertices: Vec<V3>,
    pub triangles: Vec<[u32; 3]>,
    pub pins: Vec<ClothPin>,
    pub stitches: Vec<ClothSeam>,
}

pub(crate) type PanelBoundaries<'a> = BTreeMap<&'a str, (u32, u32, BTreeSet<(u32, u32)>)>;

pub(crate) fn build(pattern: &SewnPattern) -> Result<BuiltPattern, String> {
    if !(2..=MAX_SEWN_PANELS).contains(&pattern.panels.len()) {
        return Err(format!("sewn cloth needs 2..={MAX_SEWN_PANELS} individually placed panels"));
    }
    if pattern.seams.len() > 256 {
        return Err("sewn cloth supports at most 256 seam chains".into());
    }
    let mut built = BuiltPattern { vertices: vec![], triangles: vec![], pins: vec![], stitches: vec![] };
    let mut panels = BTreeMap::new();
    for panel in &pattern.panels {
        identifier(&panel.id)?;
        if panels.contains_key(panel.id.as_str()) {
            return Err(format!("duplicate sewn panel ID {}", panel.id));
        }
        let (vertices, triangles) = cloth::panel_geometry(
            panel.origin,
            panel.axis_u,
            panel.axis_v,
            panel.segments,
            panel.width_m,
            panel.height_m,
        )?;
        if built.vertices.len() + vertices.len() > MAX_CLOTH_VERTICES {
            return Err("all sewn panels together must fit the 256-vertex garment budget".into());
        }
        let offset = built.vertices.len() as u32;
        let count = vertices.len() as u32;
        let mut edge_counts = BTreeMap::new();
        for &[a, b, c] in &triangles {
            for (a, b) in [(a, b), (b, c), (c, a)] {
                *edge_counts.entry((a.min(b), a.max(b))).or_insert(0u32) += 1;
            }
        }
        let boundary: BTreeSet<_> = edge_counts.into_iter().filter_map(|(edge, n)| (n == 1).then_some(edge)).collect();
        let mut pins = BTreeSet::new();
        for pin in &panel.pins {
            if pin.vertex >= count || !pins.insert(pin.vertex) {
                return Err(format!("panel {} pins require distinct existing LOCAL vertex indices", panel.id));
            }
            let mut pin = pin.clone();
            pin.vertex += offset;
            built.pins.push(pin);
        }
        panels.insert(panel.id.as_str(), (offset, count, boundary));
        built.vertices.extend(vertices);
        built.triangles.extend(triangles.into_iter().map(|triangle| triangle.map(|i| i + offset)));
    }
    built.stitches = seam_constraints(&panels, &pattern.seams)?;
    Ok(built)
}

/// Shared exact boundary-correspondence policy for rectangular and outline panels.
/// Coordinates, orientation, welding and correspondence are never inferred here.
pub(crate) fn seam_constraints(panels: &PanelBoundaries<'_>, seams: &[SeamChain]) -> Result<Vec<ClothSeam>, String> {
    if seams.len() > 256 {
        return Err("sewn cloth supports at most 256 seam chains".into());
    }
    let mut stitches = vec![];
    let mut stitch_pairs = BTreeSet::new();
    for seam in seams {
        if seam.panel_a == seam.panel_b {
            return Err("seam chains must join two different panels; same-panel darts are not implemented".into());
        }
        range(seam.rest_length_m, 0.0, 20.0, "seam rest_length_m")?;
        range(seam.compliance, 0.0, 1.0, "seam compliance")?;
        if seam.chain_a.len() != seam.chain_b.len() || seam.chain_a.len() < 2 {
            return Err("seam chains need equal counts of at least two ordered vertices".into());
        }
        let a = panels.get(seam.panel_a.as_str()).ok_or_else(|| format!("missing seam panel {}", seam.panel_a))?;
        let b = panels.get(seam.panel_b.as_str()).ok_or_else(|| format!("missing seam panel {}", seam.panel_b))?;
        for (chain, (_, count, boundary)) in [(&seam.chain_a, a), (&seam.chain_b, b)] {
            if chain.len() > *count as usize {
                return Err("seam chain count exceeds its panel's local vertex count".into());
            }
            let distinct: BTreeSet<_> = chain.iter().collect();
            if distinct.len() != chain.len() || chain.iter().any(|i| i >= count) {
                return Err("seam chain contains repeated or out-of-range local vertices".into());
            }
            if chain.windows(2).any(|pair| !boundary.contains(&(pair[0].min(pair[1]), pair[0].max(pair[1])))) {
                return Err("seam chain must follow contiguous boundary edges in the explicit supplied order".into());
            }
        }
        for (&i, &j) in seam.chain_a.iter().zip(&seam.chain_b) {
            let vertices = [i + a.0, j + b.0];
            let pair = (vertices[0].min(vertices[1]), vertices[0].max(vertices[1]));
            if !stitch_pairs.insert(pair) {
                return Err("duplicate stitch pair across seam chains".into());
            }
            if stitches.len() >= 1024 {
                return Err("sewn cloth exceeds the 1024-stitch garment budget".into());
            }
            stitches.push(ClothSeam { vertices, rest_length: seam.rest_length_m, compliance: seam.compliance });
        }
    }
    Ok(stitches)
}

pub fn create(document: &mut Document, request: &SewnClothRequest) -> Result<(), String> {
    identifier(&request.id)?;
    if document.objects.iter().any(|o| o.id == request.id) {
        return Err(format!("object {} already exists", request.id));
    }
    range(request.vertex_mass_kg, 0.0001, 100.0, "sewn cloth vertex_mass_kg")?;
    let pattern = SewnPattern { panels: request.panels.clone(), seams: request.seams.clone() };
    let built = build(&pattern)?;
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
        cache: None,
        sewing: Some(pattern),
        pattern: None,
    };
    let mut candidate = document.clone();
    candidate.objects.push(Entity {
        id: request.id.clone(),
        label: request.label.clone(),
        group: request.group.clone(),
        role: "sewn_cloth".into(),
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

pub fn update(document: &mut Document, request: &SewnClothRequest) -> Result<(), String> {
    let asset_index = document
        .cloths
        .iter()
        .position(|c| c.id == request.id && c.sewing.is_some())
        .ok_or_else(|| format!("missing sewn cloth {}", request.id))?;
    let object_index = document.objects.iter().position(|o| o.id == request.id).ok_or("missing sewn cloth object")?;
    let mut candidate = document.clone();
    candidate.cloths.remove(asset_index);
    candidate.objects.remove(object_index);
    create(&mut candidate, request)?;
    let object = candidate.objects.pop().unwrap();
    let asset = candidate.cloths.pop().unwrap();
    candidate.objects.insert(object_index, object);
    candidate.cloths.insert(asset_index, asset);
    candidate.compile(&Pass::Beauty)?;
    *document = candidate;
    Ok(())
}
