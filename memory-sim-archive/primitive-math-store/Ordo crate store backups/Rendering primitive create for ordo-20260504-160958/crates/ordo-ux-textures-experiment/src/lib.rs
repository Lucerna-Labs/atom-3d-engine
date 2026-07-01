//! Renderer-neutral texture and material recipe experiments for Ordo UX.
//!
//! This crate describes film grain, glass, sheen, scanlines, paper, metal, and
//! other surface treatments as data. It can emit lightweight approximation
//! primitives for debugging, but it does not own shaders, image generation,
//! GPU passes, render targets, or backend-specific effect code.

use kurbo::Point;
use ordo_ux_primitives::{Bounds, Fill, Primitive, Shape, Stroke, ThemeTokens};
use peniko::Color;

/// High-level texture category.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextureKind {
    /// Fine random luminance variation.
    FilmGrain,
    /// Frosted translucent glass surface.
    FrostedGlass,
    /// Glossy glass surface.
    GlossGlass,
    /// Subtle paper fibers.
    PaperFiber,
    /// Brushed metal streaks.
    BrushedMetal,
    /// CRT or monitor scanlines.
    Scanlines,
    /// Soft edge darkening.
    Vignette,
    /// Directional highlight sheen.
    Sheen,
    /// Noise-backed glow.
    GlowMist,
}

/// Blend intent for a texture layer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextureBlend {
    /// Normal source-over compositing.
    #[default]
    Normal,
    /// Multiply/darken intent.
    Multiply,
    /// Screen/lighten intent.
    Screen,
    /// Overlay contrast intent.
    Overlay,
    /// Additive glow intent.
    Add,
    /// Mask/alpha-only intent.
    Mask,
}

/// Deterministic noise settings.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NoiseSpec {
    /// Stable seed for deterministic texture generation.
    pub seed: u64,
    /// Noise scale in logical UI units.
    pub scale: f64,
    /// Number of octave layers.
    pub octaves: u8,
    /// Roughness between octaves.
    pub roughness: f32,
}

impl NoiseSpec {
    /// Creates a fine-grain noise spec.
    #[must_use]
    pub fn fine(seed: u64) -> Self {
        Self {
            seed,
            scale: 2.0,
            octaves: 3,
            roughness: 0.56,
        }
    }

    /// Creates a broad soft-noise spec.
    #[must_use]
    pub fn soft(seed: u64) -> Self {
        Self {
            seed,
            scale: 18.0,
            octaves: 4,
            roughness: 0.48,
        }
    }
}

/// Glass material settings.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlassSpec {
    /// Tint color.
    pub tint: Color,
    /// Tint alpha contribution.
    pub tint_alpha: f32,
    /// Intended blur radius for a renderer backend.
    pub blur_radius: f64,
    /// Intended saturation multiplier for backdrop content.
    pub backdrop_saturation: f32,
    /// Border highlight alpha.
    pub highlight_alpha: f32,
}

impl GlassSpec {
    /// Creates a frosted glass spec.
    #[must_use]
    pub fn frosted(tint: Color) -> Self {
        Self {
            tint,
            tint_alpha: 0.34,
            blur_radius: 18.0,
            backdrop_saturation: 1.18,
            highlight_alpha: 0.42,
        }
    }

    /// Creates a glossy glass spec.
    #[must_use]
    pub fn glossy(tint: Color) -> Self {
        Self {
            tint,
            tint_alpha: 0.22,
            blur_radius: 8.0,
            backdrop_saturation: 1.08,
            highlight_alpha: 0.62,
        }
    }
}

/// Directional texture settings.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DirectionalSpec {
    /// Angle in radians.
    pub angle_radians: f64,
    /// Distance between strokes or bands.
    pub spacing: f64,
    /// Stroke width.
    pub width: f64,
}

impl DirectionalSpec {
    /// Creates horizontal directional settings.
    #[must_use]
    pub fn horizontal(spacing: f64, width: f64) -> Self {
        Self {
            angle_radians: 0.0,
            spacing,
            width,
        }
    }
}

/// One material layer.
#[derive(Clone, Debug, PartialEq)]
pub struct TextureLayer {
    /// Layer id.
    pub id: String,
    /// Texture category.
    pub kind: TextureKind,
    /// Layer color.
    pub color: Color,
    /// Overall opacity.
    pub opacity: f32,
    /// Blend intent.
    pub blend: TextureBlend,
    /// Optional noise settings.
    pub noise: Option<NoiseSpec>,
    /// Optional glass settings.
    pub glass: Option<GlassSpec>,
    /// Optional directional settings.
    pub directional: Option<DirectionalSpec>,
}

impl TextureLayer {
    /// Creates a texture layer.
    #[must_use]
    pub fn new(id: impl Into<String>, kind: TextureKind, color: Color) -> Self {
        Self {
            id: id.into(),
            kind,
            color,
            opacity: 1.0,
            blend: TextureBlend::Normal,
            noise: None,
            glass: None,
            directional: None,
        }
    }

    /// Creates a film-grain layer.
    #[must_use]
    pub fn film_grain(seed: u64, opacity: f32) -> Self {
        Self::new(
            "film-grain",
            TextureKind::FilmGrain,
            Color::new([1.0, 1.0, 1.0, 1.0]),
        )
        .with_opacity(opacity)
        .with_blend(TextureBlend::Overlay)
        .with_noise(NoiseSpec::fine(seed))
    }

    /// Creates a frosted-glass layer.
    #[must_use]
    pub fn frosted_glass(tint: Color) -> Self {
        Self::new("frosted-glass", TextureKind::FrostedGlass, tint)
            .with_opacity(1.0)
            .with_blend(TextureBlend::Screen)
            .with_glass(GlassSpec::frosted(tint))
    }

    /// Creates a scanline layer.
    #[must_use]
    pub fn scanlines(color: Color, opacity: f32) -> Self {
        Self::new("scanlines", TextureKind::Scanlines, color)
            .with_opacity(opacity)
            .with_blend(TextureBlend::Multiply)
            .with_directional(DirectionalSpec::horizontal(6.0, 1.0))
    }

    /// Sets opacity.
    #[must_use]
    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }

    /// Sets blend intent.
    #[must_use]
    pub fn with_blend(mut self, blend: TextureBlend) -> Self {
        self.blend = blend;
        self
    }

    /// Sets noise settings.
    #[must_use]
    pub fn with_noise(mut self, noise: NoiseSpec) -> Self {
        self.noise = Some(noise);
        self
    }

    /// Sets glass settings.
    #[must_use]
    pub fn with_glass(mut self, glass: GlassSpec) -> Self {
        self.glass = Some(glass);
        self
    }

    /// Sets directional settings.
    #[must_use]
    pub fn with_directional(mut self, directional: DirectionalSpec) -> Self {
        self.directional = Some(directional);
        self
    }

    /// Returns a backend-neutral effect hint for this layer.
    #[must_use]
    pub fn effect_hint(&self) -> TextureEffectHint {
        match self.kind {
            TextureKind::FrostedGlass | TextureKind::GlossGlass => TextureEffectHint::BackdropBlur,
            TextureKind::FilmGrain | TextureKind::PaperFiber | TextureKind::GlowMist => {
                TextureEffectHint::ProceduralNoise
            }
            TextureKind::BrushedMetal | TextureKind::Scanlines | TextureKind::Sheen => {
                TextureEffectHint::DirectionalBands
            }
            TextureKind::Vignette => TextureEffectHint::GradientMask,
        }
    }
}

/// Backend-neutral implementation hint.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextureEffectHint {
    /// Can be approximated with procedural noise.
    ProceduralNoise,
    /// Wants backdrop blur/saturation.
    BackdropBlur,
    /// Can be approximated with repeated bands.
    DirectionalBands,
    /// Can be approximated with a gradient or mask.
    GradientMask,
}

/// A complete material recipe.
#[derive(Clone, Debug, PartialEq)]
pub struct MaterialRecipe {
    /// Material name.
    pub name: String,
    /// Base surface color.
    pub base: Color,
    /// Corner radius used by approximation primitives.
    pub radius: f64,
    /// Ordered material layers.
    pub layers: Vec<TextureLayer>,
}

impl MaterialRecipe {
    /// Creates a recipe.
    #[must_use]
    pub fn new(name: impl Into<String>, base: Color) -> Self {
        Self {
            name: name.into(),
            base,
            radius: 8.0,
            layers: Vec::new(),
        }
    }

    /// Glass card material.
    #[must_use]
    pub fn glass_card(theme: &ThemeTokens) -> Self {
        Self::new("glass-card", theme.surface.with_alpha(0.64))
            .with_radius(theme.radius_lg)
            .with_layer(TextureLayer::frosted_glass(theme.surface))
            .with_layer(TextureLayer::film_grain(17, 0.05))
            .with_layer(
                TextureLayer::new(
                    "edge-sheen",
                    TextureKind::Sheen,
                    Color::new([1.0, 1.0, 1.0, 1.0]),
                )
                .with_opacity(0.18)
                .with_blend(TextureBlend::Screen)
                .with_directional(DirectionalSpec {
                    angle_radians: -0.65,
                    spacing: 28.0,
                    width: 2.0,
                }),
            )
    }

    /// Cinematic film surface material.
    #[must_use]
    pub fn film_surface(theme: &ThemeTokens) -> Self {
        Self::new("film-surface", theme.surface)
            .with_radius(theme.radius_md)
            .with_layer(TextureLayer::film_grain(101, 0.12))
            .with_layer(
                TextureLayer::new(
                    "vignette",
                    TextureKind::Vignette,
                    Color::new([0.0, 0.0, 0.0, 1.0]),
                )
                .with_opacity(0.18)
                .with_blend(TextureBlend::Multiply),
            )
    }

    /// Monitor/terminal scanline material.
    #[must_use]
    pub fn monitor_glass(theme: &ThemeTokens) -> Self {
        Self::new("monitor-glass", theme.background)
            .with_radius(theme.radius_md)
            .with_layer(TextureLayer::scanlines(
                Color::new([0.0, 0.0, 0.0, 1.0]),
                0.12,
            ))
            .with_layer(TextureLayer::film_grain(33, 0.04))
            .with_layer(
                TextureLayer::new("glow-mist", TextureKind::GlowMist, theme.accent)
                    .with_opacity(0.08)
                    .with_blend(TextureBlend::Add)
                    .with_noise(NoiseSpec::soft(44)),
            )
    }

    /// Sets corner radius.
    #[must_use]
    pub fn with_radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }

    /// Adds a material layer.
    #[must_use]
    pub fn with_layer(mut self, layer: TextureLayer) -> Self {
        self.layers.push(layer);
        self
    }

    /// Returns all backend-neutral effect hints used by this material.
    #[must_use]
    pub fn effect_hints(&self) -> Vec<TextureEffectHint> {
        let mut hints = Vec::new();
        for layer in &self.layers {
            let hint = layer.effect_hint();
            if !hints.contains(&hint) {
                hints.push(hint);
            }
        }
        hints
    }

    /// Emits lightweight approximation primitives for debugging or fallback renderers.
    #[must_use]
    pub fn approximation_primitives(&self, bounds: Bounds) -> Vec<Primitive> {
        let mut primitives = vec![Primitive::fill(
            Shape::rounded_rect(
                bounds.rect.x0,
                bounds.rect.y0,
                bounds.rect.width(),
                bounds.rect.height(),
                self.radius,
            ),
            Fill::new(self.base),
        )];

        for layer in &self.layers {
            match layer.kind {
                TextureKind::FrostedGlass | TextureKind::GlossGlass => {
                    if let Some(glass) = layer.glass {
                        primitives.push(Primitive::fill(
                            Shape::rounded_rect(
                                bounds.rect.x0,
                                bounds.rect.y0,
                                bounds.rect.width(),
                                bounds.rect.height(),
                                self.radius,
                            ),
                            Fill::new(glass.tint.with_alpha(glass.tint_alpha * layer.opacity)),
                        ));
                        primitives.push(Primitive::stroke(
                            Shape::rounded_rect(
                                bounds.rect.x0,
                                bounds.rect.y0,
                                bounds.rect.width(),
                                bounds.rect.height(),
                                self.radius,
                            ),
                            Stroke::new(Color::new([1.0, 1.0, 1.0, glass.highlight_alpha]), 1.0),
                        ));
                    }
                }
                TextureKind::FilmGrain | TextureKind::PaperFiber | TextureKind::GlowMist => {
                    primitives.extend(noise_dots(bounds, layer, self.radius));
                }
                TextureKind::BrushedMetal | TextureKind::Scanlines | TextureKind::Sheen => {
                    primitives.extend(direction_lines(bounds, layer));
                }
                TextureKind::Vignette => {
                    primitives.push(Primitive::stroke(
                        Shape::rounded_rect(
                            bounds.rect.x0 + 1.0,
                            bounds.rect.y0 + 1.0,
                            bounds.rect.width() - 2.0,
                            bounds.rect.height() - 2.0,
                            self.radius,
                        ),
                        Stroke::new(layer.color.with_alpha(layer.opacity), 6.0),
                    ));
                }
            }
        }

        primitives
    }
}

/// A named material token.
#[derive(Clone, Debug, PartialEq)]
pub struct MaterialToken {
    /// Stable token name.
    pub name: String,
    /// Material recipe.
    pub recipe: MaterialRecipe,
}

impl MaterialToken {
    /// Creates a material token.
    #[must_use]
    pub fn new(name: impl Into<String>, recipe: MaterialRecipe) -> Self {
        Self {
            name: name.into(),
            recipe,
        }
    }
}

/// Collection of material recipes.
#[derive(Clone, Debug, PartialEq)]
pub struct MaterialSystem {
    /// System name.
    pub name: String,
    /// Material tokens.
    pub tokens: Vec<MaterialToken>,
}

impl MaterialSystem {
    /// Creates a modern default material system.
    #[must_use]
    pub fn modern(theme: &ThemeTokens) -> Self {
        Self {
            name: "modern-materials".to_string(),
            tokens: vec![
                MaterialToken::new("glass-card", MaterialRecipe::glass_card(theme)),
                MaterialToken::new("film-surface", MaterialRecipe::film_surface(theme)),
                MaterialToken::new("monitor-glass", MaterialRecipe::monitor_glass(theme)),
            ],
        }
    }

    /// Looks up a material by name.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&MaterialRecipe> {
        self.tokens
            .iter()
            .find(|token| token.name == name)
            .map(|token| &token.recipe)
    }

    /// Inserts or replaces a material token.
    pub fn set(&mut self, token: MaterialToken) {
        if let Some(existing) = self
            .tokens
            .iter_mut()
            .find(|existing| existing.name == token.name)
        {
            *existing = token;
        } else {
            self.tokens.push(token);
        }
    }
}

fn noise_dots(bounds: Bounds, layer: &TextureLayer, radius: f64) -> Vec<Primitive> {
    let noise = layer.noise.unwrap_or_else(|| NoiseSpec::fine(0));
    let count = ((bounds.rect.width() * bounds.rect.height()) / (noise.scale * noise.scale * 18.0))
        .clamp(6.0, 96.0) as usize;
    let mut seed = noise.seed;
    let mut primitives = Vec::with_capacity(count);

    for _ in 0..count {
        let x = bounds.rect.x0 + next_unit(&mut seed) * bounds.rect.width();
        let y = bounds.rect.y0 + next_unit(&mut seed) * bounds.rect.height();
        let alpha = (0.12 + next_unit(&mut seed) as f32 * 0.28) * layer.opacity;
        let dot_radius = (0.35 + next_unit(&mut seed) * 0.75).min(radius.max(0.75));
        primitives.push(Primitive::fill(
            Shape::circle(Point::new(x, y), dot_radius),
            Fill::new(layer.color.with_alpha(alpha)),
        ));
    }

    primitives
}

fn direction_lines(bounds: Bounds, layer: &TextureLayer) -> Vec<Primitive> {
    let directional = layer
        .directional
        .unwrap_or_else(|| DirectionalSpec::horizontal(8.0, 1.0));
    let count = (bounds.rect.height() / directional.spacing).ceil().max(0.0) as usize;
    let mut primitives = Vec::with_capacity(count);

    for index in 0..count {
        let y = bounds.rect.y0 + index as f64 * directional.spacing;
        primitives.push(Primitive::stroke(
            Shape::line(
                Point::new(bounds.rect.x0, y),
                Point::new(bounds.rect.x1, y + directional.angle_radians.sin() * 12.0),
            ),
            Stroke::new(layer.color.with_alpha(layer.opacity), directional.width),
        ));
    }

    primitives
}

fn next_unit(seed: &mut u64) -> f64 {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
    ((*seed >> 33) as f64) / (u32::MAX as f64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use kurbo::Size;

    fn bounds() -> Bounds {
        Bounds::new(Point::ORIGIN, Size::new(180.0, 100.0))
    }

    #[test]
    fn film_grain_layer_has_noise_and_overlay_blend() {
        let layer = TextureLayer::film_grain(7, 0.1);

        assert_eq!(layer.kind, TextureKind::FilmGrain);
        assert_eq!(layer.blend, TextureBlend::Overlay);
        assert!(layer.noise.is_some());
    }

    #[test]
    fn glass_material_reports_backdrop_blur_hint() {
        let recipe = MaterialRecipe::glass_card(&ThemeTokens::default());

        assert!(
            recipe
                .effect_hints()
                .contains(&TextureEffectHint::BackdropBlur)
        );
        assert!(
            recipe
                .effect_hints()
                .contains(&TextureEffectHint::ProceduralNoise)
        );
    }

    #[test]
    fn material_emits_approximation_primitives() {
        let recipe = MaterialRecipe::film_surface(&ThemeTokens::default());
        let primitives = recipe.approximation_primitives(bounds());

        assert!(primitives.len() > 3);
        assert!(matches!(primitives.first(), Some(Primitive::Fill { .. })));
    }

    #[test]
    fn monitor_glass_contains_scanline_and_glow_layers() {
        let recipe = MaterialRecipe::monitor_glass(&ThemeTokens::default());

        assert!(
            recipe
                .layers
                .iter()
                .any(|layer| layer.kind == TextureKind::Scanlines)
        );
        assert!(
            recipe
                .layers
                .iter()
                .any(|layer| layer.kind == TextureKind::GlowMist)
        );
    }

    #[test]
    fn material_system_lookup_and_replace_work() {
        let mut system = MaterialSystem::modern(&ThemeTokens::default());
        let custom = MaterialRecipe::new("custom", Color::BLACK)
            .with_layer(TextureLayer::scanlines(Color::BLACK, 0.2));
        system.set(MaterialToken::new("custom", custom));

        assert!(system.get("glass-card").is_some());
        assert!(system.get("custom").is_some());
    }

    #[test]
    fn noise_generation_is_deterministic() {
        let layer = TextureLayer::film_grain(9, 0.1);
        let first = noise_dots(bounds(), &layer, 8.0);
        let second = noise_dots(bounds(), &layer, 8.0);

        assert_eq!(first, second);
    }
}
