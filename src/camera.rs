use rand::rngs::ThreadRng;

use crate::rtweekend::degrees_to_radians;
// listing 27
use crate::vec3::{Vec3, Point3, point3, vec3_};
use crate::ray::Ray;

#[derive(Debug)]
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,

}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        let origin = point3(0., 0., 0.);

        Camera::from_origin_hvf(origin, viewport_width, viewport_height, focal_length)
    }
}

pub fn viewport_width_height(vfov_deg: f64, aspect_ratio: f64) -> (f64, f64) {
    let theta = degrees_to_radians(vfov_deg);
    let h = (theta / 2.0).tan();
    let viewport_height = 2.0 * h;
    let viewport_width = aspect_ratio * viewport_height;

    (viewport_width, viewport_height)
}

impl Camera {

    // listing 64
    pub fn from_lookfrom_at(lookfrom: Point3, lookat: Point3, vup: Vec3,
                        vfov_deg: f64, aspect_ratio: f64) -> Self {

        let w  = (&lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);
        let (viewport_width, viewport_height) = viewport_width_height(vfov_deg, aspect_ratio);
        let horizontal = viewport_width * &u;
        let vertical = viewport_height * &v;

        Camera{
            lower_left_corner: &lookfrom - (&horizontal / 2.0) - &(&vertical / 2.0) - &w,
            origin: lookfrom,
            horizontal, vertical
        }
    }

    // listing 63
    pub fn from_vfov_aspect(vfov_deg: f64, aspect_ratio: f64) -> Self {

        let (viewport_width, viewport_height) = viewport_width_height(vfov_deg, aspect_ratio);
        let focal_length = 1.0;

        Self::from_origin_hvf(point3(0.0, 0.0, 0.0),
            viewport_width, viewport_height, focal_length)
    }

    fn from_origin_hvf(origin: Point3, viewport_width: f64,
                       viewport_height: f64, focal_length: f64) -> Self {
        let horizontal =  vec3_(viewport_width, 0.0, 0.0);
        let vertical =  vec3_(0.0, viewport_height, 0.0);
        let lower_left_corner = &origin - &horizontal / 2. - &vertical/2.
        - vec3_(0., 0., focal_length);

        Camera{
            origin, horizontal, vertical,
            lower_left_corner
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
          origin: self.origin.clone(),
          dir: &self.lower_left_corner + u * &self.horizontal
               + v * &self.vertical - &self.origin
        }
    }

}


pub struct CameraWithFocus {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,  // listing 68
}


//listing 68
impl CameraWithFocus {
    pub fn new(lookfrom: &Point3, lookat: &Point3, vup: Vec3,
               vfov_deg: f64, aspect_ratio: f64, aperture: f64,
               focus_dist: f64) -> Self {

        let (viewport_width, viewport_height) = viewport_width_height(vfov_deg, aspect_ratio);
        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let horizontal = focus_dist * viewport_width * &u;
        let vertical = focus_dist * viewport_height * &v;

        CameraWithFocus {
            lower_left_corner: lookfrom - (&horizontal / 2.0) - &(&vertical / 2.0) - focus_dist * &w,
            horizontal, vertical, u, v, w, lens_radius: aperture/2.0,
            origin: lookfrom.clone()
        }

    }
    pub fn get_ray(&self, s: f64, t: f64, rng: &mut ThreadRng) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_disk_1(rng);
        let offset = rd.x * &self.u + rd.y * &self.v;
        return Ray{
            origin: (&self.origin + &offset),
            dir: &self.lower_left_corner + s * &self.horizontal + t * &self.vertical - &self.origin - &offset
        }
    }
}