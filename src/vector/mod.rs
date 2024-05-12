use std::ops;

pub mod gui;

#[derive(Clone, Debug)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64
}
impl Vector2 { // Constructors
    pub fn new() -> Self {
        return Self {
            x: 0.0,
            y: 0.0
        }
    }
    pub fn from_components(x: f64, y: f64) -> Self {
        return Self {
            x,
            y
        };
    }
    pub fn from_polar_compass(magnitude: f64, direction: f64) -> Self {
        return Vector2::from_components(
            f64::sin(direction.to_radians()) * magnitude,
            f64::cos(direction.to_radians()) * magnitude
        )
    }
}
// Operations
impl ops::Add<Vector2> for Vector2 { // Vec1 + Vec2
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        return Vector2::from_components(
            self.x + rhs.x,
            self.y + rhs.y
        );
    }
}
impl ops::Sub<Vector2> for Vector2 { // Vec1 - Vec2
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        return Vector2::from_components(
            self.x - rhs.x,
            self.y - rhs.y
        )
    }
}
impl ops::Mul<Vector2> for Vector2 { // Dot product
    type Output = f64;

    fn mul(self, rhs: Vector2) -> Self::Output {
        return self.dot_prod(rhs);
    }
}

impl ops::Mul<f64> for Vector2 { // Scalar multiplication
    type Output = Vector2;

    fn mul(self, rhs: f64) -> Self::Output {
        return Vector2::from_components(
            self.x * rhs,
            self.y * rhs
        )
    }
}

impl Vector2 { // Operations
    pub fn dot_prod(&self, other: Vector2) -> f64 {
        // ac + bd
        return self.x * other.x + self.y * other.y;
    }
}

impl Vector2 { // Miscellaneous (mostly angle stuff)
    pub fn compass_direction(&self) -> f64 {
        // No direction
        if self.x == 0.0 && self.y == 0.0 {return 0.0;}
        // Quadrant 1 (top-right)
        else if (self.x > 0.0 && self.y > 0.0) {
            return f64::to_degrees(f64::atan(self.x / self.y));
        }
        // Quadrant 2 (top-left)
        else if (self.x < 0.0 && self.y > 0.0) {
            return 270.0 + f64::to_degrees(f64::atan(self.y / f64::abs(self.x)));
        }
        // Quadrant 3 (bottom-left)
        else if (self.x < 0.0 && self.y < 0.0) {
            return 180.0 + f64::to_degrees(f64::atan(f64::abs(self.x) / f64::abs(self.y)));
        }
        // Quadrant 4 (bottom-right)
        else if (self.x > 0.0 && self.y < 0.0) {
            return 90.0 + f64::to_degrees(f64::atan(f64::abs(self.y) / self.x));
        }
        // Vertical
        else if (self.x == 0.0) {
            if (self.y > 0.0) {return 0.0;}        // Up
            else if (self.y < 0.0) {return 180.0;} // Down
        }
        // Horizontal
        else if (self.y == 0.0) {
            if (self.x < 0.0) {return 270.0;}     // Left
            else if (self.x > 0.0) {return 90.0;} // Right
        }
        return 0.0;
    }
    fn polar_angle_from_compass(angle: f64) -> f64 {
        let angle = angle % 360.0;
        if angle >= 0.0 && angle < 90.0 { // Top right
            return -90.0 + angle;
        }
        else if angle >= 90.0 && angle <= 270.0 { // Bottom
            return angle - 90.0;
        }
        else if angle > 270.0 && angle < 360.0 { // Top left
            return -180.0 + (angle - 270.0)
        }
        return 0.0;
    }
    fn compass_angle_from_polar(angle: f64) -> f64 {
        if angle < 0.0 { // Top
            return if angle < -90.0 { // Top left
                360.0 - angle.abs()
            } else { // Top right
                90.0 - angle.abs()
            }
        }
        else if angle <= 0.0 {
            return angle + 90.0
        }
        return 0.0
    }
    pub fn magnitude(&self) -> f64 {
        // Basic pythagoras
        return f64::sqrt(self.x * self.x + self.y * self.y);
    }
}
