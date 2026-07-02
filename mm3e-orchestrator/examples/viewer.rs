//! Adaptive real-time CPU viewer — a live orbit window driven by the CPU raymarcher, using raw
//! Win32 + GDI through `extern "system"` FFI (no winit, no softbuffer).
//!
//! The CPU path is treated as a progressive previewer, not a brute-force real-timer. While the
//! camera moves it renders at a low internal resolution with the `fast` quality preset and
//! auto-tunes that resolution to hold a frame budget (dynamic resolution). When the camera holds
//! still it progressively refines — resolution and quality climb toward a full-quality still over
//! ~20 frames. The low-res frame is upscaled to the window with `StretchDIBits`, so it stays smooth.
//!
//! Controls: arrow keys or left-drag to orbit, `W`/`S` to zoom, `Esc` to quit.
//! Run (Windows desktop): cargo run -p mm3e-orchestrator --example viewer --release

// Everything below the stub is Windows-only; gate the imports and the scene builder too, or the
// non-Windows build is left with unused items that fail clippy's -D warnings on Linux CI.
#[cfg(windows)]
use mm3e_kit::color::{Material, Rgba};
#[cfg(windows)]
use mm3e_kit::vec::{Mat3, Transform, Vec3};
#[cfg(windows)]
use mm3e_orchestrator::{Light, Object, Prim, Scene};

#[cfg(windows)]
fn build_scene() -> Scene {
    let mut scene = Scene::new(960, 540);
    scene.bounces = 2;

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
    eprintln!("The interactive viewer requires Windows (raw Win32/GDI). On other platforms, use");
    eprintln!("the offline renderers, e.g. `cargo run -p mm3e-orchestrator --example spheres`.");
}

#[cfg(windows)]
fn main() {
    win32::run(build_scene());
}

#[cfg(windows)]
mod win32 {
    use super::*;
    use mm3e_kit::font;
    use mm3e_orchestrator::reproject::{reproject, GFrame};
    use mm3e_orchestrator::{orbit_camera, render, render_gbuffer, Quality};
    use std::ffi::c_void;
    use std::time::Instant;

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
    const VK_LEFT: i32 = 0x25;
    const VK_UP: i32 = 0x26;
    const VK_RIGHT: i32 = 0x27;
    const VK_DOWN: i32 = 0x28;
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

    pub fn run(mut scene: Scene) {
        let (base_w, base_h) = (960u32, 540u32);
        let bg = Rgba::rgb8(0, 0, 0);
        let target_ms = 30.0f32; // frame budget while moving
        let mut move_div = 3.0f32; // dynamic-resolution divisor (auto-tuned)
        let mut still = 0u32;

        let mut yaw = 0.5f32;
        let mut pitch = 0.3f32;
        let mut radius = 8.5f32;
        let mut last = Point { x: 0, y: 0 };
        let mut dragging = false;
        let mut bgra: Vec<u32> = Vec::new();
        let mut prev_g: Option<GFrame> = None; // last real G-buffer, for reprojected fake frames
        let mut cycle = 0u32;

        unsafe {
            let instance = GetModuleHandleW(std::ptr::null());
            let class_name = wide("mm3e_viewer");
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
            let title = wide("MM3E — adaptive CPU viewer (arrows/drag orbit, W/S zoom, Esc quit)");
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

                // --- input → camera, and motion detection ---
                let mut moved = false;
                if down(VK_LEFT) {
                    yaw -= 0.04;
                    moved = true;
                }
                if down(VK_RIGHT) {
                    yaw += 0.04;
                    moved = true;
                }
                if down(VK_UP) {
                    pitch = (pitch + 0.03).min(1.45);
                    moved = true;
                }
                if down(VK_DOWN) {
                    pitch = (pitch - 0.03).max(-0.2);
                    moved = true;
                }
                if down(0x57) {
                    radius = (radius - 0.15).max(2.5);
                    moved = true;
                }
                if down(0x53) {
                    radius = (radius + 0.15).min(30.0);
                    moved = true;
                }
                let mut cur = Point { x: 0, y: 0 };
                GetCursorPos(&mut cur);
                if down(VK_LBUTTON) {
                    if dragging && (cur.x != last.x || cur.y != last.y) {
                        yaw += (cur.x - last.x) as f32 * 0.01;
                        pitch = (pitch - (cur.y - last.y) as f32 * 0.01).clamp(-0.2, 1.45);
                        moved = true;
                    }
                    dragging = true;
                } else {
                    dragging = false;
                }
                last = cur;

                if moved {
                    still = 0;
                } else {
                    still = (still + 1).min(1000);
                }

                // --- choose internal resolution + quality, then render or reproject ---
                let cam = orbit_camera(Vec3::new(0.0, 0.85, 0.4), radius, yaw, pitch, 52f32.to_radians());
                let t0 = Instant::now();
                let (rw, rh);
                let mode;
                let mut rgba: Vec<u8>; // flat RGBA8 at (rw, rh)

                if still == 0 {
                    // Moving: low-res fast preset, with reprojection between real frames. A real
                    // G-buffer is rendered when the resolution changed or every 3rd frame; the other
                    // frames are reprojected (warped) from the last real frame — much cheaper.
                    rw = ((base_w as f32 / move_div) as u32).max(220);
                    rh = ((base_h as f32 / move_div) as u32).max(124);
                    Quality::fast(rw, rh).apply(&mut scene);
                    let res_changed = prev_g.as_ref().is_none_or(|g| g.width != rw || g.height != rh);
                    if res_changed || cycle.is_multiple_of(3) {
                        let g = render_gbuffer(&scene, &cam, &[]);
                        rgba = g.color.iter().flat_map(|&p| p).collect();
                        prev_g = Some(g);
                        mode = "MOVING";
                    } else if let Some(p) = prev_g.as_ref() {
                        rgba = reproject(p, &cam, &[]).iter().flat_map(|&p| p).collect();
                        mode = "REPROJ";
                    } else {
                        let g = render_gbuffer(&scene, &cam, &[]);
                        rgba = g.color.iter().flat_map(|&p| p).collect();
                        prev_g = Some(g);
                        mode = "MOVING";
                    }
                    cycle += 1;
                } else {
                    // Still: progressive refinement to a full-quality beauty frame (no reprojection).
                    prev_g = None;
                    cycle = 0;
                    let tnorm = (still as f32 / 20.0).min(1.0);
                    let div = 3.0 + (1.0 - 3.0) * tnorm; // 3 → 1 as it converges
                    rw = ((base_w as f32 / div) as u32).max(220);
                    rh = ((base_h as f32 / div) as u32).max(124);
                    Quality::lerp(Quality::fast(rw, rh), Quality::full(rw, rh), tnorm).apply(&mut scene);
                    rgba = render(&scene, &cam).to_rgba8(bg);
                    mode = if tnorm >= 1.0 { "FULL" } else { "REFINING" };
                }
                let ms = t0.elapsed().as_secs_f32() * 1000.0;
                let fps = if ms > 0.0 { 1000.0 / ms } else { 999.0 };

                // Dynamic resolution: hold the frame budget while moving.
                if still == 0 {
                    if ms > target_ms * 1.25 {
                        move_div = (move_div * 1.12).min(9.0);
                    } else if ms < target_ms * 0.8 {
                        move_div = (move_div / 1.1).max(1.0);
                    }
                }

                // --- HUD + present (upscaled to the window) ---
                let hud = format!("{mode} {rw}X{rh} {fps:.0} FPS");
                font::draw_text(&mut rgba, rw, rh, 6, 6, 2, &hud, [255, 232, 96]);
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
                // When fully converged, idle a little so we don't spin re-rendering the same frame.
                if still > 24 {
                    Sleep(30);
                } else {
                    Sleep(1);
                }
            }
        }
    }
}
