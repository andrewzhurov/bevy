use bevy_math::Vec2;
use bevy_reflect::{std_traits::ReflectDefault, Reflect};

/// Defines the extents of the border of a rectangle.
///
/// This struct is used to represent thickness or offsets from the edges
/// of a rectangle (left, right, top, and bottom), with values increasing inwards.
#[derive(Default, Copy, Clone, PartialEq, Debug, Reflect)]
#[reflect(Clone, PartialEq, Default)]
pub struct BorderRect {
    /// Extent of the border along the left edge
    pub left: f32,
    /// Extent of the border along the right edge
    pub right: f32,
    /// Extent of the border along the top edge
    pub top: f32,
    /// Extent of the border along the bottom edge
    pub bottom: f32,
}

impl BorderRect {
    /// An empty border with zero thickness along each edge
    pub const ZERO: Self = Self::all(0.);

    /// Creates a border with the same `extent` along each edge
    #[must_use]
    #[inline]
    pub const fn all(extent: f32) -> Self {
        Self {
            left: extent,
            right: extent,
            top: extent,
            bottom: extent,
        }
    }

    /// Creates a new border with the `left` and `right` extents equal to `horizontal`, and `top` and `bottom` extents equal to `vertical`.
    #[must_use]
    #[inline]
    pub const fn axes(horizontal: f32, vertical: f32) -> Self {
        Self {
            left: horizontal,
            right: horizontal,
            top: vertical,
            bottom: vertical,
        }
    }

    pub fn sum_axes(&self) -> Vec2 {
        Vec2::new(self.left + self.right, self.top + self.bottom)
    }

    /// Expands the given border rect on all sides by a given expansion amount.
    pub fn inflate_mut(&mut self, expansion: f32) {
        self.left += expansion;
        self.right += expansion;
        self.top += expansion;
        self.bottom += expansion;
    }
}

impl core::ops::AddAssign for BorderRect {
    fn add_assign(&mut self, rhs: Self) {
        self.left += rhs.left;
        self.right += rhs.right;
        self.top += rhs.top;
        self.bottom += rhs.bottom;
    }
}

impl From<f32> for BorderRect {
    fn from(extent: f32) -> Self {
        Self::all(extent)
    }
}

impl From<[f32; 4]> for BorderRect {
    fn from([left, right, top, bottom]: [f32; 4]) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }
}
