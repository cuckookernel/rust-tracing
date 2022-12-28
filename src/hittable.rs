
use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::rtweekend::Shared;

#[derive(Default, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    // listing 18
    #[inline(always)]
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {outward_normal.clone()} else {(-outward_normal).clone()};
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

type SharedHittable = Shared<dyn Hittable>;

pub struct HittableList {
    pub objects: Vec<SharedHittable>
}

pub fn hittable_single(object: SharedHittable) -> HittableList {
    HittableList{ objects: vec![object] }
}

pub fn hittable_list(objects: &Vec<SharedHittable>) -> HittableList {
    HittableList{ objects: objects.clone() }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord{..Default::default()};
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                let temp_rec = temp_rec.clone();
                *rec = temp_rec.clone()
            }
        }

        return hit_anything;
    }
}