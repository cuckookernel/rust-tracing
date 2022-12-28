
use image::Rgb;
use crate::vec3::Vec3;

pub type RGB = Rgb<u8>;

impl Vec3 {
    // this corresponds to listing_6
    pub fn to_rgb(self) -> RGB {
        color(self.x, self.y, self.z)
    }

    // listing 29
    pub fn to_rgb_sampled(self, samples_per_pixel: i32) -> RGB {
        let scale = 1.0 / (samples_per_pixel as f64);

        color(self.x * scale, self.y * scale, self.z * scale)
    }
}


fn color(r: f64, g: f64, b: f64) -> RGB {
    Rgb([(255.999 * r) as u8,
         (255.999 * g) as u8,
         (255.999 * b) as u8])

}
