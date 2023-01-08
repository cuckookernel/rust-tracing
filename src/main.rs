
mod vec3_img;
mod vec3;
mod ray;
mod sphere;
mod hittable;
mod rtweekend;
mod camera;
mod material;

use hittable::HittableList;
use image::ImageBuffer;
use rand::rngs::ThreadRng;
use crate::rtweekend::{INF, random_unif, random_unif_1, degrees_to_radians, clamp};
use crate::hittable::{HitRecord, hittable_list, Hittable, hittable_single};
use crate::vec3::{vec3_, color, Color, point3, Vec3, Point3};
use crate::ray::{Ray};
use crate::sphere::{Sphere, hit_sphere_10, hit_sphere_11, hit_sphere_12};
use crate::camera::Camera;
use crate::vec3_img::color_no_gamma;
use crate::material::{Lambertian, Metal};

use std::time::Instant;


fn main() {
    let listing_num = 55;

    match  listing_num {
        1 => listing_1(),
        7 => listing_7(),
        n if n>=9 && n <= 24 => listing_9_24(n),
        n if n >= 30 => listing_30_(n),
        n  if n < 0 => _use_some_funs(),
        _ =>  panic!("listing_num: {} out of range", listing_num)
    }
}



fn listing_30_(listing_num: i32) {
    // sampling many rays per Pixel
    let world = match listing_num {
        n if n <= 48 => two_sphere_world(),
        n if n >= 49 && n < 55 => three_sphere_world_50(),
        n if n >= 55 => three_sphere_world_50(),
        _ => panic!("Can't make world for {}", listing_num)
    };

    let aspect_ratio = 16.0 / 9.0;
    let img_width = 400;
    let img_height = ((img_width as f64) / aspect_ratio) as u32;
    let samples_per_pixel = 100;

    let camera = Camera::default();
    let depth = 50;

    let now = Instant::now();
    let mut rng = rand::thread_rng();
    let img =
        ImageBuffer::from_fn(img_width, img_height,
         |i, j| {
            let mut pixel_color = color(0., 0., 0.);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_unif_1(&mut rng))
                              / (img_width - 1) as f64;
                let v = ((img_height - j) as f64 + random_unif_1(&mut rng))
                              / (img_height - 1) as f64;
                let ray = camera.get_ray(u, v);

                match listing_num {
                   30 => pixel_color += &ray_color_24(&ray, &world),
                   33 => pixel_color += &ray_color_33(&ray, &mut rng, &world),
                   36 => pixel_color += &ray_color_36(&ray, &mut rng, &world, depth),
                   38 => pixel_color += &ray_color_38(&ray, &mut rng, &world, depth),
                   n if n >= 49  => pixel_color += &ray_color_49(&ray, &mut rng, &world, depth),
                   _ => panic!("can't trace rays for listing_num: {}", listing_num)
                }

            }
            pixel_color.to_rgb_sampled(samples_per_pixel)
        });
    let elapsed = now.elapsed();

    let mps = (img_width * img_height) as f64 / 1.0e6;
    let outfn = format!("generated_imgs/listing_{}.png", listing_num);
    println!("Elapsed: {:.2?} ({:.0?} ms / megapixel) - writing image to {}",
             elapsed,  elapsed.as_secs_f64() * 1000.0 /mps, outfn);

    //img.save("generated_imgs/listing_9.png").unwrap();
    img.save(outfn).unwrap();

}


fn three_sphere_world_52() -> HittableList {
    let material_ground = Lambertian::new(0.8, 0.8, 0.0);
    let material_center = Lambertian::new(0.7, 0.3, 0.3);
    let material_left = Metal::new(&color(0.8, 0.8, 0.8), 0.3);
    let material_right = Metal::new(&color(0.8, 0.6, 0.2), 1.0);

    let world = hittable_list(&vec![
        Sphere::new(point3( 0., -100.5, -1.0), 100.0, material_ground),
        Sphere::new(point3( 0.,   0.0, -1.0), 0.5, material_center),
        Sphere::new(point3(-1.,   0.0, -1.0), 0.5, material_left),
        Sphere::new(point3( 1.,   0.0, -1.0), 0.5, material_right)
    ]);
    world
}


fn three_sphere_world_50() -> HittableList {
    let material_ground = Lambertian::new(0.8, 0.8, 0.0);
    let material_center = Lambertian::new(0.7, 0.3, 0.3);
    let material_left = Metal::new_rgb(0.8, 0.8, 0.8);
    let material_right = Metal::new_rgb(0.8, 0.6, 0.2);

    let world = hittable_list(&vec![
        Sphere::new(point3( 0., -100.5, -1.0), 100.0, material_ground),
        Sphere::new(point3( 0.,   0.0, -1.0), 0.5, material_center),
        Sphere::new(point3(-1.,   0.0, -1.0), 0.5, material_left),
        Sphere::new(point3( 1.,   0.0, -1.0), 0.5, material_right)
    ]);
    world
}


fn ray_color_49(ray: &Ray, rng: &mut ThreadRng, world: &dyn Hittable, depth: i32) -> Color {
    // listing 38: true lambertian reflection
    let mut rec = HitRecord::default();
    if depth <= 0 { return color(0., 0., 0.)}

    if world.hit(ray, 0.001, INF, &mut rec) {
        if let Some(s_rec) = rec.material.scatter(&ray, &rec, rng) {
            &s_rec.attenuation * &ray_color_49(&s_rec.scattered, rng,  world, depth - 1)
        } else {
            color(0., 0., 0.)
        }

    } else {
        ray_color_background(ray)
    }
}


fn ray_color_38(ray: &Ray, rng: &mut ThreadRng,
    world: &dyn Hittable, depth: i32) -> Color {
    // listing 38: true lambertian reflection
    let mut rec = HitRecord::default();
    if depth <= 0 { return color(0., 0., 0.)}

    if world.hit(ray, 0.001, INF, &mut rec) {
        let target = &rec.p + &rec.normal + &Vec3::random_unit_vector(rng);
        let reflected_ray = Ray::new(&rec.p, &(target - &rec.p));
        0.5 * &ray_color_38(&reflected_ray, rng, world, depth -1)
    } else {
        ray_color_background(ray)
    }
}


fn ray_color_36(ray: &Ray, rng: &mut ThreadRng,
    world: &dyn Hittable, depth: i32) -> Color {
    // listing 36: fixing shadow acne
    let mut rec = HitRecord::default();
    if depth <= 0 { return color(0., 0., 0.) }

    if world.hit(ray, 0.001, INF, &mut rec) {
    let target = &rec.p + &rec.normal + &Vec3::rand_in_sphere_1(rng);
        let reflected_ray = Ray::new(&rec.p, &(target - &rec.p));
        0.5 * &ray_color_36(&reflected_ray, rng, world, depth - 1)
    } else {
        ray_color_background(ray)
    }
}


fn ray_color_33(ray: &Ray, rng: &mut ThreadRng,
    world: &dyn Hittable) -> Color {
    // listing 33: with reflection from diffuse materials
    let mut rec = HitRecord{..Default::default()};

    if world.hit(ray, 0., INF, &mut rec) {
    let target = &rec.p + &rec.normal + &Vec3::rand_in_sphere_1(rng);
        let reflected_ray = Ray::new(&rec.p, &(target - &rec.p));
        0.5 * &ray_color_33(&reflected_ray, rng, world)
    } else {
        ray_color_background(ray)
    }
}


fn listing_9_24(listing_num: i32) {
    let world = two_sphere_world();
    // listing 24

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

            let color = match listing_num {
                9 => ray_color_background(&ray),
                10 => ray_color_with_sphere(&ray),
                11 => ray_color_with_shaded_sphere_11(&ray),
                12 => ray_color_with_shaded_sphere_12(&ray),
                24 => ray_color_24(&ray, &world),
                _ => panic!("Can't do listing_num {}", listing_num)
            };
            color.to_rgb()
        });
    let elapsed = now.elapsed();

    let mps = (img_width * img_height) as f64 / 1.0e6;
    println!("Elapsed: {:.2?} ({:.2?} ms / megapixel)", elapsed,  elapsed.as_secs_f64() * 1000.0 /mps);

    //img.save("generated_imgs/listing_9.png").unwrap();
    img.save(format!("generated_imgs/listing_{}.png", listing_num)).unwrap();

}

fn two_sphere_world() -> HittableList {
    hittable_list(&vec![
        Sphere::new_cr( vec3_(0., 0., -1.), 0.5),
        Sphere::new_cr( vec3_(0., -100.5, -1.), 100.0)
    ])
}

fn ray_color_24(ray: &Ray, world: &dyn Hittable) -> Color {
    // listing 24
    let mut rec = HitRecord{..Default::default()};

    if world.hit(ray, 0., INF, &mut rec) {
        0.5 * (&rec.normal + color(1., 1., 1.))
    } else {
        ray_color_background(ray)
    }
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


#[inline(always)]
fn ray_color_background(ray: &Ray) -> Color {
    let unit_dir = ray.dir.unit_vector();
    let t = 0.5 * (unit_dir.y + 1.0);
    let rgb_vec = (1.0 - t) * color(1.0, 1.0, 1.0) + t * color(0.5, 0.7, 1.);
    return rgb_vec
}


fn ray_color_with_sphere(ray: &Ray) -> Color {
    if hit_sphere_10(&vec3_(0., 0., -1.), 0.5, &ray) {
        color(1.0, 1.0, 0.5)
    } else {
        ray_color_background(ray)
    }
}

fn ray_color_with_shaded_sphere_11(ray: &Ray) -> Color {
    // listing 11
    let t = hit_sphere_11(&vec3_(0., 0., -1.), 0.5, &ray);

    if t > 0. {
        let normal = (ray.at(t) - vec3_(0., 0., -1.)).unit_vector();
        let rgb_vec = 0.5 * color(normal.x, normal.y, normal.z);
        rgb_vec
    } else {
        ray_color_background(ray)
    }
}

fn ray_color_with_shaded_sphere_12(ray: &Ray) -> Color {
    // listing 12
    let t = hit_sphere_12(&vec3_(0., 0., -1.), 0.5, &ray);

    if t > 0. {
        let normal = (ray.at(t) - vec3_(0., 0., -1.)).unit_vector();
        let rgb_vec = 0.5 * color(normal.x, normal.y, normal.z);
        rgb_vec
    } else {
        ray_color_background(ray)
    }
}

fn _use_some_funs() {
    // function to use some other functions and avoid warnings `xyz` is never used
    let mut rng = rand::thread_rng();
    println!( "{} {} {} {:?} {}",
             degrees_to_radians(90.0),
             random_unif(&mut rng, 0., 1.0),
             clamp(3.0, 1.0, 2.0),
             color_no_gamma(1.0, 1.0, 1.0),
             three_sphere_world_52().objects.len());

    let sph1 = Sphere::new_cr(point3(0., 0., 0.), 1.0);
    let hittable_list = hittable_single(sph1);
    println!("{}", hittable_list.objects.len())


}