
use crate::vec3::{Vec3, Point3};

#[derive(Default)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + &self.dir * t
    }
}
