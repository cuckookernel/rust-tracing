
use image::Rgb;
use crate::vec3::Vec3;


impl Vec3 {
    // this corresponds to listing_6
    pub fn to_rgb(self) -> Rgb<u8> {
        color(self.x, self.y, self.z)
    }
}


fn color(r: f64, g: f64, b: f64) -> Rgb<u8> {
    Rgb([(255.999 * r) as u8,
         (255.999 * g) as u8,
         (255.999 * b) as u8])

}