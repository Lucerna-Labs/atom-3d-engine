use crate::{animation::AnimationSample, model::*, protocol::Failure, storage};
use mm3e_kit::color::Rgba;
use mm3e_orchestrator::Scene;
use serde_json::{json, Value};
use std::{collections::BTreeMap, path::Path, time::Instant};

pub fn owner(document: &Document, mat: u32) -> Option<&str> {
    mat.checked_sub(1).and_then(|i| document.objects.get(i as usize)).map(|e| e.id.as_str())
}

pub fn entity(e: &Entity) -> Value {
    let shape = match &e.shape {
        Shape::Surface { vertices, triangles, thickness_m } => json!({"type":"surface","vertex_count":vertices.len(),
            "triangle_count":triangles.len(),"thickness_m":thickness_m,"topology_omitted":true}),
        Shape::Volume { dims, min, cell, samples } => json!({"type": "volume", "dims": dims, "min": min,
            "cell": cell, "sample_count": samples.len(), "samples_omitted": true}),
        shape => serde_json::to_value(shape).expect("validated shape is JSON representable"),
    };
    json!({"id": e.id, "label": e.label, "role": e.role, "group": e.group, "shape": shape,
        "position": e.position, "rotation_degrees": e.rotation_degrees, "scale": e.scale,
        "material": e.material, "combine": e.combine, "modifiers": e.modifiers})
}

pub fn inspection(document: &Document) -> Value {
    let objects: Vec<Value> = document.objects.iter().map(entity).collect();
    let cloths:Vec<_>=document.cloths.iter().map(|cloth|json!({"id":cloth.id,"vertex_count":cloth.rest_vertices.len(),
        "triangle_count":cloth.triangles.len(),"pins":cloth.pins,"collision_object_ids":cloth.collision_object_ids,
        "settings":cloth.settings,"cache":cloth.cache.as_ref().map(|cache|json!({"clip":cache.clip,"duration":cache.duration,
            "frame_count":cache.frames.len(),"source_fingerprint":cache.source_fnv1a64,"cache_vertices_omitted":true}))})).collect();
    let deformers:Vec<_>=document.deformers.iter().map(|asset|json!({"id":asset.id,"object":asset.object,"method":asset.method,
        "joints":asset.joints,"influence_rows":asset.weights.len(),"morphs":asset.blendshapes.iter().map(|shape|json!({"id":shape.id,"weight":shape.weight,
            "min_weight":shape.min_weight,"max_weight":shape.max_weight,"delta_count":shape.deltas.len()})).collect::<Vec<_>>(),
        "dense_deformation_data_omitted":true})).collect();
    json!({"version": document.version, "units": document.units, "axes": "right-handed; Y up; character faces +Z",
        "objects": objects, "camera": document.camera, "settings": document.settings, "lights": document.lights,
        "joints": document.joints, "clips": document.clips, "garments": document.garments, "faces":document.faces,"cloths":cloths,"deformers":deformers,
        "audio":document.audio.iter().map(crate::audio::metadata).collect::<Vec<_>>(),"shots":document.shots,
        "textures":document.textures.iter().map(crate::textures::metadata).collect::<Vec<_>>(),
        "uv_sets":document.uv_sets.iter().map(|set|json!({"id":set.id,"object":set.object,"vertex_count":set.vertex_count,"triangles_sha256":set.triangles_sha256,"uv_values":set.values.len(),"uv_triangles":set.corner_indices.len()})).collect::<Vec<_>>(),"texture_bindings":document.texture_bindings})
}

fn camera(evaluated: mm3e_kit::camera::Camera, view: Option<&View>) -> Result<mm3e_kit::camera::Camera, Failure> {
    if let Some(v) = view {
        v.validate().map_err(Failure::invalid)?;
        Ok(v.compile())
    } else {
        Ok(evaluated)
    }
}

pub fn pick(
    document: &Document,
    x: f32,
    y: f32,
    view: Option<&View>,
    animation: Option<&AnimationSample>,
) -> Result<Value, Failure> {
    let (scene, evaluated_camera) = document.compile_at(&Pass::Beauty, animation).map_err(Failure::invalid)?;
    if !x.is_finite() || !y.is_finite() || x < 0.0 || y < 0.0 || x >= scene.width as f32 || y >= scene.height as f32 {
        return Err(Failure::invalid("pick coordinates must lie inside the configured image"));
    }
    let camera = camera(evaluated_camera, view)?;
    let ray = camera.ray(x, y, scene.width, scene.height);
    let hit = scene.marcher.march(&scene.field(), &ray);
    if hit.hit {
        Ok(
            json!({"status": "hit", "animation": animation, "material_owner_id": owner(document, hit.mat), "position": array(hit.pos),
            "normal": array(hit.normal), "ray_t": hit.t, "steps": hit.steps,
            "identity_semantics": "material owner; subtraction keeps the base material; blends use the dominant material"}),
        )
    } else {
        Ok(json!({"status": "no_hit_or_budget_exhausted", "animation": animation, "steps": hit.steps}))
    }
}

fn metrics(document: &Document, scene: &Scene, camera: &mm3e_kit::camera::Camera) -> Value {
    let field = scene.field();
    let mut hits = 0u64;
    let mut steps = 0u64;
    let mut bounds = [scene.width, scene.height, 0, 0];
    let mut by_owner = BTreeMap::<String, u64>::new();
    for y in 0..scene.height {
        for x in 0..scene.width {
            let ray = camera.ray(x as f32 + 0.5, y as f32 + 0.5, scene.width, scene.height);
            let hit = scene.marcher.march_with(&field, |_| mm3e_kit::vec::Vec3::ZERO, &ray);
            steps += u64::from(hit.steps);
            if hit.hit {
                hits += 1;
                bounds[0] = bounds[0].min(x);
                bounds[1] = bounds[1].min(y);
                bounds[2] = bounds[2].max(x);
                bounds[3] = bounds[3].max(y);
                if let Some(id) = owner(document, hit.mat) {
                    *by_owner.entry(id.into()).or_default() += 1;
                }
            }
        }
    }
    let pixels = u64::from(scene.width) * u64::from(scene.height);
    json!({"primary_center_ray_hits": hits, "pixel_count": pixels, "coverage": hits as f64 / pixels as f64,
        "hit_bounds_xyxy_inclusive": if hits > 0 { Some(bounds) } else { None },
        "mean_reported_march_steps": steps as f64 / pixels as f64, "visible_material_owner_pixels": by_owner,
        "scope": "single center ray per pixel; geometric visibility diagnostics, not anatomy, topology or aesthetic quality"})
}

pub fn image(
    document: &Document,
    root: &Path,
    path: &str,
    pass: &Pass,
    view: Option<&View>,
    overwrite: bool,
    animation: Option<&AnimationSample>,
) -> Result<Value, Failure> {
    let started = Instant::now();
    let target = storage::path(root, path, false)?;
    let extension = target.extension().and_then(|e| e.to_str()).unwrap_or("");
    if extension != "png" && extension != "bmp" && extension != "exr" {
        return Err(Failure::invalid("render path must end in .png, .bmp or .exr"));
    }
    if extension == "exr" && !matches!(pass, Pass::Beauty) {
        return Err(Failure::invalid(
            "EXR currently exports scene-linear beauty RGB; display-mapped AOVs are not linear data passes",
        ));
    }
    if !overwrite && target.exists() {
        return Err(Failure::invalid("output exists; set overwrite to true to replace it"));
    }
    if extension == "bmp" && document.settings.film.transparent_background {
        return Err(Failure::invalid("BMP cannot retain coverage alpha; choose PNG or EXR"));
    }
    crate::shot::preflight(document, pass, view, animation).map_err(Failure::invalid)?;
    let (scene, evaluated_camera) = document.compile_at(pass, animation).map_err(Failure::invalid)?;
    let camera = camera(evaluated_camera, view)?;
    let mut display_fingerprint = None;
    let mut linear_fingerprint = None;
    let mut film_observation = None;
    let mut channels = None;
    let render_ms;
    let mut bytes = vec![];
    if document.settings.film.active() {
        let exposure = crate::shot::render(document, view, animation).map_err(Failure::invalid)?;
        render_ms = started.elapsed().as_secs_f64() * 1000.0;
        film_observation = Some(crate::shot::summary(&exposure, &document.settings.film));
        let linear = if document.settings.film.transparent_background {
            &exposure.frame.foreground
        } else {
            &exposure.frame.beauty
        };
        let mut hash = 0xcbf29ce484222325u64;
        for pixel in linear {
            for value in array(*pixel) {
                for byte in value.to_le_bytes() {
                    hash = (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3);
                }
            }
        }
        linear_fingerprint = Some(format!("{hash:016x}"));
        if extension == "exr" {
            if document.settings.film.exr_data_channels {
                bytes = crate::film::encode_compositing_exr_with_metadata(
                    &exposure.frame,
                    document.settings.film.transparent_background,
                    &exposure.metadata,
                )
                .map_err(Failure::invalid)?;
                channels = Some(json!(["R", "G", "B", "A", "Z", "N.X", "N.Y", "N.Z", "material.ID"]));
            } else {
                bytes = crate::film::encode_rgba_exr_with_metadata(
                    &exposure.frame,
                    document.settings.film.transparent_background,
                    &exposure.metadata,
                )
                .map_err(Failure::invalid)?;
                channels = Some(json!(["R", "G", "B", "A"]));
            }
        } else {
            let rgba = crate::shot::display_rgba(
                &exposure.frame,
                document.settings.film.transparent_background,
                document.settings.exposure,
            );
            display_fingerprint = Some(format!("{:016x}", mm3e_kit::atoms::hash(&rgba)));
            if extension == "png" {
                let mut encoder = png::Encoder::new(&mut bytes, scene.width, scene.height);
                encoder.set_color(png::ColorType::Rgba);
                encoder.set_depth(png::BitDepth::Eight);
                let mut writer = encoder.write_header().map_err(|e| Failure::io(e.to_string()))?;
                writer.write_image_data(&rgba).map_err(|e| Failure::io(e.to_string()))?;
                writer.finish().map_err(|e| Failure::io(e.to_string()))?;
            } else {
                let framebuffer =
                    mm3e_orchestrator::post::resolve(&exposure.frame.beauty, scene.width, scene.height, &scene.post);
                bytes = framebuffer.to_bmp(Rgba::rgb8(0, 0, 0));
            }
        }
    } else if extension == "exr" {
        let linear = mm3e_orchestrator::render_linear_checked(&scene, &camera).map_err(Failure::invalid)?;
        render_ms = started.elapsed().as_secs_f64() * 1000.0;
        let mut hash = 0xcbf29ce484222325u64;
        for pixel in &linear {
            for value in array(*pixel) {
                for byte in value.to_le_bytes() {
                    hash = (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3);
                }
            }
        }
        linear_fingerprint = Some(format!("{hash:016x}"));
        bytes = crate::film::encode_exr(scene.width, scene.height, &linear).map_err(Failure::invalid)?;
    } else {
        let framebuffer = mm3e_orchestrator::render_checked(&scene, &camera).map_err(Failure::invalid)?;
        let rgba = framebuffer.to_rgba8(Rgba::rgb8(0, 0, 0));
        render_ms = started.elapsed().as_secs_f64() * 1000.0;
        display_fingerprint = Some(format!("{:016x}", mm3e_kit::atoms::hash(&rgba)));
        if extension == "png" {
            let mut encoder = png::Encoder::new(&mut bytes, scene.width, scene.height);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().map_err(|e| Failure::io(e.to_string()))?;
            writer.write_image_data(&rgba).map_err(|e| Failure::io(e.to_string()))?;
            writer.finish().map_err(|e| Failure::io(e.to_string()))?;
        } else {
            bytes = framebuffer.to_bmp(Rgba::rgb8(0, 0, 0));
        }
    }
    let observations = metrics(document, &scene, &camera);
    let path = storage::write(root, path, &bytes, overwrite)?;
    Ok(json!({"path": path, "width": scene.width, "height": scene.height, "pass": pass,
        "view": {"eye":array(camera.eye),"target":array(camera.eye + camera.forward),"up":array(camera.up),"fov_degrees":(camera.fov_scale.atan()*2.0).to_degrees()},
        "animation": animation, "quality": document.settings.quality,
        "backend": "mm3e_orchestrator_cpu", "render_ms": render_ms,
        "total_ms": started.elapsed().as_secs_f64() * 1000.0,
        "rgba_fnv1a64": display_fingerprint, "linear_rgb_fnv1a64": linear_fingerprint,
        "film":film_observation,"channels":channels,
        "color_encoding":if extension=="exr" { if document.settings.film.exr_data_channels {"scene_linear_rec709_f32_rgba_data"}else if document.settings.film.active(){"scene_linear_rec709_f32_rgba"}else{"scene_linear_rec709_f32_rgb"} } else { "display_aces_fit_gamma_rgba8" },
        "metrics": observations}))
}
