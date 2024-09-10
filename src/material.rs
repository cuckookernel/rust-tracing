// listing 41
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::rtweekend::{Shared, random_unif_1};
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

    pub fn with_color(color: Vec3) -> Shared<dyn Material> {
        return Shared::new( Lambertian {albedo: color} )
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
    albedo: Color,
    fuzz: f64
}

impl Metal {
    pub fn new(color: &Color, fuzz: f64) -> Shared<dyn Material> {
        Shared::new( Metal {
            albedo: color.clone(),
            fuzz: if fuzz < 1.0  { fuzz } else {1.0}
        })
    }
    pub fn new_rgb(r: f64, g: f64, b: f64) -> Shared<dyn Material> {
        return Shared::new( Metal {albedo: color(r,g,b), fuzz: 0.} )
    }
}


impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng)
        -> Option<ScatterRecord> {
            let reflected = ray_in.dir.unit_vector().reflect(&hit_record.normal);
            let dir = reflected + &(self.fuzz * Vec3::rand_in_sphere_1(rng)); // listing 51
            let scattered = Ray::new(&hit_record.p, &dir);

            if scattered.dir.dot(&hit_record.normal) > 0. {
                Some(ScatterRecord{attenuation: self.albedo.clone(),
                                   scattered: scattered})
            } else {
                None
            }

    }
}


pub struct Dielectric {
    eta: f64  // index_of_refraction
}

impl Dielectric {
    pub fn new(eta: f64) -> Shared<dyn Material> {
        Shared::new( Dielectric {eta} )
    }

    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0_ = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0_*r0_;
        return r0 + (1.0 - r0) * pow5(1.0 - cosine)
    }
}

fn pow5(x: f64) -> f64 {
    let s = x * x;
    return s * s * x
}


impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rng: &mut ThreadRng)
     -> Option<ScatterRecord> {
        let attenuation = color(1.0, 1.0, 1.0);

        let eta_ratio = if rec.front_face { 1.0 / self.eta } else { self.eta };

        let unit_dir = r_in.dir.unit_vector();
        let cos_theta = (-unit_dir.dot(&rec.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = eta_ratio * sin_theta > 1.0;
        let shall_not_refract = cannot_refract
                        || Dielectric::reflectance(cos_theta, eta_ratio) > random_unif_1(rng);

        let direction = if shall_not_refract {
           unit_dir.reflect(&rec.normal)
        } else {
           unit_dir.refract(&rec.normal, eta_ratio)
        };

        Some(ScatterRecord{
                attenuation,
                scattered: Ray::new(&rec.p, &direction)
             })
    }
}