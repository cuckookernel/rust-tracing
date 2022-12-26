
mod vec3_img;
mod vec3;
mod ray;

use image::ImageBuffer;
use vec3::Point3;
use crate::vec3::{vec3_, Vec3};
use crate::ray::{Ray, hit_sphere};
use std::time::Instant;


fn main() {
    // listing_1();
    // listing_7()
    listing_9_10()
}


fn listing_1() {
    let image_width = 1024;
    let image_height = 1024;

    let img =
        ImageBuffer::from_fn(image_width, image_height,
        |i, j| {
            let r = (i as f32) / ((image_width - 1) as f32);
            let g: f32 = (j as f32) / ((image_height -1) as f32);
            let b = 0.25 as f32;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;
            image::Rgb( [ir, ig, ib])
        }
    );

    img.save("generated_imgs/listing_1.png").unwrap();
    // jgp works fine but produces larger file size and quality
    // is not as good in this case, presumably because of the way
    // neighboring pixels only differ buy at most 1 in each
    // of the color components
    img.save("generated_imgs/listing_1.jpg").unwrap();
}


fn listing_7() {
    let img_width = 1024;
    let img_height = 1024;

    let img =
        ImageBuffer::from_fn(img_width, img_height,
        |i, j|{
            vec3_((i as f64) / ((img_width - 1) as f64),
                  (j as f64) / ((img_height - 1) as f64),
                0.25).to_rgb()
        }

    );
    img.save("generated_imgs/listing_7.png").unwrap();
}

fn listing_9_10() {
    fn ray_color(ray: &Ray) -> Vec3 {
        let unit_dir = ray.dir.unit_vector();
        let t = 0.5 * (unit_dir.y + 1.0);
        return (1.0 - t) * vec3_(1.0, 1.0, 1.0) + t * vec3_(0.5, 0.7, 1.);
    }

    fn ray_color_withs_sphere(ray: &Ray) -> Vec3 {
        if hit_sphere(&vec3_(0., 0., -1.), 0.5, &ray) {
            vec3_(1.0, 1.0, 0.5)
        } else {
            ray_color(ray)
        }

    }

    let aspect_ratio = 16.0 / 9.0;
    let img_width = 400;
    let img_height = ((img_width as f64) / aspect_ratio) as u32;

    let viewport_height = 2.0;
    let viewport_with = aspect_ratio * viewport_height;

    let focal_length = 1.0;

    let origin = Point3{x: 0., y: 0., z: 0.};
    let horizontal = vec3_(viewport_with, 0., 0.);
    let vertical = vec3_(0., viewport_height, 0.);
    let lower_left_corner = (&origin - (&horizontal / 2.0)) - (&vertical/2.0) - vec3_(0., 0., focal_length);

    let now = Instant::now();
    let img =
        ImageBuffer::from_fn(img_width, img_height,
         |i, j| {
            let u = i as f64 / (img_width - 1) as f64;
            let v = (img_height - j) as f64 / (img_height - 1) as f64;
            let r = Ray{origin: origin.clone(),
                             dir: &lower_left_corner + u * &horizontal + v * &vertical - &origin};
            // listing_9
            // ray_color(&r).to_rgb()
            // listing 10
            ray_color_withs_sphere(&r).to_rgb()
        });
    let elapsed = now.elapsed();

    let mps = (img_width * img_height) as f64 / 1.0e6;
    println!("Elapsed: {:.2?} ({:.2?} per megapixel)", elapsed,  elapsed.as_secs_f64() /mps);

    //img.save("generated_imgs/listing_9.png").unwrap();
    img.save("generated_imgs/listing_10.png").unwrap();

}