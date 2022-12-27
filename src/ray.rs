
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

// listing 10
pub fn hit_sphere_10(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = &r.origin - center;
    let a = r.dir.dot(& r.dir);
    let b = 2.0 * oc.dot(&r.dir);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b*b - 4.0 * a * c;

    discriminant > 0.0
}

// listing 11
pub fn hit_sphere_11(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = &r.origin - center;
    let a = r.dir.dot(& r.dir);
    let b = 2.0 * oc.dot(&r.dir);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b*b - 4.0 * a * c;


    if discriminant > 0. {
        return (-b - discriminant.sqrt()) / 2.0
    } else {
        return -1.
    }
}

// listing 12: hit sphere optimized
pub fn hit_sphere_12(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = &r.origin - center;
    let a = r.dir.length_squared();
    let half_b = oc.dot(&r.dir);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return -1.0
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}
