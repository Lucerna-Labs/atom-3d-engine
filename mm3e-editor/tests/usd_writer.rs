use mm3e_editor::usd::*;

fn fixture() -> UsdAsset {
    let first = FrameMesh {
        time: 10.0,
        positions: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
        triangles: vec![[0, 1, 2]],
        material_ids: vec![0],
        corner_uvs: None,
    };
    let second = FrameMesh {
        time: 11.0,
        positions: vec![[0.0, 0.0, 0.5], [2.0, 0.0, 0.5], [0.0, 2.0, 0.5], [2.0, 2.0, 0.5]],
        triangles: vec![[0, 1, 2], [1, 3, 2]],
        material_ids: vec![1, 0],
        corner_uvs: None,
    };
    let mut retained_second = first.clone();
    retained_second.time = 11.0;
    retained_second.positions[2][2] = 0.5;
    UsdAsset {
        frames_per_second: 24.0,
        start_time_code: 10.0,
        end_time_code: 11.0,
        meshes: vec![
            UsdMesh {
                id: "a-b".into(),
                source_entity_ids: vec!["head\" / 😀\n\\body".into(), "body".into()],
                representation: UsdRepresentation::ComposedSdfBake,
                double_sided: false,
                frames: vec![first.clone(), second],
            },
            UsdMesh {
                id: "a_b".into(),
                source_entity_ids: vec!["cape".into()],
                representation: UsdRepresentation::NativeSurface,
                double_sided: true,
                frames: vec![first, retained_second],
            },
        ],
        materials: vec![
            UsdMaterial {
                id: "warm/red".into(),
                diffuse_color: [0.25, 0.02, 0.01],
                roughness: 0.37,
                metallic: 0.0,
                ior: 1.5,
                emissive_color: [0.0; 3],
                textures: vec![],
            },
            UsdMaterial {
                id: "metal\"bright".into(),
                diffuse_color: [0.8, 0.7, 0.6],
                roughness: 0.2,
                metallic: 1.0,
                ior: 1.8,
                emissive_color: [2.5, 0.0, 0.0],
                textures: vec![],
            },
        ],
        camera: Some(UsdCamera {
            id: "shot/camera".into(),
            frames: vec![
                CameraFrame {
                    time: 10.0,
                    position: [0.0, 0.0, 5.0],
                    target: [0.0; 3],
                    up: [0.0, 1.0, 0.0],
                    vertical_fov_degrees: 50.0,
                    aspect_ratio: 1.5,
                    near_clip: 0.1,
                    far_clip: 1000.0,
                    aperture_radius_m: 0.006,
                    focus_distance_m: 5.0,
                    shutter_open_seconds: -0.01,
                    shutter_close_seconds: 0.01,
                },
                CameraFrame {
                    time: 11.0,
                    position: [3.0, 2.0, 4.0],
                    target: [0.0, 1.0, 0.0],
                    up: [0.0, 1.0, 0.0],
                    vertical_fov_degrees: 65.0,
                    aspect_ratio: 2.0,
                    near_clip: 0.2,
                    far_clip: 500.0,
                    aperture_radius_m: 0.0,
                    focus_distance_m: 4.0,
                    shutter_open_seconds: 0.0,
                    shutter_close_seconds: 0.02,
                },
            ],
        }),
    }
}

fn textured_fixture() -> UsdAsset {
    let mut asset = fixture();
    asset.materials[0].textures = [
        (UsdTextureRole::Diffuse, UsdTextureChannel::Rgb, UsdTextureColorSpace::Srgb),
        (UsdTextureRole::Emissive, UsdTextureChannel::Rgb, UsdTextureColorSpace::Raw),
        (UsdTextureRole::Roughness, UsdTextureChannel::G, UsdTextureColorSpace::Raw),
        (UsdTextureRole::Metallic, UsdTextureChannel::A, UsdTextureColorSpace::Raw),
        (UsdTextureRole::Normal, UsdTextureChannel::Rgb, UsdTextureColorSpace::Raw),
    ]
    .into_iter()
    .map(|(role, channel, source_color_space)| UsdTexture {
        role,
        channel,
        source_color_space,
        asset_path: "textures/packed.png".into(),
        wrap_s: UsdTextureWrap::Mirror,
        wrap_t: UsdTextureWrap::Clamp,
        scale: if role == UsdTextureRole::Normal { [2.0, 2.0, 2.0, 1.0] } else { [0.8, 0.75, 0.7, 0.6] },
        bias: if role == UsdTextureRole::Normal { [-1.0, -1.0, -1.0, 0.0] } else { [0.01, 0.02, 0.03, 0.04] },
    })
    .collect();
    for mesh in &mut asset.meshes {
        for (sample, frame) in mesh.frames.iter_mut().enumerate() {
            frame.corner_uvs = Some(
                frame
                    .triangles
                    .iter()
                    .enumerate()
                    .map(|(face, _)| {
                        let offset = face as f32 * 2.0 + sample as f32 * 0.125;
                        [[-0.25 + offset, 0.1], [1.25 + offset, 0.25], [0.25 + offset, 1.1]]
                    })
                    .collect(),
            );
        }
    }
    asset
}

#[test]
fn usd_writer_emits_typed_texture_graphs_and_face_varying_uv_samples() {
    let asset = textured_fixture();
    let text = String::from_utf8(encode_usda(&asset).unwrap()).unwrap();
    assert!(text.contains("texCoord2f[] primvars:st.timeSamples"));
    assert!(text.contains("interpolation = \"faceVarying\""));
    assert!(text.contains("float inputs:roughness.connect"));
    assert!(text.contains("roughnessTexture.outputs:g"));
    assert!(text.contains("metallicTexture.outputs:a"));
    assert!(text.contains("normal3f inputs:normal.connect"));
    assert!(text.contains("float3 outputs:rgb"));
    assert!(text.contains("token inputs:sourceColorSpace = \"sRGB\""));
    assert!(text.contains("token inputs:sourceColorSpace = \"raw\""));
    assert!(text.contains("token inputs:wrapS = \"mirror\""));
    assert!(text.contains("token inputs:wrapT = \"clamp\""));
    assert!(!text.contains("opacity.connect"));
    let plain = String::from_utf8(encode_usda(&fixture()).unwrap()).unwrap();
    assert!(!plain.contains("primvars:st"));
    assert!(!plain.contains("UsdUVTexture"));
}

#[test]
fn usd_writer_rejects_missing_uvs_injection_paths_and_incompatible_map_types() {
    type Mutation = Box<dyn Fn(&mut UsdAsset)>;
    let mut cases: Vec<Mutation> = vec![
        Box::new(|a| a.meshes[0].frames[0].corner_uvs = None),
        Box::new(|a| a.meshes[0].frames[1].corner_uvs.as_mut().unwrap().pop().map(|_| ()).unwrap()),
        Box::new(|a| a.meshes[0].frames[0].corner_uvs.as_mut().unwrap()[0][1][0] = f32::NAN),
        Box::new(|a| {
            let duplicate = a.materials[0].textures[0].clone();
            a.materials[0].textures.push(duplicate);
        }),
        Box::new(|a| a.materials[0].textures[0].channel = UsdTextureChannel::R),
        Box::new(|a| a.materials[0].textures[2].channel = UsdTextureChannel::Rgb),
        Box::new(|a| a.materials[0].textures[2].source_color_space = UsdTextureColorSpace::Srgb),
        Box::new(|a| a.materials[0].textures[4].source_color_space = UsdTextureColorSpace::Srgb),
        Box::new(|a| a.materials[0].textures[0].scale[0] = f32::INFINITY),
        Box::new(|a| a.materials[0].textures[0].bias[3] = f32::NAN),
    ];
    for path in [
        "",
        "/tmp/texture.png",
        "../texture.png",
        "textures/../image.png",
        "textures/./image.png",
        "textures//image.png",
        "C:/image.png",
        "textures\\image.png",
        "image.png@\n def Shader \"bad\"",
        "https://host/image.png",
        "textures/",
    ] {
        cases.push(Box::new(move |asset| asset.materials[0].textures[0].asset_path = path.into()));
    }
    for (index, mutate) in cases.into_iter().enumerate() {
        let mut asset = textured_fixture();
        mutate(&mut asset);
        assert!(encode_usda(&asset).is_err(), "invalid texture case {index} accepted");
    }
}

#[test]
fn uv_sampling_follows_uv_changes_and_topology_without_inventing_missing_coordinates() {
    let mut asset = textured_fixture();
    asset.meshes.remove(0);
    asset.camera = None;
    asset.meshes[0].frames[1].corner_uvs = asset.meshes[0].frames[0].corner_uvs.clone();
    let static_uv = String::from_utf8(encode_usda(&asset).unwrap()).unwrap();
    assert!(!static_uv.contains("primvars:st.timeSamples"));
    asset.meshes[0].frames[1].corner_uvs.as_mut().unwrap()[0][0][0] += 0.125;
    let moving_uv = String::from_utf8(encode_usda(&asset).unwrap()).unwrap();
    assert!(moving_uv.contains("primvars:st.timeSamples"));
    // An untextured sample may lack UVs. Author a value block rather than
    // fabricating coordinates or leaking another sample's UV array into it.
    asset.meshes[0].frames[1].corner_uvs = None;
    asset.meshes[0].frames[1].material_ids = vec![1];
    let absent = String::from_utf8(encode_usda(&asset).unwrap()).unwrap();
    assert!(absent.contains("11: None"));
}

#[test]
fn usd_writer_handles_topology_material_camera_animation_and_stable_ids() {
    let asset = fixture();
    let bytes = encode_usda(&asset).unwrap();
    assert_eq!(bytes, encode_usda(&asset).unwrap());
    let output = String::from_utf8(bytes).unwrap();
    assert!(output.starts_with("#usda 1.0"));
    assert!(output.contains("metersPerUnit = 1"));
    assert!(output.contains("timeCodesPerSecond = 24"));
    assert!(output.contains("faceVertexIndices.timeSamples"));
    assert!(output.contains("indices.timeSamples"));
    assert!(output.contains("xformOp:transform.timeSamples"));
    assert_ne!(prim_name("a-b"), prim_name("a_b"));
    assert!(output.contains(&format!("def Mesh \"{}\"", prim_name("a-b"))));
    assert!(output.contains("evaluatedPolygonMeshCache"));
}

#[test]
fn usd_writer_rejects_invalid_payloads_before_returning_any_bytes() {
    type InvalidMutation = Box<dyn Fn(&mut UsdAsset)>;
    let cases: Vec<InvalidMutation> = vec![
        Box::new(|a| a.frames_per_second = 0.0),
        Box::new(|a| a.frames_per_second = f64::NAN),
        Box::new(|a| a.start_time_code = f64::NEG_INFINITY),
        Box::new(|a| a.end_time_code = 9.0),
        Box::new(|a| a.meshes.clear()),
        Box::new(|a| a.materials.clear()),
        Box::new(|a| a.meshes[0].frames.clear()),
        Box::new(|a| a.meshes[0].frames[1].time = 10.0),
        Box::new(|a| a.meshes[1].frames[1].time = 12.0),
        Box::new(|a| a.meshes[0].frames[0].positions[0][0] = f32::NAN),
        Box::new(|a| a.meshes[0].frames[0].triangles[0][2] = 9),
        Box::new(|a| a.meshes[0].frames[0].triangles[0][2] = 0),
        Box::new(|a| a.meshes[0].frames[0].positions[2] = [2.0, 0.0, 0.0]),
        Box::new(|a| a.meshes[0].frames[0].material_ids.clear()),
        Box::new(|a| a.meshes[0].frames[0].material_ids[0] = 7),
        Box::new(|a| a.meshes[0].frames[0].triangles.clear()),
        Box::new(|a| a.meshes[0].id.clear()),
        Box::new(|a| a.meshes[1].id = a.meshes[0].id.clone()),
        Box::new(|a| a.meshes[0].source_entity_ids.clear()),
        Box::new(|a| a.meshes[0].source_entity_ids.push("body".into())),
        Box::new(|a| a.materials[1].id = a.materials[0].id.clone()),
        Box::new(|a| a.materials[0].id = "bad\0id".into()),
        Box::new(|a| a.materials[0].diffuse_color[0] = -0.1),
        Box::new(|a| a.materials[0].emissive_color[0] = f32::INFINITY),
        Box::new(|a| a.materials[0].roughness = 1.01),
        Box::new(|a| a.materials[0].metallic = f32::NAN),
        Box::new(|a| a.materials[0].ior = f32::NAN),
        Box::new(|a| a.materials[0].ior = 0.99),
        Box::new(|a| a.materials[0].ior = 4.01),
        Box::new(|a| a.camera.as_mut().unwrap().frames.clear()),
        Box::new(|a| a.camera.as_mut().unwrap().frames[0].target = [0.0, 0.0, 5.0]),
        Box::new(|a| a.camera.as_mut().unwrap().frames[0].up = [0.0, 0.0, 1.0]),
        Box::new(|a| a.camera.as_mut().unwrap().frames[0].position[0] = f32::INFINITY),
        Box::new(|a| a.camera.as_mut().unwrap().frames[0].vertical_fov_degrees = 180.0),
        Box::new(|a| a.camera.as_mut().unwrap().frames[0].vertical_fov_degrees = 0.0),
        Box::new(|a| a.camera.as_mut().unwrap().frames[0].aspect_ratio = -1.0),
        Box::new(|a| a.camera.as_mut().unwrap().frames[0].near_clip = 0.0),
        Box::new(|a| a.camera.as_mut().unwrap().frames[0].far_clip = 0.01),
        Box::new(|a| a.camera.as_mut().unwrap().frames[0].aperture_radius_m = -0.01),
        Box::new(|a| a.camera.as_mut().unwrap().frames[0].aperture_radius_m = f32::NAN),
        Box::new(|a| a.camera.as_mut().unwrap().frames[0].focus_distance_m = 0.0),
        Box::new(|a| a.camera.as_mut().unwrap().frames[0].focus_distance_m = f32::INFINITY),
        Box::new(|a| a.camera.as_mut().unwrap().frames[0].shutter_open_seconds = f64::NAN),
        Box::new(|a| a.camera.as_mut().unwrap().frames[0].shutter_close_seconds = -0.02),
        Box::new(|a| a.camera.as_mut().unwrap().frames[0].shutter_close_seconds = f64::MAX),
        Box::new(|a| a.camera.as_mut().unwrap().frames[0].vertical_fov_degrees = f32::from_bits(1)),
        Box::new(|a| a.camera.as_mut().unwrap().frames[0].aspect_ratio = f32::from_bits(1)),
    ];
    for (index, mutate) in cases.iter().enumerate() {
        let mut asset = fixture();
        mutate(&mut asset);
        assert!(encode_usda(&asset).is_err(), "invalid case {index} accepted");
    }
}

#[test]
fn usd_writer_represents_mesh_disappearance_and_single_frame_delivery() {
    let mut asset = fixture();
    asset.meshes[0].frames[1].positions.clear();
    asset.meshes[0].frames[1].triangles.clear();
    asset.meshes[0].frames[1].material_ids.clear();
    assert!(encode_usda(&asset).is_ok());
    asset.end_time_code = asset.start_time_code;
    for mesh in &mut asset.meshes {
        mesh.frames.truncate(1);
    }
    asset.camera = None;
    assert!(encode_usda(&asset).is_ok());
}

/// This is an explicit acceptance gate, not a text-pattern substitute for a USD
/// reader. Install usd-core into a temporary directory, then run with
/// MM3E_USD_PYTHONPATH=/tmp/mm3e-usd-reader cargo test -p mm3e-editor --test usd_writer -- --ignored
#[test]
#[ignore = "requires independent pxr OpenUSD reader; set MM3E_USD_PYTHONPATH"]
fn openusd_reader_validates_real_schema_values_and_bindings() {
    let pythonpath = std::env::var("MM3E_USD_PYTHONPATH").expect("set MM3E_USD_PYTHONPATH to usd-core installation");
    for disappearing in [false, true] {
        let mut asset = fixture();
        if disappearing {
            asset.meshes[0].frames[1].positions.clear();
            asset.meshes[0].frames[1].triangles.clear();
            asset.meshes[0].frames[1].material_ids.clear();
        }
        let path = std::env::temp_dir().join(format!("mm3e-usd-writer-{}-{disappearing}.usda", std::process::id()));
        std::fs::write(&path, encode_usda(&asset).unwrap()).unwrap();
        let expected = serde_json::json!({
            "fps": asset.frames_per_second,
            "start": asset.start_time_code,
            "end": asset.end_time_code,
            "meshes": asset.meshes.iter().map(|mesh| serde_json::json!({
                "path": format!("/World/Geometry/{}", prim_name(&mesh.id)),
                "id": mesh.id,
                "source_ids": mesh.source_entity_ids,
                "representation": if mesh.representation == UsdRepresentation::ComposedSdfBake {"composedSdfBake"} else {"nativeSurfaceMeshCache"},
                "double_sided": mesh.double_sided,
                "frames": mesh.frames.iter().map(|frame| serde_json::json!({
                    "time": frame.time, "points": frame.positions, "triangles": frame.triangles, "material_ids": frame.material_ids
                })).collect::<Vec<_>>()
            })).collect::<Vec<_>>(),
            "materials": asset.materials.iter().map(|material| serde_json::json!({
                "path": format!("/World/Looks/{}", prim_name(&material.id)),
                "id": material.id, "diffuse": material.diffuse_color, "emissive": material.emissive_color, "roughness": material.roughness, "metallic": material.metallic, "ior": material.ior
            })).collect::<Vec<_>>(),
            "camera": {
                "path": format!("/World/{}", prim_name(&asset.camera.as_ref().unwrap().id)),
                "frames": asset.camera.as_ref().unwrap().frames.iter().map(|frame| serde_json::json!({
                    "time": frame.time, "position": frame.position, "target": frame.target,
                    "fov": frame.vertical_fov_degrees, "aspect": frame.aspect_ratio,
                    "near": frame.near_clip, "far": frame.far_clip,
                    "aperture_radius_m": frame.aperture_radius_m, "focus_distance_m": frame.focus_distance_m,
                    "shutter_open_seconds": frame.shutter_open_seconds, "shutter_close_seconds": frame.shutter_close_seconds
                })).collect::<Vec<_>>()
            }
        });
        let expected_path = path.with_extension("json");
        std::fs::write(&expected_path, serde_json::to_vec(&expected).unwrap()).unwrap();
        let output = std::process::Command::new("python3")
            .args(["-c", OPENUSD_READER])
            .arg(&path)
            .arg(&expected_path)
            .env("PYTHONPATH", &pythonpath)
            .output()
            .expect("launch independent OpenUSD Python reader");
        assert!(
            output.status.success(),
            "OpenUSD rejected {}:\n{}\n{}",
            path.display(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        println!("{}", String::from_utf8_lossy(&output.stdout));
        std::fs::remove_file(path).unwrap();
        std::fs::remove_file(expected_path).unwrap();
    }
}

const OPENUSD_READER: &str = r#"
import json, math, sys
from pxr import Usd, UsdGeom, UsdShade, UsdValidation, Gf
stage = Usd.Stage.Open(sys.argv[1])
assert stage, 'stage did not open'
expected = json.load(open(sys.argv[2]))
def close(a, b, epsilon=1e-5):
    if isinstance(b, list):
        assert len(a) == len(b), (a,b)
        for x,y in zip(a,b): close(x,y,epsilon)
    else: assert math.isclose(a,b,abs_tol=epsilon,rel_tol=epsilon), (a,b)
assert stage.GetDefaultPrim().GetPath() == '/World'
assert UsdGeom.GetStageMetersPerUnit(stage) == 1
assert UsdGeom.GetStageUpAxis(stage) == 'Y'
assert stage.GetFramesPerSecond() == expected['fps']
assert stage.GetTimeCodesPerSecond() == expected['fps']
assert stage.GetStartTimeCode() == expected['start']
assert stage.GetEndTimeCode() == expected['end']
assert stage.GetRootLayer().customLayerData['representation'] == 'evaluatedPolygonMeshCache'
registry = UsdValidation.ValidationRegistry()
metadata = [m for m in registry.GetAllValidatorMetadata() if set(m.GetKeywords()) & {'UsdCoreValidators', 'UsdGeomValidators', 'UsdShadeValidators'}]
assert metadata, 'OpenUSD schema validators unavailable'
validators = [registry.GetOrLoadValidatorByName(m.name) for m in metadata]
errors = UsdValidation.ValidationContext(validators).Validate(stage)
assert not errors, [error.GetMessage() for error in errors]
for material in expected['materials']:
    mat = UsdShade.Material(stage.GetPrimAtPath(material['path']))
    assert mat and mat.GetPrim().GetCustomDataByKey('originalMaterialId') == material['id']
    shader, _, _ = mat.ComputeSurfaceSource()
    assert shader and shader.GetIdAttr().Get() == 'UsdPreviewSurface'
    close(shader.GetInput('diffuseColor').Get(), material['diffuse'])
    close(shader.GetInput('emissiveColor').Get(), material['emissive'])
    close(shader.GetInput('roughness').Get(), material['roughness'])
    close(shader.GetInput('metallic').Get(), material['metallic'])
    close(shader.GetInput('ior').Get(), material['ior'])
    assert shader.GetInput('useSpecularWorkflow').Get() == 0
    if material['metallic'] == 0 and material['ior'] == 1.5:
        ior = shader.GetInput('ior').Get()
        close(((ior - 1) / (ior + 1)) ** 2, 0.04)
for item in expected['meshes']:
    mesh = UsdGeom.Mesh(stage.GetPrimAtPath(item['path']))
    assert mesh
    assert mesh.GetPrim().GetCustomDataByKey('originalMeshId') == item['id']
    assert list(mesh.GetPrim().GetCustomDataByKey('originalEntityIds')) == item['source_ids']
    assert mesh.GetPrim().GetCustomDataByKey('representation') == item['representation']
    assert mesh.GetSubdivisionSchemeAttr().Get() == 'none'
    assert mesh.GetOrientationAttr().Get() == 'rightHanded'
    assert mesh.GetDoubleSidedAttr().Get() == item['double_sided']
    assert mesh.GetNormalsInterpolation() == 'uniform'
    times = [frame['time'] for frame in item['frames']]
    assert mesh.GetPointsAttr().GetTimeSamples() == times
    topology_varies = any(f['triangles'] != item['frames'][0]['triangles'] or len(f['points']) != len(item['frames'][0]['points']) for f in item['frames'][1:])
    assert mesh.GetFaceVertexCountsAttr().GetTimeSamples() == (times if topology_varies else [])
    assert mesh.GetFaceVertexIndicesAttr().GetTimeSamples() == (times if topology_varies else [])
    subsets = UsdShade.MaterialBindingAPI(mesh).GetMaterialBindSubsets()
    assert UsdGeom.Subset.GetFamilyType(mesh, 'materialBind') == 'partition'
    for frame in item['frames']:
        time = frame['time']
        points = mesh.GetPointsAttr().Get(time)
        counts = mesh.GetFaceVertexCountsAttr().Get(time)
        indices = mesh.GetFaceVertexIndicesAttr().Get(time)
        close(points, frame['points'])
        assert list(counts) == [3] * len(frame['triangles'])
        assert list(indices) == [index for tri in frame['triangles'] for index in tri]
        valid, reason = UsdGeom.Mesh.ValidateTopology(indices, counts, len(points))
        assert valid, reason
        normals = mesh.GetNormalsAttr().Get(time)
        assert len(normals) == len(frame['triangles'])
        for normal, triangle in zip(normals, frame['triangles']):
            a,b,c = [Gf.Vec3d(*frame['points'][i]) for i in triangle]
            wanted = Gf.Cross(b-a,c-a).GetNormalized()
            close(normal, list(wanted))
        wanted_extent = [[min(p[axis] for p in frame['points']) for axis in range(3)], [max(p[axis] for p in frame['points']) for axis in range(3)]] if frame['points'] else [[0,0,0],[0,0,0]]
        close(mesh.GetExtentAttr().Get(time), wanted_extent)
        assigned = {}
        for subset in subsets:
            material, relationship = UsdShade.MaterialBindingAPI(subset).ComputeBoundMaterial()
            assert material and relationship
            for face in subset.GetIndicesAttr().Get(time):
                assert face not in assigned
                assigned[face] = str(material.GetPath())
        assert len(assigned) == len(frame['triangles'])
        for face, material_id in enumerate(frame['material_ids']):
            assert assigned[face] == expected['materials'][material_id]['path']
    valid, reason = UsdGeom.Subset.ValidateFamily(mesh, 'face', 'materialBind')
    assert valid, reason
camera = UsdGeom.Camera(stage.GetPrimAtPath(expected['camera']['path']))
assert camera and camera.GetProjectionAttr().Get() == 'perspective'
assert camera.GetFocalLengthAttr().GetTimeSamples() == [f['time'] for f in expected['camera']['frames']]
for frame in expected['camera']['frames']:
    time = frame['time']
    matrix = camera.ComputeLocalToWorldTransform(time)
    close(matrix.ExtractTranslation(), frame['position'])
    world_forward = matrix.TransformDir(Gf.Vec3d(0,0,-1)).GetNormalized()
    direction = (Gf.Vec3d(*frame['target'])-Gf.Vec3d(*frame['position'])).GetNormalized()
    close(world_forward, list(direction))
    local_target = matrix.GetInverse().Transform(Gf.Vec3d(*frame['target']))
    close(local_target[0], 0)
    close(local_target[1], 0)
    assert local_target[2] < 0
    close(matrix.GetDeterminant(), 1)
    aperture = camera.GetVerticalApertureAttr().Get(time)
    close(aperture, 0.24)
    close(camera.GetHorizontalApertureAttr().Get(time) / aperture, frame['aspect'])
    fov = math.degrees(2*math.atan(aperture/(2*camera.GetFocalLengthAttr().Get(time))))
    close(fov, frame['fov'])
    close(camera.GetClippingRangeAttr().Get(time), [frame['near'],frame['far']])
    # Inspect physical scale as well as the FOV ratio, which could hide 100x optics errors.
    gf_camera = camera.GetCamera(time)
    close(gf_camera.verticalAperture * Gf.Camera.APERTURE_UNIT * UsdGeom.GetStageMetersPerUnit(stage), 0.024)
    close(gf_camera.focalLength * Gf.Camera.FOCAL_LENGTH_UNIT * UsdGeom.GetStageMetersPerUnit(stage), 0.012 / math.tan(math.radians(frame['fov']) / 2))
    close(gf_camera.GetFieldOfView(Gf.Camera.FOVVertical), frame['fov'])
    close(gf_camera.focusDistance, frame['focus_distance_m'])
    close(gf_camera.frustum.GetNearFar().GetMin(), frame['near'])
    close(gf_camera.frustum.GetNearFar().GetMax(), frame['far'])
    if frame['aperture_radius_m'] == 0:
        assert gf_camera.fStop == 0
    else:
        recovered_radius = gf_camera.focalLength * Gf.Camera.FOCAL_LENGTH_UNIT / (2 * gf_camera.fStop)
        close(recovered_radius, frame['aperture_radius_m'], 1e-7)
    close(camera.GetShutterOpenAttr().Get(time), frame['shutter_open_seconds'] * expected['fps'])
    close(camera.GetShutterCloseAttr().Get(time), frame['shutter_close_seconds'] * expected['fps'])
print('OpenUSD', '.'.join(map(str, Usd.GetVersion())), 'passed', len(validators), 'schema validators and verified mesh points/topology/normals/extents, animated material partitions, linear materials, IDs, meters/Y-up, timing, physical lens/focus/shutter and camera:', sys.argv[1])
"#;

#[test]
#[ignore = "requires independent pxr OpenUSD reader; set MM3E_USD_PYTHONPATH"]
fn openusd_reader_validates_texture_graphs_uv_seams_and_original_packed_channels() {
    use sha2::{Digest, Sha256};
    let pythonpath = std::env::var("MM3E_USD_PYTHONPATH").expect("set MM3E_USD_PYTHONPATH to usd-core installation");
    let path = std::env::temp_dir().join(format!(
        "mm3e-textured-usd-{}-{}",
        std::process::id(),
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
    ));
    std::fs::create_dir_all(path.join("textures")).unwrap();
    let pixels = [51u8, 153, 229, 64, 204, 102, 26, 255];
    let mut png_bytes = vec![];
    {
        let mut encoder = png::Encoder::new(&mut png_bytes, 2, 1);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&pixels).unwrap();
        writer.finish().unwrap();
    }
    std::fs::write(path.join("textures/packed.png"), &png_bytes).unwrap();
    for mode in ["animated", "constant", "missing", "disappearing"] {
        let mut asset = textured_fixture();
        if mode == "constant" {
            let mesh = &mut asset.meshes[1];
            mesh.frames[1].corner_uvs = mesh.frames[0].corner_uvs.clone();
        } else if mode == "missing" {
            asset.meshes[0].frames[1].corner_uvs = None;
            asset.meshes[0].frames[1].material_ids = vec![1, 1];
        } else if mode == "disappearing" {
            let frame = &mut asset.meshes[0].frames[1];
            frame.positions.clear();
            frame.triangles.clear();
            frame.material_ids.clear();
            frame.corner_uvs = None;
        }
        let layer = path.join(format!("{mode}.usda"));
        std::fs::write(&layer, encode_usda(&asset).unwrap()).unwrap();
        let material = &asset.materials[0];
        let expected = serde_json::json!({
            "material_path":format!("/World/Looks/{}",prim_name(&material.id)),
            "textures":material.textures.iter().map(|texture| {
                let (input,input_type)=match texture.role {UsdTextureRole::Diffuse=>("diffuseColor","color3f"),UsdTextureRole::Emissive=>("emissiveColor","color3f"),UsdTextureRole::Roughness=>("roughness","float"),UsdTextureRole::Metallic=>("metallic","float"),UsdTextureRole::Normal=>("normal","normal3f")};
                let channel=match texture.channel {UsdTextureChannel::Rgb=>"rgb",UsdTextureChannel::R=>"r",UsdTextureChannel::G=>"g",UsdTextureChannel::B=>"b",UsdTextureChannel::A=>"a"};
                serde_json::json!({"input":input,"input_type":input_type,"channel":channel,"space":if texture.source_color_space==UsdTextureColorSpace::Raw{"raw"}else{"sRGB"},"scale":texture.scale,"bias":texture.bias,"path":texture.asset_path})
            }).collect::<Vec<_>>(),
            "meshes":asset.meshes.iter().map(|mesh|serde_json::json!({
                "path":format!("/World/Geometry/{}",prim_name(&mesh.id)),
                "uv_sampled":mesh.frames.iter().skip(1).any(|frame|frame.corner_uvs!=mesh.frames[0].corner_uvs||frame.triangles!=mesh.frames[0].triangles||frame.positions.len()!=mesh.frames[0].positions.len()),
                "frames":mesh.frames.iter().map(|frame|serde_json::json!({"time":frame.time,"triangles":frame.triangles,"uvs":frame.corner_uvs})).collect::<Vec<_>>()
            })).collect::<Vec<_>>(),
            "png_sha256":format!("{:x}",Sha256::digest(&png_bytes)),"png_pixels":pixels,
        });
        let reference = path.join(format!("{mode}.json"));
        std::fs::write(&reference, serde_json::to_vec(&expected).unwrap()).unwrap();
        let output = std::process::Command::new("python3")
            .args(["-c", TEXTURED_OPENUSD_READER])
            .arg(&layer)
            .arg(&reference)
            .env("PYTHONPATH", &pythonpath)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "OpenUSD textured reader failed {}:\n{}\n{}",
            layer.display(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
    std::fs::remove_dir_all(path).unwrap();
}

const TEXTURED_OPENUSD_READER: &str = r#"
import hashlib, json, math, pathlib, struct, sys, zlib
from pxr import Usd, UsdGeom, UsdShade, UsdValidation
stage=Usd.Stage.Open(sys.argv[1]); assert stage
expected=json.load(open(sys.argv[2]))
def close(a,b):
    if isinstance(b,list):
        assert len(a)==len(b),(a,b)
        for x,y in zip(a,b): close(x,y)
    else: assert math.isclose(a,b,abs_tol=1e-6,rel_tol=1e-6),(a,b)
registry=UsdValidation.ValidationRegistry()
metadata=[m for m in registry.GetAllValidatorMetadata() if set(m.GetKeywords()) & {'UsdCoreValidators','UsdGeomValidators','UsdShadeValidators'}]
validators=[registry.GetOrLoadValidatorByName(m.name) for m in metadata]
assert validators
errors=UsdValidation.ValidationContext(validators).Validate(stage)
assert not errors,[error.GetMessage() for error in errors]
material=UsdShade.Material(stage.GetPrimAtPath(expected['material_path']))
surface,_,_=material.ComputeSurfaceSource()
assert surface.GetIdAttr().Get()=='UsdPreviewSurface'
assert surface.GetInput('opacity').Get()==1 and surface.GetInput('opacity').GetConnectedSource() is None
nodes={}
for texture in expected['textures']:
    connection=surface.GetInput(texture['input'])
    assert str(connection.GetTypeName())==texture['input_type']
    source,output,kind=connection.GetConnectedSource()
    node=UsdShade.Shader(source.GetPrim());assert node.GetIdAttr().Get()=='UsdUVTexture'
    assert str(output)==texture['channel']
    assert str(node.GetOutput(output).GetTypeName())==('float3' if output=='rgb' else 'float')
    file=node.GetInput('file').Get();assert file.path==texture['path']
    assert pathlib.Path(file.resolvedPath).is_file(),file
    assert hashlib.sha256(pathlib.Path(file.resolvedPath).read_bytes()).hexdigest()==expected['png_sha256']
    assert node.GetInput('sourceColorSpace').Get()==texture['space']
    assert node.GetInput('wrapS').Get()=='mirror';assert node.GetInput('wrapT').Get()=='clamp'
    close(node.GetInput('scale').Get(),texture['scale']);close(node.GetInput('bias').Get(),texture['bias'])
    primvar,output,kind=node.GetInput('st').GetConnectedSource()
    reader=UsdShade.Shader(primvar.GetPrim())
    assert reader.GetIdAttr().Get()=='UsdPrimvarReader_float2'
    assert reader.GetInput('varname').Get()=='st' and str(output)=='result'
    assert str(reader.GetOutput('result').GetTypeName())=='float2'
    nodes[texture['input']]=node
for item in expected['meshes']:
    mesh=UsdGeom.Mesh(stage.GetPrimAtPath(item['path']))
    uv=UsdGeom.PrimvarsAPI(mesh).GetPrimvar('st')
    assert uv and uv.GetInterpolation()=='faceVarying' and str(uv.GetTypeName())=='texCoord2f[]'
    assert not uv.IsIndexed()
    assert uv.GetAttr().GetTimeSamples()==([f['time'] for f in item['frames']] if item['uv_sampled'] else [])
    for frame in item['frames']:
        actual=uv.ComputeFlattened(frame['time'])
        if frame['uvs'] is None:
            assert actual is None,actual
        else:
            assert len(actual)==3*len(frame['triangles'])
            close(actual,[pair for triangle in frame['uvs'] for pair in triangle])
# Decode the original RGBA PNG independently. This fixture has one row, so
# PNG's left/above filters reduce to these exact byte recurrences.
data=pathlib.Path(nodes['roughness'].GetInput('file').Get().resolvedPath).read_bytes()
assert data[:8]==b'\x89PNG\r\n\x1a\n'
offset=8;compressed=b''
while offset<len(data):
    length=struct.unpack_from('>I',data,offset)[0];kind=data[offset+4:offset+8];body=data[offset+8:offset+8+length]
    if kind==b'IHDR': assert struct.unpack('>IIBBBBB',body)==(2,1,8,6,0,0,0)
    if kind==b'IDAT': compressed+=body
    offset+=length+12
row=zlib.decompress(compressed);filtered=row[0];assert len(row)==9 and filtered in range(5)
pixels=[]
for byte in row[1:]:
    left=pixels[-4] if len(pixels)>=4 else 0
    predictor=left if filtered in (1,4) else left//2 if filtered==3 else 0
    pixels.append((byte+predictor)&255)
assert pixels==expected['png_pixels']
rough=nodes['roughness'];metal=nodes['metallic']
assert surface.GetInput('roughness').GetConnectedSource()[1]=='g'
assert surface.GetInput('metallic').GetConnectedSource()[1]=='a'
rough_value=pixels[1]/255*rough.GetInput('scale').Get()[1]+rough.GetInput('bias').Get()[1]
metal_value=pixels[3]/255*metal.GetInput('scale').Get()[3]+metal.GetInput('bias').Get()[3]
close(rough_value,0.47);close(metal_value,64/255*0.6+0.04)
assert abs(rough_value-0.47*pixels[3]/255)>0.2,'raw scalar must not be alpha-premultiplied'
print('OpenUSD',Usd.GetVersion(),'validated texture connections/types, original PNG identities, raw G/A channel math and authored UV samples:',sys.argv[1])
"#;
