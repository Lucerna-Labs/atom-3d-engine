//! mm3e-gpu — the GPU backend. Codegens the SDF world field into a WGSL compute shader (see
//! [`wgsl`]) and runs it on wgpu/Vulkan/Metal/DX12. The core engine stays dependency-free; this
//! crate is the opt-in real-time path. One workgroup invocation shades one pixel — the per-pixel
//! independence the CPU renderer already had ports almost perfectly to the GPU.
//!
//! Usage: [`GpuRenderer::new`] once, then [`GpuRenderer::compile`] a scene into a [`GpuScene`],
//! then [`GpuScene::render`] per camera (a fixed scene only re-uploads the camera uniform, so a
//! real-time loop never recompiles the shader).

pub mod wgsl;

use mm3e_kit::camera::Camera;
use mm3e_kit::color::Rgba;
use mm3e_kit::framebuffer::Framebuffer;
use mm3e_orchestrator::Scene;

const MAX_DYN: usize = 12;

/// A dynamic sphere (player / physics body) rendered without recompiling the shader — its data
/// rides in the uniform and is unioned into the field on the GPU each frame.
#[derive(Clone, Copy, Debug)]
pub struct DynSphere {
    pub pos: mm3e_kit::vec::Vec3,
    pub radius: f32,
    pub albedo: mm3e_kit::vec::Vec3,
    pub metallic: f32,
}

/// The camera + frame uniform, matching `struct U` in the shader (std140 16-byte rows).
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    eye: [f32; 3],
    fov: f32,
    right: [f32; 3],
    aa: f32,
    up: [f32; 3],
    bounces: f32,
    fwd: [f32; 3],
    n_dyn: f32,
    res: [f32; 2],
    _pad2: [f32; 2],
    dyn_pr: [[f32; 4]; MAX_DYN],
    dyn_col: [[f32; 4]; MAX_DYN],
}

fn align_up(v: u32, a: u32) -> u32 {
    v.next_multiple_of(a)
}

/// A GPU device + queue. Create once; compile many scenes.
pub struct GpuRenderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    adapter_name: String,
}

/// Return a human-readable description of the GPU adapter wgpu selects, or an error string.
pub fn adapter_info() -> Result<String, String> {
    GpuRenderer::new().map(|r| r.adapter_name)
}

impl GpuRenderer {
    pub fn new() -> Result<GpuRenderer, String> {
        pollster::block_on(async {
            let instance = wgpu::Instance::default();
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    force_fallback_adapter: false,
                    compatible_surface: None,
                })
                .await
                .map_err(|e| format!("no GPU adapter: {e}"))?;
            let info = adapter.get_info();
            let (device, queue) = adapter
                .request_device(&wgpu::DeviceDescriptor { label: Some("mm3e-gpu"), ..Default::default() })
                .await
                .map_err(|e| format!("request_device failed: {e}"))?;
            Ok(GpuRenderer {
                device,
                queue,
                adapter_name: format!("{} ({:?}, {:?})", info.name, info.device_type, info.backend),
            })
        })
    }

    pub fn adapter_name(&self) -> &str {
        &self.adapter_name
    }

    /// Compile `scene` into a GPU pipeline + render targets at `width × height`.
    pub fn compile(&self, scene: &Scene, width: u32, height: u32) -> GpuScene {
        let device = &self.device;
        let source = wgsl::build_shader(scene);
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("mm3e-sdf"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });

        let uniform_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("uniforms"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("out"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = tex.create_view(&wgpu::TextureViewDescriptor::default());

        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("bgl"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bg"),
            layout: &bgl,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: uniform_buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&view) },
            ],
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("pl"),
            bind_group_layouts: &[&bgl],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("sdf"),
            layout: Some(&layout),
            module: &module,
            entry_point: Some("main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        });

        let bpr = align_up(width * 4, wgpu::COPY_BYTES_PER_ROW_ALIGNMENT);
        let readback = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("readback"),
            size: (bpr * height) as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        GpuScene {
            pipeline,
            bind_group,
            uniform_buf,
            tex,
            readback,
            width,
            height,
            bpr,
            aa: scene.aa.max(1),
            bounces: scene.bounces,
        }
    }
}

/// A compiled scene: pipeline + render targets. Re-render with a new camera each frame.
pub struct GpuScene {
    pipeline: wgpu::ComputePipeline,
    bind_group: wgpu::BindGroup,
    uniform_buf: wgpu::Buffer,
    tex: wgpu::Texture,
    readback: wgpu::Buffer,
    width: u32,
    height: u32,
    bpr: u32,
    aa: u32,
    bounces: u32,
}

impl GpuScene {
    fn uniforms(&self, camera: &Camera, dyn_spheres: &[DynSphere]) -> Uniforms {
        let mut dyn_pr = [[0.0f32; 4]; MAX_DYN];
        let mut dyn_col = [[0.0f32; 4]; MAX_DYN];
        let n = dyn_spheres.len().min(MAX_DYN);
        for (i, s) in dyn_spheres.iter().take(MAX_DYN).enumerate() {
            dyn_pr[i] = [s.pos.x, s.pos.y, s.pos.z, s.radius];
            dyn_col[i] = [s.albedo.x, s.albedo.y, s.albedo.z, s.metallic];
        }
        Uniforms {
            eye: [camera.eye.x, camera.eye.y, camera.eye.z],
            fov: camera.fov_scale,
            right: [camera.right.x, camera.right.y, camera.right.z],
            aa: self.aa as f32,
            up: [camera.up.x, camera.up.y, camera.up.z],
            bounces: self.bounces as f32,
            fwd: [camera.forward.x, camera.forward.y, camera.forward.z],
            n_dyn: n as f32,
            res: [self.width as f32, self.height as f32],
            _pad2: [0.0, 0.0],
            dyn_pr,
            dyn_col,
        }
    }

    /// Dispatch the compute shader for `camera` (+ dynamic spheres) into the readback buffer.
    fn dispatch(&self, r: &GpuRenderer, camera: &Camera, dyn_spheres: &[DynSphere]) {
        r.queue.write_buffer(&self.uniform_buf, 0, bytemuck::bytes_of(&self.uniforms(camera, dyn_spheres)));
        let mut enc = r.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("enc") });
        {
            let mut pass =
                enc.begin_compute_pass(&wgpu::ComputePassDescriptor { label: Some("sdf"), timestamp_writes: None });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.dispatch_workgroups(self.width.div_ceil(8), self.height.div_ceil(8), 1);
        }
        enc.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &self.tex,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &self.readback,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(self.bpr),
                    rows_per_image: Some(self.height),
                },
            },
            wgpu::Extent3d { width: self.width, height: self.height, depth_or_array_layers: 1 },
        );
        r.queue.submit([enc.finish()]);
    }

    /// Render `camera` and return the unpadded RGBA8 pixel rows (row-major, top-down).
    pub fn render_rgba(&self, r: &GpuRenderer, camera: &Camera) -> Vec<u8> {
        self.render_rgba_dyn(r, camera, &[])
    }

    /// Render `camera` with extra dynamic spheres (player / physics bodies) unioned into the field.
    pub fn render_rgba_dyn(&self, r: &GpuRenderer, camera: &Camera, dyn_spheres: &[DynSphere]) -> Vec<u8> {
        self.dispatch(r, camera, dyn_spheres);
        let slice = self.readback.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        r.device.poll(wgpu::PollType::Wait).expect("device poll");
        let data = slice.get_mapped_range();
        let mut out = Vec::with_capacity((self.width * self.height * 4) as usize);
        for y in 0..self.height {
            let row = (y * self.bpr) as usize;
            out.extend_from_slice(&data[row..row + (self.width * 4) as usize]);
        }
        drop(data);
        self.readback.unmap();
        out
    }

    /// Render `camera` into a [`Framebuffer`] (so the kit's BMP encoder can save it).
    pub fn render(&self, r: &GpuRenderer, camera: &Camera) -> Framebuffer {
        let rgba = self.render_rgba(r, camera);
        let mut fb = Framebuffer::new(self.width, self.height, Rgba::new(0.0, 0.0, 0.0, 1.0));
        for y in 0..self.height {
            for x in 0..self.width {
                let i = ((y * self.width + x) * 4) as usize;
                let c = Rgba::new(rgba[i] as f32 / 255.0, rgba[i + 1] as f32 / 255.0, rgba[i + 2] as f32 / 255.0, 1.0);
                fb.put(x, y, c);
            }
        }
        fb
    }
}
