// We use Scalar here, if tomorrow if we want to convert to f64 
// or any other type

pub type Scalar = f32;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Point {
    pub x: Scalar, //meters, horizontal (left/right)
    pub y: Scalar, //meters, depth (forward/back)
    pub z: Scalar, //meters, height (up/down)
}

impl Point {
    //Constructor
    pub const fn new(x: Scalar, y: Scalar, z: Scalar) -> Self {
        Self { x, y, z }
    }

    //Handy Functions
    pub const fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
    pub const fn unit_x() -> Self {
        Self { x: 1.0, y: 0.0, z: 0.0 }
    }

    pub const fn unit_y() -> Self {
        Self { x: 0.0, y: 1.0, z: 0.0 }
    }

    pub const fn unit_z() -> Self {
        Self {x: 0.0, y: 0.0, z: 1.0 }
    }

    //Replace One Component
    pub const fn with_x(self, x: Scalar) -> Self {
        Self { x: x, y: self.y, z: self.z }
    }
    pub const fn with_y(self, y: Scalar) -> Self {
        Self { x: self.x, y: y, z: self.z }
    }
    
    pub const fn with_z(self, z: Scalar) -> Self {
        Self { x: self.x, y: self.y, z: z }
    }
}
