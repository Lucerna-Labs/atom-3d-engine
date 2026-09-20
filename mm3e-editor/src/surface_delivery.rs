//! Evaluated native-surface feature extraction. Geometry stays the authored
//! spherical-offset field; native scalar checks and rounded-feature refinement
//! remain mandatory after the cell-local affine arrangement.
use crate::{model::array, surface_condition, surface_refine, surface_retessellate};
use mm3e_kit::{
    meshing::Mesh,
    meshing_local::{
        extract_local_convex_union_isosurface_with_representation, ConvexRepresentationPolicy, LocalExtractionOptions,
    },
    surface_features::SurfaceFeatures,
    Vec3,
};
use mm3e_orchestrator::{Prim, Scene};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

pub(crate) fn eligible(scene: &Scene) -> bool {
    scene.objects.len() == 1 && matches!(scene.objects[0].prim, Prim::Surface { .. }) && {
        let m = &scene.objects[0].mods;
        !m.mirror.iter().any(|&v| v)
            && m.elongate == Vec3::ZERO
            && m.repeat == Vec3::ZERO
            && m.twist == 0.0
            && m.bend == 0.0
            && m.onion == 0.0
    }
}
pub(crate) struct Extraction {
    pub mesh: Mesh,
    pub report: Value,
    pub work: u64,
}
pub(crate) struct Options {
    pub min: Vec3,
    pub max: Vec3,
    pub resolution: [u32; 3],
    pub max_work: usize,
    pub max_vertices: usize,
    pub max_triangles: usize,
    pub max_field_residual: f32,
}
pub(crate) fn extract(scene: &Scene, options: Options) -> Result<Extraction, String> {
    if !eligible(scene) {
        return Err("source feature extraction requires one affine native surface without onion mapping".into());
    }
    let object = &scene.objects[0];
    let Prim::Surface { id } = object.prim else { unreachable!() };
    let surface = &scene.surfaces[id as usize];
    let radius = f64::from(surface.half_thickness()) + f64::from(object.mods.round) / f64::from(object.xform.scale);
    let world_radius = radius * f64::from(object.xform.scale);
    if world_radius <= 0. || !world_radius.is_finite() {
        return Err("rounded native surface has no positive representable shell radius".into());
    }
    let requested = options.max_field_residual.min((world_radius * 0.005) as f32);
    if requested <= 0. {
        return Err("rounded-feature residual cannot be represented".into());
    }
    let features = SurfaceFeatures::with_supporting_planes(surface.clone(), radius, options.max_work)?;
    let constructor_work = features.construction_work;
    let identity = object.xform.pos == Vec3::ZERO
        && object.xform.scale == 1.0
        && object.xform.rot.cols == mm3e_kit::Mat3::IDENTITY.cols;
    let max_representation_error_m = (world_radius * 0.0005).min(f64::from(requested) * 0.1);
    let extracted = extract_local_convex_union_isosurface_with_representation(
        options.min,
        options.max,
        options.resolution,
        LocalExtractionOptions {
            max_work: options.max_work.saturating_sub(constructor_work),
            max_vertices: options.max_vertices,
            max_triangles: options.max_triangles,
        },
        ConvexRepresentationPolicy { max_representation_error_m },
        |cell, remaining| {
            if identity {
                return features.sample_axis_aligned_cell(cell.points[0], cell.points[7], remaining);
            }
            const DOMAIN_WORK: usize = 16;
            if remaining < DOMAIN_WORK {
                return Err("surface cell domain mapping exhausted work".into());
            }
            let points = cell.points.map(|p| scene.surface_texture_local_point(0, p));
            let mut local = [Vec3::ZERO; 8];
            for (i, p) in points.into_iter().enumerate() {
                local[i] = p?;
            }
            let mut data = features.sample_cell(local, remaining - DOMAIN_WORK)?;
            data.work += DOMAIN_WORK;
            Ok(data)
        },
    )?;
    let extraction_work = extracted.report.charged_work();
    let representation =
        extracted.report.representation.as_ref().ok_or("surface extraction omitted representation certification")?;
    let diagnostic_work = representation.contractions.len() + representation.rounding_searches.len();
    if constructor_work + extraction_work + diagnostic_work > options.max_work {
        return Err("surface representation diagnostics exceed aggregate work".into());
    }
    let contractions: Vec<_> = representation
        .contractions
        .iter()
        .map(|entry| {
            json!({
                "removed_vertex":entry.removed_vertex,"retained_vertex":entry.retained_vertex,
                "max_displacement_m":entry.max_displacement_m,"source_check_work":entry.source_check_work,
                "affected_triangles":entry.affected_triangles,
            })
        })
        .collect();
    let rounding_searches: Vec<_> = representation.rounding_searches.iter().map(|search| json!({
        "status":search.status,"work":search.work,"variables":search.variables,"participants":search.participants,
        "constraints":search.constraints,"allowed_tuples":search.allowed_tuples,"decisions":search.decisions,
        "stable_faces":search.stable_faces,"component_limit":search.component_limit,
    })).collect();
    let certificate = &representation.correspondence;
    let embedding = &representation.embedding;
    let representation_report = json!({
        "policy":"bounded_static_representation","max_representation_error_m":representation.max_representation_error_m,
        "contractions":contractions,"rejected_proposals":representation.rejected_proposals,
        "source_check_work":representation.source_check_work,"adjacent_sweeps_certified":representation.adjacent_sweeps_certified,
        "embedding_validation_work":representation.embedding_validation_work,
        "embedding_certificate_work":representation.embedding_certificate_work,
        "embedding_certificate_binding_work":representation.embedding_certificate_binding_work,
        "embedding_repair_attempts":representation.embedding_repair_attempts,
        "embedding_initial_contacts":representation.embedding_initial_contacts,
        "rounding_searches":rounding_searches,
        "correspondence":{"work":certificate.work,"max_displacement_m":certificate.max_displacement_m,
            "worst_vertex":certificate.worst_vertex,"vertex_witnesses":certificate.vertex_witnesses,
            "original_triangles":certificate.original_triangles,"final_triangles":certificate.final_triangles,
            "triangle_images":certificate.triangle_images,"edge_images":certificate.edge_images,"vertex_images":certificate.vertex_images},
        "embedding_validation":{"intersection_free":true,"work":embedding.work,"candidate_pairs":embedding.candidate_pairs,
            "reused_pairs":embedding.reused_pairs,
            "predicate_tests":embedding.predicate_tests,"exact_predicates":embedding.exact_predicates,"bvh_chunks":embedding.bvh_chunks},
        "scope":"Source contraction admission and direct cumulative source-to-stored displacement, followed by complete stored-f32 intersection validation before native refinement. Adjacent swept trajectories and continuous animation are not certified by this static representation step.",
    });
    let local_report = json!({
        "cell_queries":extracted.report.cell_queries,
        "callback_work":extracted.report.callback_work,
        "arrangement_work":extracted.report.arrangement_work,
        "feature_node_values":extracted.report.feature_node_values,
        "peak_local_channels":extracted.report.peak_local_channels,
        "peak_shared_values":extracted.report.peak_shared_values,
        "shared_value_checks":extracted.report.shared_value_checks,
        "global_features":extracted.report.global_features,
        "proven_outside_cells":extracted.report.proven_outside_cells,
        "exact_support_reductions":extracted.report.exact_support_reductions,
        "convex_aliases_merged":extracted.report.convex_aliases_merged,
        "convex_edges_split":extracted.report.convex_edges_split,
        "convex_max_rounding_edge_deviation_m":extracted.report.convex_max_rounding_edge_deviation_m,
        "convex_rounded_vertices":extracted.report.convex_rounded_vertices,
        "convex_max_source_rounding_displacement_m":extracted.report.convex_max_source_rounding_displacement_m,
        "convex_source_collinear_faces":extracted.report.convex_source_collinear_faces,
        "convex_exact_pool_corrections":extracted.report.convex_exact_pool_corrections,
        "convex_exact_pool_binary_steps":extracted.report.convex_exact_pool_binary_steps,
        "convex_merged_polygons":extracted.report.convex_merged_polygons,
        "convex_coalesced_interior_vertices":extracted.report.convex_coalesced_interior_vertices,
        "convex_redundant_boundary_vertices":extracted.report.convex_redundant_boundary_vertices,
        "representation":representation_report,
        "identity_domain":identity,
    });
    let before_vertices = extracted.mesh.positions.len();
    let before_triangles = extracted.mesh.triangles.len();
    let producer_error = certificate.max_displacement_m;
    if producer_error > max_representation_error_m {
        return Err("surface producer exceeded its representation allowance".into());
    }
    let remaining_error = (max_representation_error_m - producer_error).next_down().max(0.);
    let mut prefix_work = constructor_work + extraction_work + diagnostic_work;
    let conditioned = surface_condition::condition_certified(
        extracted.mesh,
        surface_condition::Options {
            max_error_m: remaining_error,
            max_work: options.max_work.saturating_sub(prefix_work),
        },
        Some(&representation.embedding_certificate),
    )
    .map_err(|error| format!("surface preprojection conditioning: {error}"))?;
    prefix_work += conditioned.report.work;
    let conditioning_error = conditioned.correspondence.max_displacement_m;
    let composed_error = if conditioning_error == 0. {
        producer_error
    } else if producer_error == 0. {
        conditioning_error
    } else {
        (producer_error + conditioning_error).next_up()
    };
    if composed_error > max_representation_error_m {
        return Err("composed surface representation and conditioning exceed the original allowance".into());
    }
    // Labels continue to refer to the original extracted faces. Exact flips
    // preserve the complete union of these labels rather than choosing one.
    let ancestry_work = conditioned.face_source_indices.len();
    if ancestry_work > options.max_work.saturating_sub(prefix_work) {
        return Err("surface conditioning ancestry exceeds aggregate work".into());
    }
    prefix_work += ancestry_work;
    let face_ancestors = conditioned.face_source_indices.into_iter().map(|index| vec![index]).collect();
    let retessellated = surface_retessellate::retessellate_certified(
        conditioned.mesh,
        face_ancestors,
        surface_retessellate::Options {
            max_work: options.max_work.saturating_sub(prefix_work),
            max_passes: 8,
            max_candidates: surface_retessellate::MAX_CANDIDATES,
        },
        conditioned.embedding_certificate.as_ref(),
    )
    .map_err(|error| format!("surface coplanar retessellation: {error}"))?;
    prefix_work += retessellated.report.work;
    if retessellated.report.geometric_displacement_m != 0. {
        return Err("surface retessellation did not certify exact patch preservation".into());
    }
    let warped_allowance = (max_representation_error_m - composed_error).next_down().max(0.);
    let warped = surface_retessellate::retessellate_bounded_certified(
        retessellated.mesh,
        retessellated.face_ancestors,
        surface_retessellate::Options {
            max_work: options.max_work.saturating_sub(prefix_work),
            max_passes: 8,
            max_candidates: surface_retessellate::MAX_CANDIDATES,
        },
        warped_allowance,
        retessellated.embedding_certificate.as_ref(),
    )
    .map_err(|error| format!("surface bounded retessellation: {error}"))?;
    prefix_work += warped.report.work;
    let warped_error = warped.report.max_surface_error_m;
    let composed_error = if warped_error == 0. {
        composed_error
    } else if composed_error == 0. {
        warped_error
    } else {
        (composed_error + warped_error).next_up()
    };
    if !composed_error.is_finite() || composed_error > max_representation_error_m {
        return Err("composed surface preprocessing exceeds the original representation allowance".into());
    }
    let refined = surface_refine::refine(
        scene,
        warped.mesh,
        surface_refine::Options {
            max_residual: requested,
            normal_step_m: world_radius * 0.05,
            max_work: options.max_work.saturating_sub(prefix_work),
            max_vertices: options.max_vertices,
            max_triangles: options.max_triangles,
            max_passes: 6,
        },
    ).map_err(|error| format!(
        "{error}; preprocessing work {prefix_work}/{} (construction {constructor_work}, extraction {extraction_work}, conditioning {}, exact retessellation {}, bounded retessellation {})",
        options.max_work, conditioned.report.work, retessellated.report.work, warped.report.work,
    ))?;
    let refinement_work = refined.report["charged_work"].as_u64().ok_or("surface refinement omitted charged work")?;
    if refined.face_source_indices.len() != refined.mesh.triangles.len() {
        return Err("surface refinement lost face provenance".into());
    }
    let mut provenance_work = 0u64;
    for &parent in &refined.face_source_indices {
        let ancestors = warped.face_ancestors.get(parent).ok_or("surface refinement returned invalid parent face")?;
        provenance_work += ancestors.len() as u64 + 1;
    }
    let work = prefix_work as u64 + refinement_work + provenance_work;
    if work > options.max_work as u64 {
        return Err("surface feature extraction exceeded aggregate work".into());
    }
    let mut provenance = Sha256::new();
    provenance.update(b"MM3E producing-feature ancestry v1\0");
    let mut single_feature_provenance = Some(Sha256::new());
    let mut max_feature_ancestors = 0;
    for &parent in &refined.face_source_indices {
        let ancestors = &warped.face_ancestors[parent];
        let mut ids = BTreeSet::new();
        for &ancestor in ancestors {
            ids.insert(
                *extracted
                    .face_feature_ids
                    .get(ancestor)
                    .ok_or("surface retessellation returned invalid original face")?,
            );
        }
        if ids.is_empty() {
            return Err("surface retessellation lost feature ancestry".into());
        }
        max_feature_ancestors = max_feature_ancestors.max(ids.len());
        provenance.update((ids.len() as u64).to_le_bytes());
        for id in &ids {
            provenance.update(id.to_le_bytes());
        }
        if ids.len() == 1 {
            if let Some(hash) = &mut single_feature_provenance {
                hash.update(ids.first().unwrap().to_le_bytes());
            }
        } else {
            single_feature_provenance = None;
        }
    }
    let provenance_sha256 = format!("{:x}", provenance.finalize());
    let single_feature_sha256 = single_feature_provenance.map(|hash| format!("{:x}", hash.finalize()));
    Ok(Extraction {
        mesh: refined.mesh,
        work,
        report: json!({"method":"local_prisms_and_capsule_supporting_planes","source_triangles":surface.triangles().len(),
        "global_features":features.feature_count(),"local_radius":radius,"world_radius_m":world_radius,"constructor_work":constructor_work,
        "local_extraction":local_report,"affine_vertices":before_vertices,"affine_triangles":before_triangles,"refinement":refined.report,
        "conditioning":conditioned.report,"retessellation":retessellated.report,"bounded_retessellation":warped.report,
        "conditioning_ancestry_work":ancestry_work,
        "composed_representation_error_m":composed_error,
        "producing_feature_ids_sha256":single_feature_sha256,"producing_feature_ancestry_sha256":provenance_sha256,
        "max_producing_feature_ancestors":max_feature_ancestors,"provenance_work":provenance_work,"diagnostic_work":diagnostic_work,
        "feature_identity_scope":"Within this evaluated sample; complete transitive feature sets are length-framed in producing_feature_ancestry_sha256. The single-feature hash is present only when every face has one feature ancestor. Contracted or refined faces need not lie on an original feature plane.",
        "charged_work":work,"bounds_min":array(options.min),"bounds_max":array(options.max),"resolution":options.resolution,
        "native_residual_target":requested,"max_representation_error_m":max_representation_error_m,"capsule_support_planes":14,
        "semantics":"The initial triangle-prism and14-plane capsule program encloses the native source. Bounded static representation correction and native projection are separately certified; independent rounded-feature and texture validation remain mandatory. Program scalars are not Euclidean distances."}),
    })
}
