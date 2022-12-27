use crate::vec3::Point3;
use crate::ray::Ray;

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64
}


pub trait Hittable {
    fn hit(ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool
}