use std::ops;

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
    pub fn from(x: f64, y: f64) -> Self {
        return Self {
            x,
            y
        };
    }
}
impl Vector2 {
    fn add() {
        println!("ADDING VECTORS!")
    }
}

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        println!("Adding vectors!");
        return Vector2::from(
            self.x + rhs.x,
            self.y + rhs.y
        );
    }
}