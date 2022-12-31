
use crate::hittable::{Hittable, HitRecord};
use crate::vec3::{Point3};
use crate::ray::Ray;
use crate::rtweekend::Shared;
use crate::material::{Material, Lambertian};

// listing 15
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Shared<dyn Material>
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Shared<dyn Material>) -> Shared<Sphere> {
        Shared::new(Sphere{center, radius, material})
    }

    pub fn new_cr(center: Point3, radius: f64) -> Shared<Sphere> {
        let material = Lambertian::new(1., 1., 1.);
        Shared::new(Sphere{center, radius, material})
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = &ray.origin - &self.center;
        let a = ray.dir.length_squared();
        let half_b = oc.dot(&ray.dir);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            false
        } else {
            let sqrtd = discriminant.sqrt();
            let root = (-half_b - sqrtd)/a;

            if root < t_min || t_max < root {
                let root = (-half_b + sqrtd) / a;
                if root < t_min || t_max < root {
                    return false
                }
            }
            rec.t = root;
            rec.p = ray.at(rec.t);
            let outward_normal= (&rec.p - &self.center) / self.radius;
            rec.set_face_normal(ray, &outward_normal);
            rec.material = self.material.clone();
            true
        }

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
