use super::constants::{ UnitVec2, Unit };

/// Represents an Axis-Aligned-Bounding-Box used for
/// collision detection.
#[derive(Debug)]
pub struct Aabb {
    pub top_left: UnitVec2,
    pub dimensions: UnitVec2,
}

impl Aabb {
    /// Gets the width of this [AABB].
    #[allow(unused)]
    pub fn width(&self) -> Unit {
        self.dimensions.x
    }

    /// Gets the height of this [AABB].
    pub fn height(&self) -> Unit {
        self.dimensions.y
    }

    /// Gets the top left coordinate of this [AABB].
    pub fn top_left(&self) -> UnitVec2 {
        self.top_left
    }

    /// Gets the top right coordinate of this [AABB].
    #[allow(unused)]
    pub fn top_right(&self) -> UnitVec2 {
        self.top_left + UnitVec2 { x: self.dimensions.x, y: 0 }
    }

    /// Gets the bottom left coordinate of this [AABB].
    #[allow(unused)]
    pub fn bottom_left(&self) -> UnitVec2 {
        self.top_left + UnitVec2 { x: 0, y: self.dimensions.y }
    }

    /// Gets the bottom right coordinate of this [AABB].
    pub fn bottom_right(&self) -> UnitVec2 {
        self.top_left + self.dimensions
    }

    /// Gets the left x coordinate of this [AABB].
    pub fn x_left(&self) -> Unit {
        self.top_left.x
    }

    /// Gets the right x coordinate of this [AABB].
    pub fn x_right(&self) -> Unit {
        self.top_left.x + self.dimensions.x
    }

    /// Gets the top y coordinate of this [AABB].
    pub fn y_top(&self) -> Unit {
        self.top_left.y
    }

    /// Gets the bottom y coordinate of this [AABB].
    pub fn y_bottom(&self) -> Unit {
        self.top_left.y + self.dimensions.y
    }

    /// Checks if this [AABB] collides with another [AABB].
    #[allow(unused)]
    pub fn collides_with(&self, other: &Aabb) -> bool {
        self.x_left() < other.x_right() &&
            self.x_right() > other.x_left() &&
            self.y_top() < other.y_bottom() &&
            self.y_bottom() > other.y_top()
    }
}
