use super::constants::{ UnitVec2, Unit };

#[derive(Debug)]
pub struct Aabb {
    pub top_left: UnitVec2,
    pub dimensions: UnitVec2,
}

impl Aabb {
    #[allow(unused)]
    pub fn width(&self) -> Unit {
        self.dimensions.x
    }

    pub fn height(&self) -> Unit {
        self.dimensions.y
    }

    pub fn top_left(&self) -> UnitVec2 {
        self.top_left
    }

    #[allow(unused)]
    pub fn top_right(&self) -> UnitVec2 {
        self.top_left + UnitVec2 { x: self.dimensions.x, y: 0 }
    }

    #[allow(unused)]
    pub fn bottom_left(&self) -> UnitVec2 {
        self.top_left + UnitVec2 { x: 0, y: self.dimensions.y }
    }

    pub fn bottom_right(&self) -> UnitVec2 {
        self.top_left + self.dimensions
    }

    pub fn x_left(&self) -> Unit {
        self.top_left.x
    }

    pub fn x_right(&self) -> Unit {
        self.top_left.x + self.dimensions.x
    }

    pub fn y_top(&self) -> Unit {
        self.top_left.y
    }

    pub fn y_bottom(&self) -> Unit {
        self.top_left.y + self.dimensions.y
    }

    #[allow(unused)]
    pub fn collides_with(&self, other: &Aabb) -> bool {
        self.x_left() < other.x_right() &&
            self.x_right() > other.x_left() &&
            self.y_top() < other.y_bottom() &&
            self.y_bottom() > other.y_top()
    }
}
