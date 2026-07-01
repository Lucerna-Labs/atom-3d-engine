//! Fill — how a shape's interior is painted.

use cap_geometry::Pixels;

/// A fill style applied to a shape's interior.
#[derive(Clone, Debug, PartialEq)]
pub enum Fill {
    /// Solid color fill (rgba).
    Solid(Rgba),

    /// Linear gradient fill.
    LinearGradient(LinearGradient),

    /// Radial gradient fill.
    RadialGradient(RadialGradient),

    /// No fill (transparent).
    None,
}

impl Default for Fill {
    fn default() -> Self {
        Fill::None
    }
}

impl Fill {
    /// Create a solid fill from rgba components (0.0–1.0).
    pub fn solid(r: f32, g: f32, b: f32, a: f32) -> Self {
        Fill::Solid(Rgba { r, g, b, a })
    }

    /// Create a solid fill from a hex u32 (0xRRGGBBAA).
    pub fn solid_hex(hex: u32) -> Self {
        Fill::Solid(Rgba::from_hex(hex))
    }

    /// Check if this fill is transparent/none.
    pub fn is_none(&self) -> bool {
        matches!(self, Fill::None)
    }
}

/// RGBA color with float components (0.0–1.0).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    /// Transparent black.
    pub const TRANSPARENT: Rgba = Rgba {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
    /// Opaque black.
    pub const BLACK: Rgba = Rgba {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    /// Opaque white.
    pub const WHITE: Rgba = Rgba {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    /// Parse from a hex u32 (0xRRGGBBAA).
    pub fn from_hex(hex: u32) -> Self {
        let [r, g, b, a] = hex.to_be_bytes().map(|b| (b as f32) / 255.0);
        Rgba { r, g, b, a }
    }

    /// Convert to a hex u32 (0xRRGGBBAA).
    pub fn to_hex(self) -> u32 {
        let r = (self.r * 255.0) as u32;
        let g = (self.g * 255.0) as u32;
        let b = (self.b * 255.0) as u32;
        let a = (self.a * 255.0) as u32;
        (r << 24) | (g << 16) | (b << 8) | a
    }

    /// Blend this color over another (alpha compositing).
    pub fn blend_over(&self, other: Rgba) -> Rgba {
        if self.a >= 1.0 {
            *self
        } else if self.a <= 0.0 {
            other
        } else {
            Rgba {
                r: (other.r * (1.0 - self.a)) + (self.r * self.a),
                g: (other.g * (1.0 - self.a)) + (self.g * self.a),
                b: (other.b * (1.0 - self.a)) + (self.b * self.a),
                a: other.a,
            }
        }
    }

    /// Check if fully transparent.
    pub fn is_transparent(&self) -> bool {
        self.a == 0.0
    }
}

/// A linear gradient fill between two or more color stops.
#[derive(Clone, Debug, PartialEq)]
pub struct LinearGradient {
    /// Start point of the gradient.
    pub start: cap_geometry::Point<Pixels>,
    /// End point of the gradient.
    pub end: cap_geometry::Point<Pixels>,
    /// Color stops (offset 0.0–1.0, color).
    pub stops: Vec<(f32, Rgba)>,
}

/// A radial gradient fill.
#[derive(Clone, Debug, PartialEq)]
pub struct RadialGradient {
    /// Center of the gradient.
    pub center: cap_geometry::Point<Pixels>,
    /// Radius of the gradient.
    pub radius: Pixels,
    /// Color stops (offset 0.0–1.0, color).
    pub stops: Vec<(f32, Rgba)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solid_fill() {
        let fill = Fill::solid(1.0, 0.0, 0.0, 1.0);
        assert_eq!(
            fill,
            Fill::Solid(Rgba {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0
            })
        );
    }

    #[test]
    fn test_hex_fill() {
        let fill = Fill::solid_hex(0xFF0000FF);
        assert!(matches!(fill, Fill::Solid(c) if (c.r - 1.0).abs() < 0.01 && c.a > 0.99));
    }

    #[test]
    fn test_rgba_blend() {
        let red = Rgba {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 0.5,
        };
        let blue = Rgba {
            r: 0.0,
            g: 0.0,
            b: 1.0,
            a: 1.0,
        };
        let blended = red.blend_over(blue);
        assert!((blended.r - 0.5).abs() < 0.01);
        assert!((blended.b - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_rgba_roundtrip() {
        let color = Rgba::from_hex(0x12345678);
        let hex = color.to_hex();
        // Allow minor float precision loss
        assert!((hex as i64 - 0x12345678i64).abs() <= 3);
    }
}
