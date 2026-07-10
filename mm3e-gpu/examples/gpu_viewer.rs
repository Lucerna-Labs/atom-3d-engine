#![cfg_attr(windows, windows_subsystem = "windows")]

//! Real-time **GPU** viewer: each frame is sphere-traced on the GPU (wgpu/Vulkan) and presented
//! through Win32/GDI — combining the verified GPU compute path with the dependency-light Win32
//! window (no winit). This is the interactive payoff of the GPU backend: fly around an SDF scene
//! in real time.
//!
//! Controls: arrow keys or left-drag to orbit, `W`/`S` to zoom, `U` to check for updates, `Esc` to quit.
//! Run (Windows desktop): cargo run -p mm3e-gpu --example gpu_viewer --release
//! Resolution: pass `480p` / `720p` / `1080p` / `1440p` / `4k` (or `WxH`) to render at that size,
//! e.g. `cargo run -p mm3e-gpu --example gpu_viewer --release -- 4k`. The live fps is in the title
//! bar. The interactive loop is capped at 120 FPS; benchmarks remain uncapped. (This Win32/GDI
//! viewer reads each frame back to the CPU to present it, so its fps reflects
//! render + readback; a swapchain-present engine would hit the higher render-only numbers from the
//! `gpu_resolution` benchmark.)

#[cfg(windows)]
use mm3e_gpu::GpuRenderer;
#[cfg(windows)]
use mm3e_kit::color::Material;
#[cfg(windows)]
use mm3e_kit::vec::{Mat3, Transform, Vec3};
#[cfg(windows)]
use mm3e_orchestrator::{Light, Object, Prim, Scene};

/// Resolution from the first non-flag CLI argument (a preset keyword or `WxH`); default 854×480.
#[cfg(windows)]
fn parse_res(args: &[String]) -> (u32, u32) {
    let pick = args.iter().skip(1).find(|a| !a.starts_with('-'));
    match pick.map(|s| s.to_lowercase()).as_deref() {
        Some("480p") => (854, 480),
        Some("720p") => (1280, 720),
        Some("1080p") => (1920, 1080),
        Some("1440p") => (2560, 1440),
        Some("4k") | Some("2160p") => (3840, 2160),
        Some(other) => {
            other.split_once('x').and_then(|(w, h)| Some((w.parse().ok()?, h.parse().ok()?))).unwrap_or((854, 480))
        }
        None => (854, 480),
    }
}

#[cfg(windows)]
fn build_scene(width: u32, height: u32) -> Scene {
    let mut scene = Scene::new(width, height);
    scene.aa = 1;
    scene.bounces = 2;
    scene.marcher.max_steps = 128;

    let floor = scene.material(Material::solid(Vec3::splat(1.0)).checkered().roughness(0.6));
    let red = scene.material(Material::solid(Vec3::new(0.85, 0.2, 0.22)).roughness(0.3).specular(0.8));
    let gold =
        scene.material(Material::solid(Vec3::new(0.95, 0.72, 0.28)).metallic(1.0).roughness(0.2).reflective(0.5));
    let chrome = scene.material(Material::solid(Vec3::splat(0.92)).metallic(1.0).roughness(0.06).reflective(0.85));

    scene.add(Object::new(Prim::Plane { n: Vec3::new(0.0, 1.0, 0.0), h: 0.0 }, Transform::IDENTITY, floor));
    scene.add(Object::new(Prim::Sphere { r: 1.0 }, Transform::at(Vec3::new(-2.2, 1.0, 0.0)), chrome));
    scene.add(Object::new(
        Prim::RoundBox { half: Vec3::splat(0.85), radius: 0.18 },
        Transform::at(Vec3::new(0.1, 0.95, -0.4)).rotated(Mat3::from_euler(0.0, 0.6, 0.0)),
        gold,
    ));
    scene.add(Object::new(Prim::Torus { major: 0.85, minor: 0.3 }, Transform::at(Vec3::new(2.5, 1.0, 0.4)), red));
    scene.sun_dir = Vec3::new(0.5, 0.75, 0.4).normalize();
    scene.light(Light::directional(scene.sun_dir, Vec3::new(1.2, 1.12, 0.98).scale(2.0)).soft(0.04));
    scene.light(Light::sphere(Vec3::new(-3.0, 4.0, 3.0), Vec3::splat(30.0), 1.0));
    scene
}

#[cfg(not(windows))]
fn main() {
    eprintln!("The GPU viewer's window uses Win32; on other platforms use `gpu_render` (offline).");
}

#[cfg(windows)]
fn main() {
    let (rw, rh) = parse_res(&std::env::args().collect::<Vec<_>>());
    let scene = build_scene(rw, rh);
    let renderer = match GpuRenderer::new() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("GPU init failed: {e}");
            std::process::exit(1);
        }
    };
    println!("GPU viewer on {} at {rw}x{rh}", renderer.adapter_name());
    let gpu_scene = renderer.compile(&scene, rw, rh);
    win32::run(&renderer, &gpu_scene, rw, rh);
}

#[cfg(windows)]
mod win32 {
    use lucerna_release_client::{StagedUpdate, UpdateStatus};
    use mm3e_gpu::{GpuRenderer, GpuScene};
    use mm3e_kit::vec::Vec3;
    use mm3e_orchestrator::{orbit_camera, wrap_orbit_yaw};
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
    #[derive(Clone, Copy)]
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
        fn ScreenToClient(hwnd: Hwnd, point: *mut Point) -> i32;
        fn GetForegroundWindow() -> Hwnd;
        fn LoadCursorW(instance: Hinstance, name: *const u16) -> *mut c_void;
        fn ShowWindow(hwnd: Hwnd, cmd: i32) -> i32;
        fn SetWindowTextW(hwnd: Hwnd, text: *const u16) -> i32;
        fn MessageBoxW(hwnd: Hwnd, text: *const u16, caption: *const u16, kind: u32) -> i32;
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
    const VK_LEFT: i32 = 0x25;
    const VK_UP: i32 = 0x26;
    const VK_RIGHT: i32 = 0x27;
    const VK_DOWN: i32 = 0x28;
    const VK_LBUTTON: i32 = 0x01;
    const VK_U: i32 = 0x55;
    const MB_OK: u32 = 0x0000;
    const MB_YESNO: u32 = 0x0004;
    const MB_ICONINFORMATION: u32 = 0x0040;
    const MB_ICONWARNING: u32 = 0x0030;
    const IDYES: i32 = 6;

    const UPDATE_MANIFEST: &str = include_str!("../../lucerna-update.json");

    enum UpdateEvent {
        Checked { manual: bool, result: Result<UpdateStatus, String> },
        Staged(Result<StagedUpdate, String>),
    }

    fn wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }
    fn down(key: i32) -> bool {
        unsafe { (GetAsyncKeyState(key) as u16 & 0x8000) != 0 }
    }

    fn start_update_check(sender: std::sync::mpsc::Sender<UpdateEvent>, manual: bool) {
        std::thread::spawn(move || {
            let result = lucerna_release_client::check_from_json(UPDATE_MANIFEST);
            let _ = sender.send(UpdateEvent::Checked { manual, result });
        });
    }

    unsafe fn message_box(hwnd: Hwnd, message: &str, kind: u32) -> i32 {
        MessageBoxW(hwnd, wide(message).as_ptr(), wide("Atom 3D Engine Updates").as_ptr(), kind)
    }

    unsafe fn point_in_client(hwnd: Hwnd, screen: Point) -> bool {
        let mut client = screen;
        if ScreenToClient(hwnd, &mut client) == 0 {
            return false;
        }
        let mut rect = Rect { left: 0, top: 0, right: 0, bottom: 0 };
        GetClientRect(hwnd, &mut rect);
        client.x >= rect.left && client.x < rect.right && client.y >= rect.top && client.y < rect.bottom
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

    // RGBA8 (from the GPU) -> BGRA for a 32-bit BI_RGB DIB.
    fn rgba_to_bgra(rgba: &[u8], out: &mut Vec<u32>) {
        out.clear();
        for px in rgba.chunks_exact(4) {
            out.push((px[2] as u32) | ((px[1] as u32) << 8) | ((px[0] as u32) << 16));
        }
    }

    pub fn run(renderer: &GpuRenderer, scene: &GpuScene, rw: u32, rh: u32) {
        const TARGET_FPS: u32 = 120;
        const FRAME_TIME: std::time::Duration = std::time::Duration::from_nanos(1_000_000_000 / TARGET_FPS as u64);
        const SPIN_THRESHOLD: std::time::Duration = std::time::Duration::from_micros(500);
        const MIN_ORBIT_PITCH: f32 = 0.02;
        const MAX_ORBIT_PITCH: f32 = 1.45;

        unsafe {
            let instance = GetModuleHandleW(std::ptr::null());
            let class_name = wide("mm3e_gpu_viewer");
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
                eprintln!("RegisterClassW failed");
                return;
            }
            let title = wide("MM3E - real-time GPU SDF viewer (arrows/drag orbit, W/S zoom, U updates, Esc quit)");
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
                eprintln!("CreateWindowExW failed");
                return;
            }
            ShowWindow(hwnd, SW_SHOW);

            let mut yaw = 0.5f32;
            let mut pitch = 0.3f32;
            let mut radius = 8.5f32;
            let mut last = Point { x: 0, y: 0 };
            GetCursorPos(&mut last);
            let mut dragging = false;
            let mut bgra: Vec<u32> = Vec::with_capacity((rw * rh) as usize);

            // Live fps in the title bar (refreshed twice a second).
            let mut frames = 0u32;
            let mut fps_clock = std::time::Instant::now();
            let mut next_frame = std::time::Instant::now();
            let (update_sender, update_receiver) = std::sync::mpsc::channel::<UpdateEvent>();
            let check_on_startup = lucerna_release_client::parse_app_manifest(UPDATE_MANIFEST)
                .map(|manifest| manifest.check_on_startup)
                .unwrap_or(false);
            let mut update_busy = check_on_startup;
            let mut update_key_was_down = false;
            if check_on_startup {
                start_update_check(update_sender.clone(), false);
            }

            let mut msg = std::mem::zeroed::<Msg>();
            'frame: loop {
                while PeekMessageW(&mut msg, std::ptr::null_mut(), 0, 0, PM_REMOVE) != 0 {
                    if msg.message == WM_QUIT {
                        break 'frame;
                    }
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
                let focused = GetForegroundWindow() == hwnd;
                if focused && down(VK_ESCAPE) {
                    break 'frame;
                }
                let update_key_down = focused && down(VK_U);
                if update_key_down && !update_key_was_down && !update_busy {
                    update_busy = true;
                    start_update_check(update_sender.clone(), true);
                }
                update_key_was_down = update_key_down;

                while let Ok(event) = update_receiver.try_recv() {
                    update_busy = false;
                    match event {
                        UpdateEvent::Checked { manual, result: Ok(UpdateStatus::UpToDate) } => {
                            if manual {
                                message_box(hwnd, "You already have the latest version.", MB_OK | MB_ICONINFORMATION);
                            }
                        }
                        UpdateEvent::Checked { result: Ok(UpdateStatus::Available(update)), .. } => {
                            let prompt = format!(
                                "Atom 3D Engine {} is available.\n\nInstalled: {}\n\nDownload and install it now?",
                                update.release.version, update.app.current_version
                            );
                            if message_box(hwnd, &prompt, MB_YESNO | MB_ICONINFORMATION) == IDYES {
                                update_busy = true;
                                let sender = update_sender.clone();
                                std::thread::spawn(move || {
                                    let result = lucerna_release_client::stage(&update);
                                    let _ = sender.send(UpdateEvent::Staged(result));
                                });
                            }
                        }
                        UpdateEvent::Checked { manual, result: Err(error) } => {
                            eprintln!("Update check failed: {error}");
                            if manual {
                                message_box(
                                    hwnd,
                                    &format!("Could not check for updates.\n\n{error}"),
                                    MB_OK | MB_ICONWARNING,
                                );
                            }
                        }
                        UpdateEvent::Staged(Ok(staged)) => {
                            let prompt = format!(
                                "Version {} was downloaded and verified.\n\nRestart now to finish installing it?",
                                staged.version
                            );
                            if message_box(hwnd, &prompt, MB_YESNO | MB_ICONINFORMATION) == IDYES {
                                match std::env::current_exe().map_err(|error| error.to_string()).and_then(|path| {
                                    let install_dir = path
                                        .parent()
                                        .ok_or_else(|| "the executable has no parent directory".to_string())?;
                                    lucerna_release_client::schedule_install(&staged, install_dir, std::process::id())
                                }) {
                                    Ok(()) => break 'frame,
                                    Err(error) => {
                                        message_box(
                                            hwnd,
                                            &format!("Could not schedule the update.\n\n{error}"),
                                            MB_OK | MB_ICONWARNING,
                                        );
                                    }
                                }
                            }
                        }
                        UpdateEvent::Staged(Err(error)) => {
                            message_box(
                                hwnd,
                                &format!("The update was not installed.\n\n{error}"),
                                MB_OK | MB_ICONWARNING,
                            );
                        }
                    }
                }
                if focused && down(VK_LEFT) {
                    yaw -= 0.04;
                }
                if focused && down(VK_RIGHT) {
                    yaw += 0.04;
                }
                if focused && down(VK_UP) {
                    pitch = (pitch + 0.03).min(MAX_ORBIT_PITCH);
                }
                if focused && down(VK_DOWN) {
                    pitch = (pitch - 0.03).max(MIN_ORBIT_PITCH);
                }
                if focused && down(0x57) {
                    radius = (radius - 0.15).max(2.5);
                }
                if focused && down(0x53) {
                    radius = (radius + 0.15).min(30.0);
                }
                let mut cur = Point { x: 0, y: 0 };
                GetCursorPos(&mut cur);
                if focused && down(VK_LBUTTON) && point_in_client(hwnd, cur) {
                    if dragging {
                        yaw += (cur.x - last.x) as f32 * 0.01;
                        pitch = (pitch - (cur.y - last.y) as f32 * 0.01).clamp(MIN_ORBIT_PITCH, MAX_ORBIT_PITCH);
                    }
                    dragging = true;
                } else {
                    dragging = false;
                }
                last = cur;
                yaw = wrap_orbit_yaw(yaw);

                // Render this frame on the GPU, then present via GDI.
                let cam = orbit_camera(Vec3::new(0.0, 0.85, 0.4), radius, yaw, pitch, 52f32.to_radians());
                let rgba = scene.render_rgba(renderer, &cam);
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

                frames += 1;
                let secs = fps_clock.elapsed().as_secs_f32();
                if secs >= 0.5 {
                    let fps = frames as f32 / secs;
                    let title = wide(&format!(
                        "MM3E - GPU SDF viewer - {rw}x{rh} - {fps:.0} fps - 120 fps cap (arrows/drag orbit, W/S zoom, U updates, Esc quit)"
                    ));
                    SetWindowTextW(hwnd, title.as_ptr());
                    frames = 0;
                    fps_clock = std::time::Instant::now();
                }
                next_frame += FRAME_TIME;
                let now = std::time::Instant::now();
                if next_frame > now {
                    let remaining = next_frame - now;
                    if remaining > SPIN_THRESHOLD {
                        std::thread::sleep(remaining - SPIN_THRESHOLD);
                    }
                    while std::time::Instant::now() < next_frame {
                        std::hint::spin_loop();
                    }
                } else {
                    next_frame = now;
                }
            }
        }
    }
}
