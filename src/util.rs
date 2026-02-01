use bevy::prelude::*;

pub fn close_on_esc(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    primary_window: Query<Entity, With<bevy::window::PrimaryWindow>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        if let Some(entity) = primary_window.iter().next() {
            commands.entity(entity).despawn();
        }
    }
}

fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let d = max - min;
    let mut h = 0.0;
    if d != 0.0 {
        if max == r {
            h = (g - b) / d + (if g < b { 6.0 } else { 0.0 });
        } else if max == g {
            h = (b - r) / d + 2.0;
        } else {
            h = (r - g) / d + 4.0;
        }
        h /= 6.0;
    }
    let s = if max == 0.0 { 0.0 } else { d / max };
    let v = max;
    (h, s, v)
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let i = (h * 6.0).floor();
    let f = h * 6.0 - i;
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);
    match i as i32 % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    }
}

pub fn load_and_shift(path: &str, shift: f32) -> Option<Image> {
    #[cfg(target_os = "macos")]
    let base_path = if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let bundle_assets = exe_dir.join("../Resources/assets");
            if bundle_assets.exists() {
                bundle_assets
            } else {
                std::path::PathBuf::from("assets")
            }
        } else {
            std::path::PathBuf::from("assets")
        }
    } else {
        std::path::PathBuf::from("assets")
    };

    #[cfg(not(target_os = "macos"))]
    let base_path = std::path::PathBuf::from("assets");

    let full_path = base_path.join(path);
    if let Ok(img) = image::open(&full_path) {
        let mut rgba = img.into_rgba8();
        for pixel in rgba.pixels_mut() {
            let [r, g, b, a] = pixel.0;
            if a > 0 {
                let (h, s, v) = rgb_to_hsv(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
                let nh = if s > 0.15 { (h + shift).fract() } else { h };
                let (nr, ng, nb) = hsv_to_rgb(nh, s, v);
                pixel.0 = [(nr * 255.0).round() as u8, (ng * 255.0).round() as u8, (nb * 255.0).round() as u8, a];
            }
        }
        
        let width = rgba.width();
        let height = rgba.height();
        let data = rgba.into_raw();
        
        Some(Image::new(
            bevy::render::render_resource::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            bevy::render::render_resource::TextureDimension::D2,
            data,
            bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
            bevy::asset::RenderAssetUsages::default(),
        ))
    } else {
        None
    }
}
