use glam::Affine2;
use crate::{Pos2, pos2, Rect, Vec2, vec2};

/// Linearly transforms positions via a translation, then a scaling.
///
/// [`TSTransform`] first scales points with the scaling origin at `0, 0`
/// (the top left corner), then translates them.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
//#[cfg_attr(feature = "bytemuck", derive(bytemuck::Pod, bytemuck::Zeroable))]
pub struct TSTransform(pub glam::Affine2);

impl Eq for TSTransform {}

impl Default for TSTransform {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl TSTransform {
    pub const IDENTITY: Self = Self(glam::Affine2::IDENTITY);

    #[inline]
    /// Creates a new translation that first scales points around
    /// `(0, 0)`, then translates them.
    pub fn new(translation: Vec2, scaling: f32, rotation: f32) -> Self {
        Self(glam::Affine2::from_scale_angle_translation(
            glam::Vec2::new(scaling, scaling),
            rotation,
            glam::Vec2::new(translation.x, translation.y),
        ))
    }

    #[inline]
    pub fn from_translation(translation: Vec2) -> Self {
        Self(glam::Affine2::from_translation(glam::Vec2::new(translation.x, translation.y)))
    }

    #[inline]
    pub fn from_scaling(scaling: f32) -> Self {
        Self::new(Vec2::ZERO, scaling, 0.0)
    }

    #[inline]
    pub fn from_rotation(rotation: f32) -> Self {
        Self::new(Vec2::ZERO, 1.0, rotation)
    }

    /// Inverts the transform.
    ///
    /// ```
    /// # use emath::{pos2, vec2, TSTransform};
    /// let p1 = pos2(2.0, 3.0);
    /// let p2 = pos2(12.0, 5.0);
    /// let ts = TSTransform::new(vec2(2.0, 3.0), 2.0);
    /// let inv = ts.inverse();
    /// assert_eq!(inv.mul_pos(p1), pos2(0.0, 0.0));
    /// assert_eq!(inv.mul_pos(p2), pos2(5.0, 1.0));
    ///
    /// assert_eq!(ts.inverse().inverse(), ts);
    /// ```
    #[inline]
    pub fn inverse(&self) -> Self {
        Self(self.0.inverse())
    }

    /// Transforms the given coordinate.
    ///
    /// ```
    /// # use emath::{pos2, vec2, TSTransform};
    /// let p1 = pos2(0.0, 0.0);
    /// let p2 = pos2(5.0, 1.0);
    /// let ts = TSTransform::new(vec2(2.0, 3.0), 2.0);
    /// assert_eq!(ts.mul_pos(p1), pos2(2.0, 3.0));
    /// assert_eq!(ts.mul_pos(p2), pos2(12.0, 5.0));
    /// ```
    #[inline]
    pub fn mul_pos(&self, pos: Pos2) -> Pos2 {
        let p = self.0.transform_point2(glam::Vec2::new(pos.x, pos.y));
        pos2(p.x, p.y)
    }

    /// Transforms the given rectangle.
    ///
    /// ```
    /// # use emath::{pos2, vec2, Rect, TSTransform};
    /// let rect = Rect::from_min_max(pos2(5.0, 5.0), pos2(15.0, 10.0));
    /// let ts = TSTransform::new(vec2(1.0, 0.0), 3.0);
    /// let transformed = ts.mul_rect(rect);
    /// assert_eq!(transformed.min, pos2(16.0, 15.0));
    /// assert_eq!(transformed.max, pos2(46.0, 30.0));
    /// ```
    #[inline]
    pub fn mul_rect(&self, rect: Rect) -> (Rect, f32) {
        let (scale, angle, translation) = self.0.to_scale_angle_translation();

        let (scale, _, translation) = (Affine2::from_angle(-angle) * self.0).to_scale_angle_translation();
        (
            Rect {
                min: scale.x * rect.min + vec2(translation.x, translation.y),
                max: scale.x * rect.max + vec2(translation.x, translation.y),
            },
            angle,
        )
    }

    pub fn scaling(&self) -> f32 {
        self.scale_angle_translation().0
    }

    pub fn scale_angle_translation(&self) -> (f32, f32, Vec2) {
        let (
            scale,
            rotation,
            translation,
        ) = self.0.to_scale_angle_translation();
        (scale.x, rotation, vec2(translation.x, translation.y))
    }
}

/// Transforms the position.
impl std::ops::Mul<Pos2> for TSTransform {
    type Output = Pos2;

    #[inline]
    fn mul(self, pos: Pos2) -> Pos2 {
        self.mul_pos(pos)
    }
}

// /// Transforms the rectangle.
// impl std::ops::Mul<Rect> for TSTransform {
//     type Output = Rect;
//
//     #[inline]
//     fn mul(self, rect: Rect) -> Rect {
//         self.mul_rect(rect)
//     }
// }

impl std::ops::Mul<Self> for TSTransform {
    type Output = Self;

    #[inline]
    /// Applies the right hand side transform, then the left hand side.
    ///
    /// ```
    /// # use emath::{TSTransform, vec2};
    /// let ts1 = TSTransform::new(vec2(1.0, 0.0), 2.0);
    /// let ts2 = TSTransform::new(vec2(-1.0, -1.0), 3.0);
    /// let ts_combined = TSTransform::new(vec2(2.0, -1.0), 6.0);
    /// assert_eq!(ts_combined, ts2 * ts1);
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        // Apply rhs first.
        Self(self.0 * rhs.0)
    }
}
