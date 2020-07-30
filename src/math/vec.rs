use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

pub const Origin : Vec2 = Vec2 {x: 0.0, y: 0.0};

#[wasm_bindgen]
impl Vec2 {

    /// Return a new Vec2.
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
        }
    }
    /// Set components of a Vec2.
    pub fn set_components(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    /// Set components of Vec2 from another.
    pub fn set(&mut self, v : &Vec2) {
        self.x = v.x;
        self.y = v.y;
    }

    /// Return whether there is a NaN value in the x or y values.
    pub fn has_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }

    /// Return the length.
    pub fn length(&self) -> f64 {
        self.x.hypot(self.y)
    }

    /// Create and return the unit vector.
    pub fn unit(&self) -> Vec2 {
        let mut vec = self.clone();
        vec.divide(self.length());
        vec
    }

    /// Return distance from one Vec2 to another.
    pub fn distance_to(&self, v: &Vec2) -> f64 {
        (self.x - v.x).hypot(self.y - v.y)
    }

    /// Returns the squared distance from one Vec2 to another.
    // might be wrong about this though
    pub fn distance_squared_to(&self, v: &Vec2) -> f64 {
        (self.x - v.x).powi(2) +  (self.y - v.y).powi(2)
    }

    /// Return the dot product.
    pub fn dot(&self, v: &Vec2) -> f64 {
        self.x * v.x + self.y * v.y
    }

}

impl Vec2 {
    
    /// Subtract a vector and return a reference to the result.
    pub fn subtract(&mut self, v: &Vec2) -> &mut Self {
        self.x -= v.x;
        self.y -= v.y;
        self
    }
    
    /// Add a vector and return a reference to the result.
    pub fn add(&mut self, v: &Vec2) -> &mut Self {
        self.x += v.x;
        self.y += v.y;
        self
    }
    
    /// Multiply two vectors and return a reference to the result.
    pub fn multiply(&mut self, s: f64) -> &mut Self {
        self.x *= s;
        self.y *= s;
        self
    }
    
    /// Scale by s and return the result.
    pub fn scale(&mut self, s: f64) -> &mut Self {
        self.multiply(s)
    }
    
    /// Divide by s and return a reference to the result.
    pub fn divide(&mut self, s: f64) -> &mut Self {
        self.x /= s;
        self.y /= s;
        self
    }
    
    
    /// Return the x and y values as an array.
    pub fn as_array(&self) -> [f64; 2] {
        [self.x, self.y]
    }
    
    /// Rotate the vector by a certain number of radians.
    /// The value `origin` represents whether it is to be rotated by the origin or not.
    /// The value `angle` is in radians.
    pub fn rotate(&mut self, angle: f64, origin : bool) -> &mut Vec2 {
        let (c, s) = (angle.cos(), angle.sin());

        if origin {
            let (x, y) = (self.x, self.y);

            self.x = x * c - y * s;
            self.y = y * c - x * s;
        } else {
            self.subtract(&Origin);
            self.rotate(angle, origin);
            self.add(&Origin);
        }

        self
    }
    
    /// Rotate the vector by a certain number of degrees.
    /// Essentially the same as the functoin `rotate()` except this takes in degrees instead of radians.
    pub fn rotateDeg(&mut self, angle_deg: f64, origin: bool) -> &mut Self {
        self.rotate(angle_deg / 180.0 * 3.14159265359, origin);
        self
    }
}

