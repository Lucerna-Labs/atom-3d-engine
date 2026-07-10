//! A playable game: roll a ball around an SDF obstacle course. Real GPU rendering + SDF-native
//! physics + input + a follow camera + a fixed-timestep game loop. The static level is compiled
//! once into a WGSL compute shader; the player and the loose balls are dynamic spheres unioned on
//! the GPU and simulated by `mm3e_orchestrator::physics` against the level's distance field.
//!
//! Controls: W/A/S/D move (camera-relative), mouse-drag to look, Space to jump, Esc to quit.
//! Run (Windows desktop): cargo run -p mm3e-gpu --example game --release

use mm3e_gpu::{DynSphere, GpuRenderer};
use mm3e_kit::color::Material;
use mm3e_kit::vec::{Mat3, Transform, Vec3};
use mm3e_orchestrator::{Light, Object, Prim, Scene};

/// The static level (floor + obstacles). Physics collides against this; it is baked into the shader.
fn build_level() -> Scene {
    let mut scene = Scene::new(854, 480);
    scene.aa = 1;
    scene.bounces = 1;
    scene.marcher.max_steps = 128;

    let floor = scene.material(Material::solid(Vec3::splat(1.0)).checkered().roughness(0.7));
    let stone = scene.material(Material::solid(Vec3::new(0.5, 0.52, 0.58)).roughness(0.6));
    let gold =
        scene.material(Material::solid(Vec3::new(0.95, 0.72, 0.28)).metallic(1.0).roughness(0.25).reflective(0.4));

    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, floor));
    // A boundary ring of pillars + a few blocks and a ramp to roll up.
    for k in 0..8 {
        let a = k as f32 / 8.0 * std::f32::consts::TAU;
        let p = Vec3::new(a.cos() * 9.0, 1.2, a.sin() * 9.0);
        scene.add(Object::new(Prim::Cylinder { h: 1.2, r: 0.5 }, Transform::at(p), stone));
    }
    scene.add(Object::new(
        Prim::Box { half: Vec3::new(1.2, 0.6, 1.2) },
        Transform::at(Vec3::new(2.5, 0.6, -1.5)),
        stone,
    ));
    scene.add(Object::new(
        Prim::Box { half: Vec3::new(0.8, 0.8, 0.8) },
        Transform::at(Vec3::new(-3.0, 0.8, 1.5)),
        stone,
    ));
    scene.add(Object::new(
        Prim::Box { half: Vec3::new(2.0, 0.15, 1.2) },
        Transform::at(Vec3::new(-1.0, 0.7, -3.5)).rotated(Mat3::from_euler(0.0, 0.0, 0.45)),
        gold,
    ));

    scene.sun_dir = Vec3::new(0.5, 0.8, 0.35).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::new(1.2, 1.14, 0.98).scale(2.1)).soft(0.05));
    scene.light(Light::sphere(Vec3::new(-4.0, 6.0, 4.0), Vec3::splat(40.0), 1.0));
    scene
}

#[cfg(not(windows))]
fn main() {
    eprintln!("The game window uses Win32; build it elsewhere, but run it on a Windows desktop.");
}

#[cfg(windows)]
fn main() {
    let level = build_level();
    let renderer = match GpuRenderer::new() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("GPU init failed: {e}");
            std::process::exit(1);
        }
    };
    println!("Game running on {}", renderer.adapter_name());
    let gpu = renderer.compile(&level, level.width, level.height);
    let field = level.field();
    win32::run(&renderer, &gpu, &field, level.width, level.height);
}

#[cfg(windows)]
mod win32 {
    use super::*;
    use mm3e_gpu::{GpuRenderer, GpuScene};
    use mm3e_kit::camera::Camera;
    use mm3e_kit::sdf::Field;
    use mm3e_orchestrator::particles::Particles;
    use mm3e_orchestrator::physics::{Body, PhysicsWorld};
    use std::ffi::c_void;

    type Hwnd = *mut c_void;
    type Hinstance = *mut c_void;
    type Hdc = *mut c_void;

    #[repr(C)]
    struct WndClassW {
        style: u32,
        wndproc: Option<unsafe extern "system" fn(Hwnd, u32, usize, isize) -> isize>,
        cls_extra: i32,
        wnd_extra: i32,
        instance: Hinstance,
        icon: *mut c_void,
        cursor: *mut c_void,
        background: *mut c_void,
        menu_name: *const u16,
        class_name: *const u16,
    }
    #[repr(C)]
    struct Point {
        x: i32,
        y: i32,
    }
    #[repr(C)]
    struct Rect {
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
    }
    #[repr(C)]
    struct Msg {
        hwnd: Hwnd,
        message: u32,
        wparam: usize,
        lparam: isize,
        time: u32,
        pt: Point,
    }
    #[repr(C)]
    struct BitmapInfoHeader {
        size: u32,
        width: i32,
        height: i32,
        planes: u16,
        bit_count: u16,
        compression: u32,
        size_image: u32,
        x_ppm: i32,
        y_ppm: i32,
        clr_used: u32,
        clr_important: u32,
    }

    #[link(name = "user32")]
    extern "system" {
        fn RegisterClassW(c: *const WndClassW) -> u16;
        #[allow(clippy::too_many_arguments)]
        fn CreateWindowExW(
            ex_style: u32,
            class_name: *const u16,
            window_name: *const u16,
            style: u32,
            x: i32,
            y: i32,
            w: i32,
            h: i32,
            parent: Hwnd,
            menu: *mut c_void,
            instance: Hinstance,
            param: *mut c_void,
        ) -> Hwnd;
        fn DefWindowProcW(hwnd: Hwnd, msg: u32, w: usize, l: isize) -> isize;
        fn PeekMessageW(msg: *mut Msg, hwnd: Hwnd, min: u32, max: u32, remove: u32) -> i32;
        fn TranslateMessage(msg: *const Msg) -> i32;
        fn DispatchMessageW(msg: *const Msg) -> isize;
        fn PostQuitMessage(code: i32);
        fn DestroyWindow(hwnd: Hwnd) -> i32;
        fn GetDC(hwnd: Hwnd) -> Hdc;
        fn ReleaseDC(hwnd: Hwnd, hdc: Hdc) -> i32;
        fn GetClientRect(hwnd: Hwnd, r: *mut Rect) -> i32;
        fn GetAsyncKeyState(key: i32) -> i16;
        fn GetCursorPos(p: *mut Point) -> i32;
        fn LoadCursorW(instance: Hinstance, name: *const u16) -> *mut c_void;
        fn ShowWindow(hwnd: Hwnd, cmd: i32) -> i32;
    }
    #[link(name = "gdi32")]
    extern "system" {
        #[allow(clippy::too_many_arguments)]
        fn StretchDIBits(
            hdc: Hdc,
            x_dst: i32,
            y_dst: i32,
            w_dst: i32,
            h_dst: i32,
            x_src: i32,
            y_src: i32,
            w_src: i32,
            h_src: i32,
            bits: *const c_void,
            bmi: *const BitmapInfoHeader,
            usage: u32,
            rop: u32,
        ) -> i32;
    }
    #[link(name = "kernel32")]
    extern "system" {
        fn GetModuleHandleW(name: *const u16) -> Hinstance;
        fn Sleep(ms: u32);
    }

    const WS_OVERLAPPEDWINDOW: u32 = 0x00CF_0000;
    const WS_VISIBLE: u32 = 0x1000_0000;
    const CW_USEDEFAULT: i32 = i32::MIN;
    const SW_SHOW: i32 = 5;
    const PM_REMOVE: u32 = 1;
    const WM_QUIT: u32 = 0x0012;
    const WM_DESTROY: u32 = 0x0002;
    const WM_CLOSE: u32 = 0x0010;
    const SRCCOPY: u32 = 0x00CC_0020;
    const IDC_ARROW: u16 = 32512;
    const VK_ESCAPE: i32 = 0x1B;
    const VK_SPACE: i32 = 0x20;
    const VK_LBUTTON: i32 = 0x01;

    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }
    fn down(key: i32) -> bool {
        unsafe { (GetAsyncKeyState(key) as u16 & 0x8000) != 0 }
    }

    unsafe extern "system" fn wndproc(hwnd: Hwnd, msg: u32, w: usize, l: isize) -> isize {
        match msg {
            WM_CLOSE => {
                DestroyWindow(hwnd);
                0
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                0
            }
            _ => DefWindowProcW(hwnd, msg, w, l),
        }
    }

    fn rgba_to_bgra(rgba: &[u8], out: &mut Vec<u32>) {
        out.clear();
        for px in rgba.chunks_exact(4) {
            out.push((px[2] as u32) | ((px[1] as u32) << 8) | ((px[0] as u32) << 16));
        }
    }

    pub fn run(renderer: &GpuRenderer, scene: &GpuScene, field: &dyn Fn(Vec3) -> Field, rw: u32, rh: u32) {
        // Game state: a player ball plus a few loose balls to bump around.
        let mut world = PhysicsWorld::new();
        let player = world.add(Body::new(Vec3::new(0.0, 1.0, 4.0), 0.5));
        let ball_colors = [
            Vec3::new(0.85, 0.2, 0.2),
            Vec3::new(0.2, 0.7, 0.3),
            Vec3::new(0.95, 0.6, 0.15),
            Vec3::new(0.6, 0.35, 0.85),
        ];
        for (i, _) in ball_colors.iter().enumerate() {
            let a = i as f32 * 1.6;
            world.add(Body::new(Vec3::new(a.cos() * 2.5, 1.5 + i as f32 * 0.4, a.sin() * 2.5), 0.45));
        }

        // Collectible gold orbs scattered around the level — roll into one to score (with a burst).
        let mut collectibles: Vec<(Vec3, bool)> = Vec::new();
        for k in 0..6 {
            let a = k as f32 / 6.0 * std::f32::consts::TAU + 0.4;
            collectibles.push((Vec3::new(a.cos() * 6.0, 0.6, a.sin() * 6.0), false));
        }
        let mut particles = Particles::new();
        let mut score = 0u32;
        let mut burst_seed = 1u32;

        let mut cam_yaw = 1.2f32;
        let mut cam_pitch = 0.35f32;
        let mut last = Point { x: 0, y: 0 };
        let mut dragging = false;

        unsafe {
            let instance = GetModuleHandleW(std::ptr::null());
            let class_name = wide("mm3e_game");
            let cursor = LoadCursorW(std::ptr::null_mut(), IDC_ARROW as *const u16);
            let wc = WndClassW {
                style: 0,
                wndproc: Some(wndproc),
                cls_extra: 0,
                wnd_extra: 0,
                instance,
                icon: std::ptr::null_mut(),
                cursor,
                background: std::ptr::null_mut(),
                menu_name: std::ptr::null(),
                class_name: class_name.as_ptr(),
            };
            if RegisterClassW(&wc) == 0 {
                return;
            }
            let title = wide("MM3E — roll the ball (WASD move, drag to look, Space jump, Esc quit)");
            let hwnd = CreateWindowExW(
                0,
                class_name.as_ptr(),
                title.as_ptr(),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                1024,
                600,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                instance,
                std::ptr::null_mut(),
            );
            if hwnd.is_null() {
                return;
            }
            ShowWindow(hwnd, SW_SHOW);
            GetCursorPos(&mut last);

            let mut bgra: Vec<u32> = Vec::with_capacity((rw * rh) as usize);
            let dt = 1.0 / 60.0;
            let mut msg = std::mem::zeroed::<Msg>();
            'frame: loop {
                while PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE) != 0 {
                    if msg.message == WM_QUIT {
                        break 'frame;
                    }
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
                if down(VK_ESCAPE) {
                    break 'frame;
                }

                // Mouse-drag camera look.
                let mut cur = Point { x: 0, y: 0 };
                GetCursorPos(&mut cur);
                if down(VK_LBUTTON) {
                    if dragging {
                        cam_yaw = mm3e_orchestrator::wrap_orbit_yaw(cam_yaw + (cur.x - last.x) as f32 * 0.01);
                        cam_pitch = (cam_pitch - (cur.y - last.y) as f32 * 0.01).clamp(0.08, 1.4);
                    }
                    dragging = true;
                } else {
                    dragging = false;
                }
                last = cur;

                // Camera-relative movement basis (horizontal).
                let fwd = Vec3::new(-cam_yaw.cos(), 0.0, -cam_yaw.sin());
                let right = Vec3::new(-cam_yaw.sin(), 0.0, cam_yaw.cos());
                let mut mv = Vec3::ZERO;
                if down(0x57) {
                    mv = mv + fwd;
                } // W
                if down(0x53) {
                    mv = mv - fwd;
                } // S
                if down(0x44) {
                    mv = mv + right;
                } // D
                if down(0x41) {
                    mv = mv - right;
                } // A
                {
                    let p = &mut world.bodies[player];
                    if mv.length() > 0.01 {
                        p.drive(mv.normalize(), 6.0);
                    }
                    if down(VK_SPACE) {
                        p.jump(7.0);
                    }
                }

                // Step physics against the static level field.
                world.step(dt, field);
                // Respawn anything that falls off the world.
                for b in world.bodies.iter_mut() {
                    if b.pos.y < -20.0 {
                        b.pos = Vec3::new(0.0, 4.0, 0.0);
                        b.vel = Vec3::ZERO;
                    }
                }

                // Collect orbs the player rolls into: score + a particle burst.
                let player_pos = world.bodies[player].pos;
                let player_r = world.bodies[player].radius;
                let total = collectibles.len();
                for c in collectibles.iter_mut() {
                    if !c.1 && (c.0 - player_pos).length() < player_r + 0.45 {
                        c.1 = true;
                        score += 1;
                        particles.burst(c.0, 14, 4.5, Vec3::new(1.0, 0.82, 0.3), burst_seed);
                        burst_seed = burst_seed.wrapping_add(101);
                        println!("collected! score = {score}/{total}");
                        if score as usize == total {
                            println!("all orbs collected — you win!");
                        }
                    }
                }
                particles.update(dt);

                let pp = world.bodies[player].pos;
                let eye = pp
                    + Vec3::new(
                        cam_yaw.cos() * 7.0 * cam_pitch.cos(),
                        7.0 * cam_pitch.sin() + 1.0,
                        cam_yaw.sin() * 7.0 * cam_pitch.cos(),
                    );
                let cam =
                    Camera::look_at(eye, pp + Vec3::new(0.0, 0.3, 0.0), Vec3::new(0.0, 1.0, 0.0), 55f32.to_radians());

                // Dynamic spheres: player (white) + the loose balls.
                let mut dyn_spheres = Vec::new();
                dyn_spheres.push(DynSphere {
                    pos: world.bodies[player].pos,
                    radius: world.bodies[player].radius,
                    albedo: Vec3::new(0.95, 0.95, 0.97),
                    metallic: 0.1,
                });
                for (i, c) in ball_colors.iter().enumerate() {
                    let b = world.bodies[player + 1 + i];
                    dyn_spheres.push(DynSphere { pos: b.pos, radius: b.radius, albedo: *c, metallic: 0.0 });
                }
                // Uncollected gold orbs.
                for c in &collectibles {
                    if !c.1 {
                        dyn_spheres.push(DynSphere {
                            pos: c.0,
                            radius: 0.35,
                            albedo: Vec3::new(1.0, 0.82, 0.3),
                            metallic: 1.0,
                        });
                    }
                }
                // Live particles fill any remaining dynamic-sphere slots.
                for p in particles.alive() {
                    if dyn_spheres.len() >= 24 {
                        break;
                    }
                    dyn_spheres.push(DynSphere { pos: p.pos, radius: p.size, albedo: p.color, metallic: 0.0 });
                }

                let mut rgba = scene.render_rgba_dyn(renderer, &cam, &dyn_spheres);
                // HUD overlay (drawn on the CPU onto the GPU frame before present).
                mm3e_kit::font::draw_text(
                    &mut rgba,
                    rw,
                    rh,
                    14,
                    12,
                    3,
                    &format!("SCORE {score}/{total}"),
                    [255, 226, 90],
                );
                if score as usize == total {
                    mm3e_kit::font::draw_text(
                        &mut rgba,
                        rw,
                        rh,
                        (rw as i32) / 2 - 120,
                        (rh as i32) / 2 - 20,
                        5,
                        "YOU WIN!",
                        [120, 255, 140],
                    );
                }
                rgba_to_bgra(&rgba, &mut bgra);

                let bmi = BitmapInfoHeader {
                    size: std::mem::size_of::<BitmapInfoHeader>() as u32,
                    width: rw as i32,
                    height: -(rh as i32),
                    planes: 1,
                    bit_count: 32,
                    compression: 0,
                    size_image: 0,
                    x_ppm: 0,
                    y_ppm: 0,
                    clr_used: 0,
                    clr_important: 0,
                };
                let mut rc = Rect { left: 0, top: 0, right: 0, bottom: 0 };
                GetClientRect(hwnd, &mut rc);
                let hdc = GetDC(hwnd);
                StretchDIBits(
                    hdc,
                    0,
                    0,
                    rc.right - rc.left,
                    rc.bottom - rc.top,
                    0,
                    0,
                    rw as i32,
                    rh as i32,
                    bgra.as_ptr() as *const c_void,
                    &bmi,
                    0,
                    SRCCOPY,
                );
                ReleaseDC(hwnd, hdc);
                Sleep(1);
            }
        }
    }
}
