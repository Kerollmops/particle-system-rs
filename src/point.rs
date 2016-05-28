use ocl::aliases::ClFloat3;
use std::ops::Deref;

pub struct Point(ClFloat3);

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point(ClFloat3::new(x, y, z))
    }
}

impl Deref for Point {
    type Target = ClFloat3;

    fn deref(&self) -> &ClFloat3 {
        &self.0
    }
}
