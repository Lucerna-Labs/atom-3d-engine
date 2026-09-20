//! Evaluated polygon-mesh caches for film interchange. This writer does not export
//! editable SDF operators, skeletons, skin weights, procedural materials or simulation.
//! Coordinates are world-space meters, Y up; colors are scene-linear Rec.709/D65
//! RGB, matching the native EXR convention. No OCIO configuration is embedded.
//!
//! Schema references used by this writer:
//! - <https://openusd.org/release/api/class_usd_geom_mesh.html>
//! - <https://openusd.org/release/api/class_usd_geom_subset.html>
//! - <https://openusd.org/release/api/class_usd_geom_camera.html>
//! - <https://openusd.org/release/spec_usdpreviewsurface.html>
//!
//! `tests/usd_writer.rs::openusd_reader_validates_real_schema_values_and_bindings`
//! independently opens generated layers with OpenUSD, runs its core/geometry/shading
//! validators, and checks numeric geometry, material and camera values. A layer's
//! generator metadata alone is not a claim that a reader validated that delivery.
use std::{collections::BTreeSet, fmt::Write};

#[derive(Clone, Debug)]
pub struct UsdAsset {
    pub frames_per_second: f64,
    pub start_time_code: f64,
    pub end_time_code: f64,
    pub meshes: Vec<UsdMesh>,
    pub materials: Vec<UsdMaterial>,
    pub camera: Option<UsdCamera>,
}

#[derive(Clone, Debug)]
pub struct UsdMesh {
    pub id: String,
    pub source_entity_ids: Vec<String>,
    pub representation: UsdRepresentation,
    pub double_sided: bool,
    pub frames: Vec<FrameMesh>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsdRepresentation {
    ComposedSdfBake,
    NativeSurface,
}

#[derive(Clone, Debug)]
pub struct FrameMesh {
    pub time: f64,
    pub positions: Vec<[f32; 3]>,
    pub triangles: Vec<[u32; 3]>,
    /// One index into `UsdAsset::materials` per triangle.
    pub material_ids: Vec<usize>,
    /// Independent values in triangle-corner order, preserving UV seams.
    pub corner_uvs: Option<Vec<[[f32; 2]; 3]>>,
}

#[derive(Clone, Debug)]
pub struct UsdMaterial {
    pub id: String,
    pub diffuse_color: [f32; 3],
    pub roughness: f32,
    pub metallic: f32,
    /// Dielectric IOR. F0 = ((ior - 1) / (ior + 1))^2 in the metallic workflow.
    pub ior: f32,
    pub emissive_color: [f32; 3],
    pub textures: Vec<UsdTexture>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum UsdTextureRole {
    Diffuse,
    Emissive,
    Roughness,
    Metallic,
    Normal,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsdTextureChannel {
    Rgb,
    R,
    G,
    B,
    A,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsdTextureColorSpace {
    Raw,
    Srgb,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsdTextureWrap {
    Clamp,
    Repeat,
    Mirror,
}

/// A portable PreviewSurface connection, with reader-defined texture filtering.
/// This describes already-prepared image values; it does not implement native
/// alpha modulation, roughness floors, tangent-frame transfer or LOD policy.
#[derive(Clone, Debug)]
pub struct UsdTexture {
    pub role: UsdTextureRole,
    pub asset_path: String,
    pub channel: UsdTextureChannel,
    pub source_color_space: UsdTextureColorSpace,
    pub wrap_s: UsdTextureWrap,
    pub wrap_t: UsdTextureWrap,
    pub scale: [f32; 4],
    pub bias: [f32; 4],
}

impl UsdTextureRole {
    fn input(self) -> (&'static str, &'static str) {
        match self {
            Self::Diffuse => ("color3f", "diffuseColor"),
            Self::Emissive => ("color3f", "emissiveColor"),
            Self::Roughness => ("float", "roughness"),
            Self::Metallic => ("float", "metallic"),
            Self::Normal => ("normal3f", "normal"),
        }
    }
}
impl UsdTextureChannel {
    fn output(self) -> (&'static str, &'static str) {
        match self {
            Self::Rgb => ("float3", "rgb"),
            Self::R => ("float", "r"),
            Self::G => ("float", "g"),
            Self::B => ("float", "b"),
            Self::A => ("float", "a"),
        }
    }
}
impl UsdTextureWrap {
    fn token(self) -> &'static str {
        match self {
            Self::Clamp => "clamp",
            Self::Repeat => "repeat",
            Self::Mirror => "mirror",
        }
    }
}

#[derive(Clone, Debug)]
pub struct UsdCamera {
    pub id: String,
    pub frames: Vec<CameraFrame>,
}

#[derive(Clone, Debug)]
pub struct CameraFrame {
    pub time: f64,
    pub position: [f32; 3],
    pub target: [f32; 3],
    pub up: [f32; 3],
    pub vertical_fov_degrees: f32,
    pub aspect_ratio: f32,
    pub near_clip: f32,
    pub far_clip: f32,
    /// Radius of the native thin lens in meters; zero means pinhole.
    pub aperture_radius_m: f32,
    pub focus_distance_m: f32,
    /// Frame-relative exposure offsets in seconds, converted to USD time codes.
    pub shutter_open_seconds: f64,
    pub shutter_close_seconds: f64,
}

/// Encode a complete USDA layer, validating the entire payload before encoding.
/// Every mesh and the optional camera must have the same strictly increasing
/// sample times, including both declared endpoints. Empty mesh samples describe
/// disappearance. Between-sample behavior is controlled by the consuming stage;
/// only authored times are evaluated outputs of the native engine.
pub fn encode_usda(asset: &UsdAsset) -> Result<Vec<u8>, String> {
    validate(asset)?;
    let mut out = String::new();
    writeln!(out, "#usda 1.0\n(\n    defaultPrim = \"World\"\n    metersPerUnit = 1\n    upAxis = \"Y\"\n    framesPerSecond = {}\n    timeCodesPerSecond = {}\n    startTimeCode = {}\n    endTimeCode = {}", asset.frames_per_second, asset.frames_per_second, asset.start_time_code, asset.end_time_code).unwrap();
    out.push_str("    customLayerData = {\n        string generator = \"MM3E native agent editor\"\n        string representation = \"evaluatedPolygonMeshCache\"\n        string colorEncoding = \"scene-linear Rec.709/D65 RGB\"\n        string sampleInterpretation = \"Native evaluation at authored times; between-sample interpolation is reader-defined.\"\n    }\n)\n\ndef Xform \"World\"\n{\n    def Scope \"Looks\"\n    {\n");
    for material in &asset.materials {
        write_material(&mut out, material);
    }
    out.push_str("    }\n    def Scope \"Geometry\"\n    {\n");
    for mesh in &asset.meshes {
        write_mesh(&mut out, mesh, &asset.materials);
    }
    out.push_str("    }\n");
    if let Some(camera) = &asset.camera {
        write_camera(&mut out, camera, asset.frames_per_second);
    }
    out.push_str("}\n");
    Ok(out.into_bytes())
}

/// Stable, collision-free USD identifier; UTF-8 bytes are hex encoded rather than
/// replacing punctuation (which would alias IDs such as `a-b` and `a_b`).
pub fn prim_name(id: &str) -> String {
    let mut name = String::from("id_");
    for byte in id.as_bytes() {
        write!(name, "{byte:02x}").unwrap();
    }
    name
}

fn validate(asset: &UsdAsset) -> Result<(), String> {
    if !asset.frames_per_second.is_finite() || asset.frames_per_second <= 0.0 {
        return Err("USD frames_per_second must be finite and positive".into());
    }
    if !asset.start_time_code.is_finite()
        || !asset.end_time_code.is_finite()
        || asset.end_time_code < asset.start_time_code
    {
        return Err("USD time range must be finite and ordered".into());
    }
    if asset.meshes.is_empty() || asset.materials.is_empty() {
        return Err("USD delivery requires at least one mesh and material".into());
    }
    let mut material_ids = BTreeSet::new();
    for material in &asset.materials {
        valid_id(&material.id)?;
        if !material_ids.insert(&material.id) {
            return Err("USD material IDs must be unique".into());
        }
        if !material.diffuse_color.iter().chain(&material.emissive_color).all(|v| v.is_finite() && *v >= 0.0)
            || !material.roughness.is_finite()
            || !(0.0..=1.0).contains(&material.roughness)
            || !material.metallic.is_finite()
            || !(0.0..=1.0).contains(&material.metallic)
            || !material.ior.is_finite()
            || !(1.0..=4.0).contains(&material.ior)
        {
            return Err(format!("USD material {} has invalid scene-linear colors or surface parameters", material.id));
        }
        let mut roles = BTreeSet::new();
        for texture in &material.textures {
            if !roles.insert(texture.role) {
                return Err("USD material repeats a texture role".into());
            }
            validate_texture(texture)?;
        }
    }
    let reference_times: Vec<_> = asset.meshes[0].frames.iter().map(|f| f.time).collect();
    validate_times(&reference_times, asset)?;
    let mut mesh_ids = BTreeSet::new();
    for mesh in &asset.meshes {
        valid_id(&mesh.id)?;
        if !mesh_ids.insert(&mesh.id) {
            return Err("USD mesh IDs must be unique".into());
        }
        if mesh.source_entity_ids.is_empty() {
            return Err("USD mesh requires original entity IDs".into());
        }
        let mut source_ids = BTreeSet::new();
        for id in &mesh.source_entity_ids {
            valid_id(id)?;
            if !source_ids.insert(id) {
                return Err("USD original entity IDs must be unique within a mesh".into());
            }
        }
        let times: Vec<_> = mesh.frames.iter().map(|f| f.time).collect();
        if times != reference_times {
            return Err("USD meshes must share the complete evaluated timeline".into());
        }
        for frame in &mesh.frames {
            if frame.positions.len() > i32::MAX as usize || frame.triangles.len() > i32::MAX as usize / 3 {
                return Err("USD mesh exceeds signed 32-bit topology limits".into());
            }
            if frame.positions.is_empty() != frame.triangles.is_empty() {
                return Err("USD empty meshes must have both empty positions and topology".into());
            }
            if !frame.positions.iter().flatten().all(|v| v.is_finite()) {
                return Err("USD positions must be finite".into());
            }
            if frame.material_ids.len() != frame.triangles.len()
                || frame.material_ids.iter().any(|&id| id >= asset.materials.len())
            {
                return Err("USD mesh requires one valid material index per triangle".into());
            }
            if let Some(uvs) = &frame.corner_uvs {
                if uvs.len() != frame.triangles.len() || !uvs.iter().flatten().flatten().all(|v| v.is_finite()) {
                    return Err("USD corner UVs require three finite UV pairs for every triangle".into());
                }
            } else if frame.material_ids.iter().any(|&id| !asset.materials[id].textures.is_empty()) {
                return Err("USD textured faces require authored corner UVs in every nonempty sample".into());
            }
            for triangle in &frame.triangles {
                if triangle.iter().any(|&v| v as usize >= frame.positions.len()) {
                    return Err("USD triangle index outside positions array".into());
                }
                if face_normal(frame, triangle).is_none() {
                    return Err("USD mesh contains a degenerate triangle".into());
                }
            }
        }
    }
    if let Some(camera) = &asset.camera {
        valid_id(&camera.id)?;
        if camera.frames.iter().map(|f| f.time).collect::<Vec<_>>() != reference_times {
            return Err("USD camera must share the complete evaluated mesh timeline".into());
        }
        for frame in &camera.frames {
            if !frame.position.iter().chain(&frame.target).chain(&frame.up).all(|v| v.is_finite())
                || camera_basis(frame).is_none()
                || !frame.vertical_fov_degrees.is_finite()
                || !(0.0..180.0).contains(&frame.vertical_fov_degrees)
                || frame.vertical_fov_degrees == 0.0
                || !frame.aspect_ratio.is_finite()
                || frame.aspect_ratio <= 0.0
                || !frame.near_clip.is_finite()
                || !frame.far_clip.is_finite()
                || frame.near_clip <= 0.0
                || frame.far_clip <= frame.near_clip
                || !camera_focal_length(frame).is_finite()
                || camera_focal_length(frame) <= 0.0
                || !(camera_focal_length(frame) as f32).is_finite()
                || (0.24 * f64::from(frame.aspect_ratio)) as f32 <= 0.0
                || !frame.aperture_radius_m.is_finite()
                || frame.aperture_radius_m < 0.0
                || !frame.focus_distance_m.is_finite()
                || frame.focus_distance_m <= 0.0
                || !(camera_f_stop(frame) as f32).is_finite()
                || (frame.aperture_radius_m > 0.0 && camera_f_stop(frame) as f32 <= 0.0)
                || !(frame.shutter_open_seconds * asset.frames_per_second).is_finite()
                || !(frame.shutter_close_seconds * asset.frames_per_second).is_finite()
                || frame.shutter_close_seconds < frame.shutter_open_seconds
            {
                return Err("USD camera has an invalid basis, projection or clipping range".into());
            }
        }
    }
    Ok(())
}

fn validate_times(times: &[f64], asset: &UsdAsset) -> Result<(), String> {
    if times.first() != Some(&asset.start_time_code)
        || times.last() != Some(&asset.end_time_code)
        || !times.iter().all(|t| t.is_finite())
        || !times.windows(2).all(|t| t[1] > t[0])
    {
        return Err("USD samples must be increasing and include both declared time endpoints".into());
    }
    Ok(())
}

fn valid_id(id: &str) -> Result<(), String> {
    if id.is_empty() || id.len() > 1024 || id.chars().any(|c| c.is_control() && !matches!(c, '\n' | '\r' | '\t')) {
        Err("USD IDs must contain 1..1024 UTF-8 bytes without unsupported control characters".into())
    } else {
        Ok(())
    }
}

fn validate_texture(texture: &UsdTexture) -> Result<(), String> {
    let path = &texture.asset_path;
    if path.is_empty()
        || path.len() > 1024
        || !path.bytes().all(|c| c.is_ascii_alphanumeric() || b"_./-".contains(&c))
        || path.split('/').any(|component| component.is_empty() || component == "." || component == "..")
    {
        return Err(
            "USD texture asset path must use safe relative ASCII components without traversal or delimiters".into()
        );
    }
    if !texture.scale.iter().chain(&texture.bias).all(|v| v.is_finite()) {
        return Err("USD texture scale and bias must be finite".into());
    }
    let scalar = matches!(texture.role, UsdTextureRole::Roughness | UsdTextureRole::Metallic);
    if scalar == (texture.channel == UsdTextureChannel::Rgb) {
        return Err("USD texture role requires RGB for color/normal or a scalar channel for roughness/metallic".into());
    }
    if (scalar || texture.role == UsdTextureRole::Normal) && texture.source_color_space != UsdTextureColorSpace::Raw {
        return Err("USD scalar and normal textures require raw color-space interpretation".into());
    }
    Ok(())
}

fn quote(value: &str) -> String {
    let escaped =
        value.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n").replace('\r', "\\r").replace('\t', "\\t");
    format!("\"{escaped}\"")
}

fn write_material(out: &mut String, material: &UsdMaterial) {
    let name = prim_name(&material.id);
    writeln!(out, "        def Material \"{name}\" (\n            customData = {{\n                string originalMaterialId = {}\n                string colorEncoding = \"scene-linear Rec.709/D65 RGB\"\n            }}\n        )\n        {{\n            token outputs:surface.connect = </World/Looks/{name}/PreviewSurface.outputs:surface>\n            def Shader \"PreviewSurface\"\n            {{\n                uniform token info:id = \"UsdPreviewSurface\"\n                int inputs:useSpecularWorkflow = 0", quote(&material.id)).unwrap();
    if material.textures.is_empty() {
        writeln!(out, "                color3f inputs:diffuseColor = {}\n                color3f inputs:emissiveColor = {}\n                float inputs:roughness = {}\n                float inputs:metallic = {}\n                float inputs:ior = {}\n                token outputs:surface\n            }}\n        }}", tuple3(material.diffuse_color), tuple3(material.emissive_color), material.roughness, material.metallic, material.ior).unwrap();
        return;
    }
    writeln!(out, "                color3f inputs:diffuseColor = {}\n                color3f inputs:emissiveColor = {}\n                float inputs:roughness = {}\n                float inputs:metallic = {}\n                float inputs:ior = {}\n                float inputs:opacity = 1", tuple3(material.diffuse_color), tuple3(material.emissive_color), material.roughness, material.metallic, material.ior).unwrap();
    for texture in &material.textures {
        let (input_type, input) = texture.role.input();
        let (_, output) = texture.channel.output();
        writeln!(out,"                {input_type} inputs:{input}.connect = </World/Looks/{name}/{input}Texture.outputs:{output}>").unwrap();
    }
    out.push_str("                token outputs:surface\n            }\n            def Shader \"PrimvarSt\"\n            {\n                uniform token info:id = \"UsdPrimvarReader_float2\"\n                string inputs:varname = \"st\"\n                float2 outputs:result\n            }\n");
    for texture in &material.textures {
        let (_, input) = texture.role.input();
        let (output_type, output) = texture.channel.output();
        let space = match texture.source_color_space {
            UsdTextureColorSpace::Raw => "raw",
            UsdTextureColorSpace::Srgb => "sRGB",
        };
        writeln!(out,"            def Shader \"{input}Texture\"\n            {{\n                uniform token info:id = \"UsdUVTexture\"\n                asset inputs:file = @{}@\n                float2 inputs:st.connect = </World/Looks/{name}/PrimvarSt.outputs:result>\n                token inputs:sourceColorSpace = \"{space}\"\n                token inputs:wrapS = \"{}\"\n                token inputs:wrapT = \"{}\"\n                float4 inputs:scale = {}\n                float4 inputs:bias = {}\n                {output_type} outputs:{output}\n            }}",texture.asset_path,texture.wrap_s.token(),texture.wrap_t.token(),tuple4(texture.scale),tuple4(texture.bias)).unwrap();
    }
    out.push_str("        }\n");
}

fn write_mesh(out: &mut String, mesh: &UsdMesh, materials: &[UsdMaterial]) {
    let name = prim_name(&mesh.id);
    let representation = match mesh.representation {
        UsdRepresentation::ComposedSdfBake => "composedSdfBake",
        UsdRepresentation::NativeSurface => "nativeSurfaceMeshCache",
    };
    writeln!(out, "        def Mesh \"{name}\" (\n            prepend apiSchemas = [\"MaterialBindingAPI\"]\n            customData = {{\n                string originalMeshId = {}\n                string[] originalEntityIds = [{}]\n                string representation = \"{representation}\"\n            }}\n        )\n        {{\n            uniform token subdivisionScheme = \"none\"\n            uniform token orientation = \"rightHanded\"\n            uniform bool doubleSided = {}\n            uniform token subsetFamily:materialBind:familyType = \"partition\"", quote(&mesh.id), mesh.source_entity_ids.iter().map(|id| quote(id)).collect::<Vec<_>>().join(", "), mesh.double_sided).unwrap();
    write_mesh_samples(out, "point3f[]", "points", &mesh.frames, true, |f| vector_array(&f.positions));
    let varying_topology = mesh
        .frames
        .iter()
        .skip(1)
        .any(|f| f.positions.len() != mesh.frames[0].positions.len() || f.triangles != mesh.frames[0].triangles);
    write_mesh_samples(out, "int[]", "faceVertexCounts", &mesh.frames, varying_topology, |f| {
        format!("[{}]", vec!["3"; f.triangles.len()].join(", "))
    });
    write_mesh_samples(out, "int[]", "faceVertexIndices", &mesh.frames, varying_topology, |f| {
        int_array(f.triangles.iter().flatten().copied())
    });
    if mesh.frames.iter().any(|frame| frame.corner_uvs.is_some()) {
        out.push_str(
            "            texCoord2f[] primvars:st (\n                interpolation = \"faceVarying\"\n            )\n",
        );
        let varying_uvs =
            varying_topology || mesh.frames.iter().skip(1).any(|frame| frame.corner_uvs != mesh.frames[0].corner_uvs);
        write_mesh_samples(out, "texCoord2f[]", "primvars:st", &mesh.frames, varying_uvs, |frame| {
            frame.corner_uvs.as_ref().map_or_else(
                || "None".into(),
                |uvs| {
                    format!(
                        "[{}]",
                        uvs.iter().flatten().map(|uv| format!("({}, {})", uv[0], uv[1])).collect::<Vec<_>>().join(", ")
                    )
                },
            )
        });
    }
    // Uniform polygon normals follow the supplied right-handed winding. They do
    // not invent smoothing across native crease or extracted material boundaries.
    out.push_str("            normal3f[] normals (\n                interpolation = \"uniform\"\n            )\n");
    write_mesh_samples(out, "normal3f[]", "normals", &mesh.frames, true, |f| {
        vector_array(&f.triangles.iter().map(|t| face_normal(f, t).expect("validated face")).collect::<Vec<_>>())
    });
    write_mesh_samples(out, "float3[]", "extent", &mesh.frames, true, |f| vector_array(&extent(f)));
    let used_materials: BTreeSet<_> = mesh.frames.iter().flat_map(|f| &f.material_ids).copied().collect();
    for material_id in used_materials {
        let material_name = prim_name(&materials[material_id].id);
        writeln!(out, "            def GeomSubset \"{material_name}\" (\n                prepend apiSchemas = [\"MaterialBindingAPI\"]\n            )\n            {{\n                uniform token elementType = \"face\"\n                uniform token familyName = \"materialBind\"\n                rel material:binding = </World/Looks/{material_name}>").unwrap();
        let indices = |f: &FrameMesh| {
            int_array(f.material_ids.iter().enumerate().filter_map(|(face, &id)| (id == material_id).then_some(face)))
        };
        writeln!(out, "                int[] indices = {}", indices(&mesh.frames[0])).unwrap();
        if varying_topology || mesh.frames.iter().skip(1).any(|f| f.material_ids != mesh.frames[0].material_ids) {
            out.push_str("                int[] indices.timeSamples = {\n");
            for frame in &mesh.frames {
                writeln!(out, "                    {}: {},", frame.time, indices(frame)).unwrap();
            }
            out.push_str("                }\n");
        }
        out.push_str("            }\n");
    }
    out.push_str("        }\n");
}

fn write_mesh_samples(
    out: &mut String,
    ty: &str,
    attr: &str,
    frames: &[FrameMesh],
    sampled: bool,
    value: impl Fn(&FrameMesh) -> String,
) {
    writeln!(out, "            {ty} {attr} = {}", value(&frames[0])).unwrap();
    if sampled {
        writeln!(out, "            {ty} {attr}.timeSamples = {{").unwrap();
        for frame in frames {
            writeln!(out, "                {}: {},", frame.time, value(frame)).unwrap();
        }
        out.push_str("            }\n");
    }
}

fn tuple3(v: [f32; 3]) -> String {
    format!("({}, {}, {})", v[0], v[1], v[2])
}
fn tuple4(v: [f32; 4]) -> String {
    format!("({}, {}, {}, {})", v[0], v[1], v[2], v[3])
}

fn vector_array(values: &[[f32; 3]]) -> String {
    format!("[{}]", values.iter().copied().map(tuple3).collect::<Vec<_>>().join(", "))
}

fn int_array<T: std::fmt::Display>(values: impl Iterator<Item = T>) -> String {
    format!("[{}]", values.map(|v| v.to_string()).collect::<Vec<_>>().join(", "))
}

fn extent(frame: &FrameMesh) -> [[f32; 3]; 2] {
    if frame.positions.is_empty() {
        return [[0.0; 3]; 2];
    }
    let mut result = [frame.positions[0]; 2];
    for point in &frame.positions[1..] {
        for (axis, value) in point.iter().enumerate() {
            result[0][axis] = result[0][axis].min(*value);
            result[1][axis] = result[1][axis].max(*value);
        }
    }
    result
}

fn as_f64(v: [f32; 3]) -> [f64; 3] {
    v.map(f64::from)
}
fn subtract(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn normalize(v: [f64; 3]) -> Option<[f64; 3]> {
    let length = v[0].hypot(v[1]).hypot(v[2]);
    (length.is_finite() && length > 0.0).then(|| v.map(|x| x / length))
}
fn face_normal(frame: &FrameMesh, triangle: &[u32; 3]) -> Option<[f32; 3]> {
    let [a, b, c] = triangle.map(|i| as_f64(frame.positions[i as usize]));
    normalize(cross(subtract(b, a), subtract(c, a))).map(|v| v.map(|x| x as f32))
}
fn camera_basis(frame: &CameraFrame) -> Option<[[f64; 3]; 3]> {
    let back = normalize(subtract(as_f64(frame.position), as_f64(frame.target)))?;
    let right = normalize(cross(as_f64(frame.up), back))?;
    let up = normalize(cross(back, right))?;
    Some([right, up, back])
}

// USD optics are measured in tenths of the world unit. In this meter stage,
// 0.24 represents a physical 24 mm vertical filmback, not 24 meters.
fn camera_focal_length(frame: &CameraFrame) -> f64 {
    0.12 / (f64::from(frame.vertical_fov_degrees).to_radians() * 0.5).tan()
}

fn camera_f_stop(frame: &CameraFrame) -> f64 {
    if frame.aperture_radius_m == 0.0 {
        0.0
    } else {
        camera_focal_length(frame) * 0.1 / (2.0 * f64::from(frame.aperture_radius_m))
    }
}

fn write_camera(out: &mut String, camera: &UsdCamera, fps: f64) {
    writeln!(out, "    def Camera \"{}\" (\n        customData = {{\n            string originalCameraId = {}\n        }}\n    )\n    {{\n        uniform token projection = \"perspective\"\n        uniform token[] xformOpOrder = [\"xformOp:transform\"]\n        float verticalAperture = 0.24\n        float horizontalApertureOffset = 0\n        float verticalApertureOffset = 0", prim_name(&camera.id), quote(&camera.id)).unwrap();
    let transform = |frame: &CameraFrame| {
        let [right, up, back] = camera_basis(frame).expect("validated camera");
        format!(
            "(({}, {}, {}, 0), ({}, {}, {}, 0), ({}, {}, {}, 0), ({}, {}, {}, 1))",
            right[0],
            right[1],
            right[2],
            up[0],
            up[1],
            up[2],
            back[0],
            back[1],
            back[2],
            frame.position[0],
            frame.position[1],
            frame.position[2]
        )
    };
    write_camera_samples(out, "matrix4d", "xformOp:transform", &camera.frames, transform);
    write_camera_samples(out, "float", "horizontalAperture", &camera.frames, |f| {
        (0.24 * f64::from(f.aspect_ratio)).to_string()
    });
    write_camera_samples(out, "float", "focalLength", &camera.frames, |f| camera_focal_length(f).to_string());
    write_camera_samples(out, "float2", "clippingRange", &camera.frames, |f| {
        format!("({}, {})", f.near_clip, f.far_clip)
    });
    write_camera_samples(out, "float", "fStop", &camera.frames, |f| camera_f_stop(f).to_string());
    write_camera_samples(out, "float", "focusDistance", &camera.frames, |f| f.focus_distance_m.to_string());
    write_camera_samples(out, "double", "shutter:open", &camera.frames, |f| (f.shutter_open_seconds * fps).to_string());
    write_camera_samples(out, "double", "shutter:close", &camera.frames, |f| {
        (f.shutter_close_seconds * fps).to_string()
    });
    out.push_str("    }\n");
}

fn write_camera_samples(
    out: &mut String,
    ty: &str,
    attr: &str,
    frames: &[CameraFrame],
    value: impl Fn(&CameraFrame) -> String,
) {
    writeln!(out, "        {ty} {attr} = {}\n        {ty} {attr}.timeSamples = {{", value(&frames[0])).unwrap();
    for frame in frames {
        writeln!(out, "            {}: {},", frame.time, value(frame)).unwrap();
    }
    out.push_str("        }\n");
}
