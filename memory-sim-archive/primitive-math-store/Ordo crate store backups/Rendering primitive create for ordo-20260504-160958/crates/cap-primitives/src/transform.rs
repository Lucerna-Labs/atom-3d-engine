//! Transform — 2D transformation matrices.

use cap_geometry::{Pixels, Point, Size};

/// A 2D affine transformation matrix.
///
/// Stored as a 2x2 rotation/scale matrix plus a translation vector.
/// This is sufficient for all 2D UI transforms (rotate, scale, translate, skew).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    /// 2x2 rotation and scale matrix (row-major).
    pub rotation_scale: [[f32; 2]; 2],
    /// Translation vector.
    pub translation: [f32; 2],
}

impl Eq for Transform {}

impl Transform {
    /// Identity transform (no effect).
    pub const IDENTITY: Transform = Transform {
        rotation_scale: [[1.0, 0.0], [0.0, 1.0]],
        translation: [0.0, 0.0],
    };

    /// Create a translation transform.
    pub fn translate(x: impl Into<Pixels>, y: impl Into<Pixels>) -> Self {
        Transform {
            rotation_scale: [[1.0, 0.0], [0.0, 1.0]],
            translation: [x.into().0, y.into().0],
        }
    }

    /// Create a rotation transform (radians, clockwise).
    pub fn rotate(radians: f32) -> Self {
        Transform {
            rotation_scale: [
                [radians.cos(), -radians.sin()],
                [radians.sin(), radians.cos()],
            ],
            translation: [0.0, 0.0],
        }
    }

    /// Create a uniform scale transform.
    pub fn scale(factor: f32) -> Self {
        Transform {
            rotation_scale: [[factor, 0.0], [0.0, factor]],
            translation: [0.0, 0.0],
        }
    }

    /// Create a non-uniform scale transform.
    pub fn scale_xy(sx: f32, sy: f32) -> Self {
        Transform {
            rotation_scale: [[sx, 0.0], [0.0, sy]],
            translation: [0.0, 0.0],
        }
    }

    /// Compose this transform with another (apply other first, then self).
    pub fn compose(self, other: Transform) -> Transform {
        if other == Self::IDENTITY {
            return self;
        }
        Transform {
            rotation_scale: [
                [
                    self.rotation_scale[0][0] * other.rotation_scale[0][0]
                        + self.rotation_scale[0][1] * other.rotation_scale[1][0],
                    self.rotation_scale[0][0] * other.rotation_scale[0][1]
                        + self.rotation_scale[0][1] * other.rotation_scale[1][1],
                ],
                [
                    self.rotation_scale[1][0] * other.rotation_scale[0][0]
                        + self.rotation_scale[1][1] * other.rotation_scale[1][0],
                    self.rotation_scale[1][0] * other.rotation_scale[0][1]
                        + self.rotation_scale[1][1] * other.rotation_scale[1][1],
                ],
            ],
            translation: [
                self.translation[0]
                    + self.rotation_scale[0][0] * other.translation[0]
                    + self.rotation_scale[0][1] * other.translation[1],
                self.translation[1]
                    + self.rotation_scale[1][0] * other.translation[0]
                    + self.rotation_scale[1][1] * other.translation[1],
            ],
        }
    }

    /// Apply this transform to a point.
    pub fn apply_to_point(&self, point: Point<Pixels>) -> Point<Pixels> {
        let input = [point.x.0, point.y.0];
        let mut output = self.translation;
        for (i, output_cell) in output.iter_mut().enumerate() {
            for (k, input_cell) in input.iter().enumerate() {
                *output_cell += self.rotation_scale[i][k] * *input_cell;
            }
        }
        Point::new(Pixels(output[0]), Pixels(output[1]))
    }

    /// Apply this transform to a size.
    pub fn apply_to_size(&self, size: Size<Pixels>) -> Size<Pixels> {
        Size::new(
            Pixels(
                self.rotation_scale[0][0].abs() * size.width.0
                    + self.rotation_scale[0][1].abs() * size.height.0,
            ),
            Pixels(
                self.rotation_scale[1][0].abs() * size.width.0
                    + self.rotation_scale[1][1].abs() * size.height.0,
            ),
        )
    }

    /// Check if this is the identity transform.
    pub fn is_identity(&self) -> bool {
        *self == Self::IDENTITY
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let t = Transform::IDENTITY;
        let p = Point::new(Pixels(5.0), Pixels(10.0));
        assert_eq!(t.apply_to_point(p), p);
    }

    #[test]
    fn test_translate() {
        let t = Transform::translate(Pixels(3.0), Pixels(7.0));
        let p = Point::new(Pixels(5.0), Pixels(10.0));
        let result = t.apply_to_point(p);
        assert_eq!(result, Point::new(Pixels(8.0), Pixels(17.0)));
    }

    #[test]
    fn test_scale() {
        let t = Transform::scale(2.0);
        let p = Point::new(Pixels(3.0), Pixels(4.0));
        let result = t.apply_to_point(p);
        assert_eq!(result, Point::new(Pixels(6.0), Pixels(8.0)));
    }

    #[test]
    fn test_compose() {
        let translate = Transform::translate(Pixels(10.0), Pixels(0.0));
        let scale = Transform::scale(2.0);
        let combined = translate.compose(scale);
        let p = Point::new(Pixels(5.0), Pixels(0.0));
        let result = combined.apply_to_point(p);
        // Scale first: 5*2=10, then translate: 10+10=20
        assert_eq!(result, Point::new(Pixels(20.0), Pixels(0.0)));
    }

    #[test]
    fn test_is_identity() {
        assert!(Transform::IDENTITY.is_identity());
        assert!(!Transform::translate(Pixels(1.0), Pixels(0.0)).is_identity());
    }
}
