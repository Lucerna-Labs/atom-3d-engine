//! Real-adapter regression for the raw primary hit underlying film depth. This exercises the
//! generated GPU raymarch function directly, rather than inferring distance from display RGB.

use bytemuck::Zeroable;
use mm3e_kit::{
    march::Ray,
    vec::{Transform, Vec3},
};
use mm3e_orchestrator::{Object, Prim, Scene};
use wgpu::util::DeviceExt;

#[test]
#[ignore = "requires actual wgpu adapter; run with --ignored --nocapture"]
fn gpu_raw_plane_hit_matches_geometric_depth_and_inside_origin_policy() {
    let gpu = super::GpuRenderer::new().expect("a real wgpu adapter is required for plane depth validation");
    println!("Raw plane depth adapter: {}", gpu.adapter_name());
    let mut scene = Scene::new(1, 1);
    scene.marcher.eps = 0.00001;
    scene.objects.push(Object::new(Prim::Plane { n: Vec3::new(0.0, 0.0, 1.0), h: 0.0 }, Transform::IDENTITY, 0));
    let rays = [
        Ray { origin: Vec3::new(0.0, 0.0, 3.0), dir: Vec3::new(0.0, 0.0, -1.0) },
        Ray { origin: Vec3::new(1.0, 1.0, 3.0), dir: Vec3::new(0.2, 0.3, -1.0).normalize() },
        Ray { origin: Vec3::new(0.0, 0.0, -1.0), dir: Vec3::new(0.0, 0.0, -1.0) },
        Ray { origin: Vec3::new(0.0, 0.0, 3.0), dir: Vec3::new(0.0, 0.0, 1.0) },
    ];
    let inputs: Vec<[f32; 4]> = rays
        .iter()
        .flat_map(|ray| [[ray.origin.x, ray.origin.y, ray.origin.z, 0.0], [ray.dir.x, ray.dir.y, ray.dir.z, 0.0]])
        .collect();
    let mut source = super::wgsl::build_shader(&scene);
    source.push_str(
        r#"
@group(0) @binding(4) var<storage,read> film_probe_rays: array<vec4<f32>>;
@group(0) @binding(5) var<storage,read_write> film_probe_hits: array<vec4<f32>>;
@compute @workgroup_size(4) fn film_probe(@builtin(global_invocation_id) gid: vec3<u32>) {
  if (gid.x >= 4u) { return; }
  let origin = film_probe_rays[2u*gid.x].xyz;
  let direction = film_probe_rays[2u*gid.x+1u].xyz;
  let hit = raymarch(origin,direction);
  film_probe_hits[gid.x] = vec4<f32>(hit, (origin+direction*hit.x).z);
}
"#,
    );
    let device = &gpu.device;
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("raw-plane-depth"),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("raw-plane-depth"),
        layout: None,
        module: &module,
        entry_point: Some("film_probe"),
        compilation_options: Default::default(),
        cache: None,
    });
    let uniforms = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("raw-plane-uniforms"),
        contents: bytemuck::bytes_of(&super::Uniforms::zeroed()),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let input = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("raw-plane-rays"),
        contents: bytemuck::cast_slice(&inputs),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let size = (rays.len() * 4 * 4) as u64;
    let output = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("raw-plane-hits"),
        size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let readback = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("raw-plane-readback"),
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let bindings = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("raw-plane-bindings"),
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: uniforms.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 4, resource: input.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 5, resource: output.as_entire_binding() },
        ],
    });
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("raw-plane-depth") });
    {
        let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("raw-plane-depth"),
            timestamp_writes: None,
        });
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bindings, &[]);
        pass.dispatch_workgroups(1, 1, 1);
    }
    encoder.copy_buffer_to_buffer(&output, 0, &readback, 0, size);
    gpu.queue.submit(Some(encoder.finish()));
    let slice = readback.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |result| tx.send(result).unwrap());
    device.poll(wgpu::PollType::Wait).unwrap();
    rx.recv().unwrap().unwrap();
    let mapped = slice.get_mapped_range();
    let results: &[[f32; 4]] = bytemuck::cast_slice(&mapped);
    for i in 0..2 {
        let expected = 3.0 / -rays[i].dir.z;
        let cpu = scene.marcher.march(&|p: Vec3| mm3e_kit::Field::new(p.z, 0), &rays[i]);
        assert!(cpu.hit);
        assert!((cpu.t - expected).abs() < 0.0001);
        assert_eq!(results[i][2], 1.0);
        assert!((results[i][0] - expected).abs() < 0.0001);
        assert!(results[i][3].abs() < 0.0001);
        println!("ray {i}: expected t={expected}, CPU t={}, GPU t={}", cpu.t, results[i][0]);
    }
    assert_eq!(results[2][0], 0.0);
    assert_eq!(results[2][2], 1.0);
    assert_eq!(results[2][3], -1.0);
    assert_eq!(results[3][2], 0.0);
    drop(mapped);
    readback.unmap();
}
