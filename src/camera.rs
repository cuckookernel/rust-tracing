// listing 27
use crate::vec3::{Vec3, Point3, point3, vec3_};
use crate::ray::Ray;

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        let origin = point3(0., 0., 0.);
        let horizontal = vec3_( viewport_width, 0., 0.);
        let vertical = vec3_( 0., viewport_height, 0.);
        let lower_left_corner = &origin - &horizontal / 2. - &vertical/2.
                                      - vec3_(0., 0., focal_length);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical
        }
    }
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
          origin: self.origin.clone(),
          dir: &self.lower_left_corner + u * &self.horizontal
               + v * &self.vertical - &self.origin
        }
    }
}