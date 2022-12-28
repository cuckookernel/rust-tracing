
mod vec3_img;
mod vec3;
mod ray;
mod sphere;
mod hittable;
mod rtweekend;

use image::ImageBuffer;
use vec3::Point3;
use crate::rtweekend::{inf};
use crate::hittable::{HitRecord, hittable_list, Hittable};
use crate::vec3::{vec3_, color};
use crate::ray::{Ray};
use crate::sphere::{hit_sphere_10, hit_sphere_11, hit_sphere_12, make_sphere};
use crate::vec3_img::RGB;

use std::time::Instant;


fn main() {
    // listing_1();
    // listing_7()
    listing_9_24(24)
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

fn listing_9_24(listing_num: i32) {
    let world = hittable_list(&vec![
        make_sphere( vec3_(0., 0., -1.), 0.5),
        make_sphere( vec3_(0., -100.5, -1.), 100.0)
    ]);

    // listing 24
    fn ray_color_hittable_world(ray: &Ray, world: &dyn Hittable) -> RGB {
        let mut rec = HitRecord{..Default::default()};

        if world.hit(ray, 0., inf, &mut rec) {
            (0.5 * (&rec.normal + color(1., 1., 1.))).to_rgb()
        } else {
            ray_color_background(ray)
        }
    }

    let aspect_ratio = 16.0 / 9.0;
    let img_width = 2000;
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
            let ray = Ray{origin: origin.clone(),
                               dir: &lower_left_corner + u * &horizontal + v * &vertical - &origin};

            match listing_num {
                9 => ray_color_background(&ray),
                10 => ray_color_with_sphere(&ray),
                11 => ray_color_with_shaded_sphere_11(&ray),
                12 => ray_color_with_shaded_sphere_12(&ray),
                24 => ray_color_hittable_world(&ray, &world),
                _ => panic!("Can't do listing_num {}", listing_num)
            }
        });
    let elapsed = now.elapsed();

    let mps = (img_width * img_height) as f64 / 1.0e6;
    println!("Elapsed: {:.2?} ({:.2?} ms / megapixel)", elapsed,  elapsed.as_secs_f64() * 1000.0 /mps);

    //img.save("generated_imgs/listing_9.png").unwrap();
    img.save(format!("generated_imgs/listing_{}.png", listing_num)).unwrap();

}



fn ray_color_background(ray: &Ray) -> RGB {
    let unit_dir = ray.dir.unit_vector();
    let t = 0.5 * (unit_dir.y + 1.0);
    let rgb_vec = (1.0 - t) * color(1.0, 1.0, 1.0) + t * color(0.5, 0.7, 1.);
    return rgb_vec.to_rgb()
}

fn ray_color_with_sphere(ray: &Ray) -> RGB {
    if hit_sphere_10(&vec3_(0., 0., -1.), 0.5, &ray) {
        color(1.0, 1.0, 0.5).to_rgb()
    } else {
        ray_color_background(ray)
    }
}

fn ray_color_with_shaded_sphere_11(ray: &Ray) -> RGB {
    let t = hit_sphere_11(&vec3_(0., 0., -1.), 0.5, &ray);

    if t > 0. {
        let normal = (ray.at(t) - vec3_(0., 0., -1.)).unit_vector();
        let rgb_vec = 0.5 * color(normal.x, normal.y, normal.z);
        rgb_vec.to_rgb()
    } else {
        ray_color_background(ray)
    }
}

fn ray_color_with_shaded_sphere_12(ray: &Ray) -> RGB {
    let t = hit_sphere_12(&vec3_(0., 0., -1.), 0.5, &ray);

    if t > 0. {
        let normal = (ray.at(t) - vec3_(0., 0., -1.)).unit_vector();
        let rgb_vec = 0.5 * color(normal.x, normal.y, normal.z);
        rgb_vec.to_rgb()
    } else {
        ray_color_background(ray)
    }
}
