// listing 41
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::rtweekend::Shared;
use crate::vec3::{Vec3, Color, color};
use rand::rngs::ThreadRng;

pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng) -> Option<ScatterRecord>;
}

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(r: f64, g: f64, b: f64) -> Shared<dyn Material> {
        return Shared::new( Lambertian {albedo: color(r,g,b)} )
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng)
        -> Option<ScatterRecord> {
        let scatter_dir0 = &hit_record.normal + Vec3::random_unit_vector(rng);
        // listing 46: guard agains very small scatter_direction
        let scatter_direction = if scatter_dir0.near_zero() {
            hit_record.normal.clone()
        } else {
            scatter_dir0
        };

        let scattered = Ray::new(&hit_record.p, &scatter_direction);
        return Some(ScatterRecord{attenuation: self.albedo.clone(),
                                  scattered: scattered})

    }
}

pub struct Metal {
    albedo: Color
}

impl Metal {
    pub fn new(r: f64, g: f64, b: f64) -> Shared<dyn Material> {
        return Shared::new( Metal {albedo: color(r,g,b)} )
    }
}


impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, _rng: &mut ThreadRng)
        -> Option<ScatterRecord> {
            let reflected = ray_in.dir.unit_vector().reflect(&hit_record.normal);
            let scattered = Ray::new(&hit_record.p, &reflected);

            if scattered.dir.dot(&hit_record.normal) > 0. {
                Some(ScatterRecord{attenuation: self.albedo.clone(),
                                   scattered: scattered})
            } else {
                None
            }

    }
}
