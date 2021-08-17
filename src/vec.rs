#[derive(Debug)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    // Normalized vector based on some angle
    pub fn new_from_angle(radians: f32) -> Self {
        let v = Self::new(radians.cos(), radians.sin());
        v.to_normalized()
    }

    // Creates a vector pointing from v1 to v2
    pub fn new_from_points(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self {
            x: x2 - x1,
            y: y2 - y1,
        }
    }

    pub fn to_normalized(&self) -> Self {
        let magnitude = self.magnitude();
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
        }
    }

    // pub fn angle(&self) -> f32 {
    //     self.y.atan2(self.x)
    // }

    pub fn magnitude(&self) -> f32 {
        self.x.hypot(self.y)
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn angle_other(&self, other: &Self) -> f32 {
        (self.dot(other) / (self.magnitude() * other.magnitude())).acos()
    }
}
