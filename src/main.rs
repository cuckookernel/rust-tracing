
mod vec3_img;
mod vec3;
mod ray;
mod sphere;
mod hittable;
mod rtweekend;
mod camera;
mod material;

use std::ops::Range;

use camera::CameraWithFocus;
use hittable::HittableList;
use image::ImageBuffer;
use material::Dielectric;
use rand::rngs::ThreadRng;
use crate::rtweekend::{INF, random_unif, random_unif_1, degrees_to_radians, clamp, Shared};
use crate::hittable::{HitRecord, hittable_list, Hittable, hittable_single};
use crate::vec3::{vec3_, color, Color, point3, Vec3, Point3};
use crate::ray::{Ray};
use crate::sphere::{Sphere, hit_sphere_10, hit_sphere_11, hit_sphere_12};
use crate::camera::Camera;
use crate::vec3_img::color_no_gamma;
use crate::material::{Lambertian, Metal, Material};

use std::time::Instant;

fn main() {
    let listing_num = 71;

    let rp = RenderParams::new(16.0/6.0, 400, 100);

    match  listing_num {
        n  if n < 0 => _use_some_funs(),
        1 => listing_1(),
        7 => listing_7(),
        n if n>=9 && n <= 24 => listing_9_24(n),
        n if n >= 30 && n < 69 => listing_30_68(n, rp),
        n if n >= 30 && n < 69 => listing_30_68(n, rp),
        n if n >= 30 && n < 69 => listing_69_(n, rp),
        69  => {
            let rp = RenderParams::new(16.0/6.0, 400, 100);
            listing_69_(listing_num, rp)
        },
        70 => {
            let rp = RenderParams::new(3.0/2.0, 1200, 500);
            listing_69_(listing_num, rp)
        },
        71 => {
            let rp = RenderParams::new(3.0/2.0, 900, 200);
            listing_69_(listing_num, rp)
        }
        _ =>  panic!("listing_num: {} out of range", listing_num)
    };
}


struct RenderParams {
    aspect_ratio: f64,
    img_width: u32,
    img_height: u32,
    samples_per_pixel: i32,
    depth: i32
}

impl RenderParams {
    fn new(aspect_ratio: f64, img_width: u32, samples_per_pixel: i32)-> Self {
        RenderParams {
            aspect_ratio,
            img_width,
            img_height: ((img_width as f64) / aspect_ratio) as u32,
            samples_per_pixel,
            depth: 50
        }
    }
}


fn listing_69_(listing_num: i32, rp: RenderParams) {
    let mut rng = rand::thread_rng();

    let world = match listing_num {
        69 => four_sphere_world_65(),
        70 => many_sphere_world_70(&mut rng),
        71 => marble_v1(&mut rng),
        _ => panic!("Can't make world for {}", listing_num)
    };


    let camera = match listing_num {
        n if (n == 70)|| (n == 71) => {
            CameraWithFocus::new(
                &point3(13., 2., 3.),
                &point3(0.,0.,0.),
                vec3_(0.,1.,0.),
                20.0,
                rp.aspect_ratio, 0.1, 10.0
            )
        },
        _ => {
            let lookfrom = point3(3., 3., 2.);
            let lookat = point3(0., 0., -1.);
            CameraWithFocus::new(
                &lookfrom,
                &lookat,
                vec3_(0., 1., 0.),
                20.0,
                rp.aspect_ratio,
                2.0,
                (&lookfrom - &lookat).length())
        }
    };

    let now = Instant::now();
    let img =
        ImageBuffer::from_fn(rp.img_width, rp.img_height,
         |i, j| {
            let mut pixel_color = color(0., 0., 0.);
            for _ in 0..rp.samples_per_pixel {
                let u = (i as f64 + random_unif_1(&mut rng))
                              / (rp.img_width - 1) as f64;
                let v = ((rp.img_height - j) as f64 + random_unif_1(&mut rng))
                              / (rp.img_height - 1) as f64;
                let ray = camera.get_ray(u, v, &mut rng);

                let ray_color = match listing_num {
                    n if n <= 70 => ray_color_49(&ray, &mut rng, &world, rp.depth),
                    n if n >= 71 => ray_color_71(&ray, &mut rng, &world, rp.depth),
                    _ => panic!("this can't happen")
                };

                pixel_color += &ray_color;
            }
            pixel_color.to_rgb_sampled(rp.samples_per_pixel)
        });
    let elapsed = now.elapsed();

    let mps = (rp.img_width * rp.img_height) as f64 / 1.0e6;
    let outfn = format!("generated_imgs/listing_{}.png", listing_num);
    println!("Elapsed: {:.2?} ({:.0?} ms / megapixel) - writing image to {}",
             elapsed,  elapsed.as_secs_f64() * 1000.0 /mps, outfn);

    //img.save("generated_imgs/listing_9.png").unwrap();
    img.save(outfn).unwrap();

}


fn listing_30_68(listing_num: i32, rp: RenderParams) {
    // sampling many rays per Pixel
    let world = match listing_num {
        n if n <= 48 => two_sphere_world(),
        n if n >= 49 && n < 52 => four_sphere_world_50(),
        n if n >= 52 && n < 55 => four_sphere_world_52(),
        n if n >= 55 && n < 60 => four_sphere_world_55(),
        n if n >= 60 && n < 65 => four_sphere_world_60(),
        n if n >= 65 => four_sphere_world_65(),
        _ => panic!("Can't make world for {}", listing_num)
    };

    let camera = match listing_num {
        n if n < 65 =>  Camera::default(),
        65 => Camera::from_lookfrom_at(point3(-2., 2., 1.),
                point3(0., 0., -1.),
                vec3_(0., 1., 0.),
                90.0, rp.aspect_ratio),
        66 => Camera::from_lookfrom_at(point3(-2., 2., 1.),
                    point3(0., 0., -1.),
                vec3_(0., 1., 0.),
                20.0, rp.aspect_ratio),
        _ => Camera::default()
    };


    let now = Instant::now();
    let mut rng = rand::thread_rng();
    let img =
        ImageBuffer::from_fn(rp.img_width, rp.img_height,
         |i, j| {
            let mut pixel_color = color(0., 0., 0.);
            for _ in 0..rp.samples_per_pixel {
                let u = (i as f64 + random_unif_1(&mut rng))
                              / (rp.img_width - 1) as f64;
                let v = ((rp.img_height - j) as f64 + random_unif_1(&mut rng))
                              / (rp.img_height - 1) as f64;
                let ray = camera.get_ray(u, v);

                match listing_num {
                   30 => pixel_color += &ray_color_24(&ray, &world),
                   33 => pixel_color += &ray_color_33(&ray, &mut rng, &world),
                   36 => pixel_color += &ray_color_36(&ray, &mut rng, &world, rp.depth),
                   38 => pixel_color += &ray_color_38(&ray, &mut rng, &world, rp.depth),
                   n if n >= 49  => pixel_color += &ray_color_49(&ray, &mut rng, &world, rp.depth),
                   _ => panic!("can't trace rays for listing_num: {}", listing_num)
                }

            }
            pixel_color.to_rgb_sampled(rp.samples_per_pixel)
        });
    let elapsed = now.elapsed();

    let mps = (rp.img_width * rp.img_height) as f64 / 1.0e6;
    let outfn = format!("generated_imgs/listing_{}.png", listing_num);
    println!("Elapsed: {:.2?} ({:.0?} ms / megapixel) - writing image to {}",
             elapsed,  elapsed.as_secs_f64() * 1000.0 /mps, outfn);

    //img.save("generated_imgs/listing_9.png").unwrap();
    img.save(outfn).unwrap();

}


fn marble_v1(rng: &mut ThreadRng) -> HittableList {
    let mut world = hittable_list( &vec![] );

    let glass = Dielectric::new(1.5);

    let rc = 2.0;
    world.add(Sphere::new(point3(0., 0.,0.), rc, glass));



    for _i in (Range{start: 0, end: 50}) {
        let center = (rc - 0.5) * Vec3::rand_in_sphere_1(rng);
        let albedo = Color::random(rng, 0.5, 1.0);
        let metal = Metal::new(&albedo, 0.1);

        world.add(Sphere::new(center, 0.15, metal));
    }
    world
}

fn many_sphere_world_70(rng: &mut ThreadRng) -> HittableList {
    let mut world = hittable_list( &vec![] );

    let ground_material = Lambertian::new(0.5, 0.5, 0.5);

    world.add(Sphere::new(point3(0.,-1000.,0.), 1000.0, ground_material));

    for a in (Range{start: -11, end: 11}) {
        for b in (Range{start: -11, end: 11}) {
            let choose_mat = random_unif_1(rng);
            let center =  point3(
                (a as f64) + 0.9 * random_unif_1(rng),
                0.2, (b as f64) + 0.9 * random_unif_1(rng));

            if (&center - point3(4., 0.2, 0.)).length() > 0.9 {

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = &Color::random(rng, 0., 1.0)
                                       * &Color::random(rng, 0., 1.0);
                    let sphere_material = Lambertian::with_color(albedo);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(rng, 0.5, 1.0);
                    let fuzz = random_unif(rng, 0., 0.5);
                    let sphere_material = Metal::new(&albedo, fuzz);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else {
                    // glass
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Sphere::new(point3(0., 1., 0.), 1.0, material1));

    let material2 = Lambertian::new(0.4, 0.2, 0.1);
    world.add(Sphere::new(point3(-4.0, 1.0, 0.), 1.0, material2));

    let material3 = Metal::new(&point3(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(point3(4.0, 1.0, 0.0), 1.0, material3));

    world
}



fn four_sphere_world_65() -> HittableList {

    four_spheres_given_mats_60(
        Lambertian::new(0.8, 0.8, 0.0),
        Lambertian::new(0.1, 0.2, 0.5),
        Dielectric::new(1.5),
        Metal::new_rgb(0.8, 0.6, 0.2)
    )
}


fn four_sphere_world_60() -> HittableList {

    four_spheres_given_mats_60(
        Lambertian::new(0.8, 0.8, 0.0),
        Dielectric::new(1.5),
        Dielectric::new(1.5),
        Metal::new_rgb(0.8, 0.6, 0.2)
    )
}

fn four_sphere_world_55() -> HittableList {

    four_spheres_given_mats(
        Lambertian::new(0.8, 0.8, 0.0),
        Dielectric::new(1.5),
        Dielectric::new(1.5),
        Metal::new_rgb(0.8, 0.6, 0.2)
    )
}

fn four_sphere_world_52() -> HittableList {
    four_spheres_given_mats(Lambertian::new(0.8, 0.8, 0.0),
            Lambertian::new(0.7, 0.3, 0.3),
            Metal::new(&color(0.8, 0.8, 0.8), 0.3),
            Metal::new(&color(0.8, 0.6, 0.2), 1.0))
}


fn four_sphere_world_50() -> HittableList {
    let material_ground = Lambertian::new(0.8, 0.8, 0.0);
    let material_center = Lambertian::new(0.7, 0.3, 0.3);
    let material_left = Metal::new_rgb(0.8, 0.8, 0.8);
    let material_right = Metal::new_rgb(0.8, 0.6, 0.2);

    four_spheres_given_mats(material_ground, material_center,
        material_left, material_right)
}


fn four_spheres_given_mats(mat_ground: Shared<dyn Material>, mat_center: Shared<dyn Material>,
                           mat_left: Shared<dyn Material>, mat_right: Shared<dyn Material>) -> HittableList {
    hittable_list(&vec![
        Sphere::new(point3( 0., -100.5, -1.0), 100.0, mat_ground),
        Sphere::new(point3( 0.,   0.0, -1.0), 0.5, mat_center),
        Sphere::new(point3(-1.,   0.0, -1.0), 0.5, mat_left),
        Sphere::new(point3( 1.,   0.0, -1.0), 0.5, mat_right)
    ])
}

fn four_spheres_given_mats_60(mat_ground: Shared<dyn Material>,
                           mat_center: Shared<dyn Material>,
                           mat_left: Shared<dyn Material>,
                           mat_right: Shared<dyn Material>) -> HittableList {
    hittable_list(&vec![
        Sphere::new(point3( 0., -100.5, -1.0), 100.0, mat_ground),
        Sphere::new(point3( 0.,   0.0, -1.0), 0.5, mat_center),
        Sphere::new(point3(-1.,   0.0, -1.0), 0.5, mat_left.clone()),
        Sphere::new(point3(-1.,   0.0, -1.0), -0.45, mat_left),
        Sphere::new(point3( 1.,   0.0, -1.0), 0.5, mat_right)
    ])
}



fn ray_color_71(ray: &Ray, rng: &mut ThreadRng, world: &dyn Hittable, depth: i32) -> Color {
    // listing 38: true lambertian reflection
    let mut rec = HitRecord::default();
    if depth <= 0 { return color(0., 0., 0.)}

    if world.hit(ray, 0.001, INF, &mut rec) {
        if let Some(s_rec) = rec.material.scatter(&ray, &rec, rng) {
            &s_rec.attenuation * &ray_color_49(&s_rec.scattered, rng, world, depth - 1)
        } else {
            color(0., 0., 0.)
        }

    } else {

        if ray.dir.z > 0.0 {
            color(1.0, 1.0, 1.0)
        } else {
            color(0., 0., 0.)
        }

    }
}


fn ray_color_49(ray: &Ray, rng: &mut ThreadRng, world: &dyn Hittable, depth: i32) -> Color {
    // listing 38: true lambertian reflection
    let mut rec = HitRecord::default();
    if depth <= 0 { return color(0., 0., 0.)}

    if world.hit(ray, 0.001, INF, &mut rec) {
        if let Some(s_rec) = rec.material.scatter(&ray, &rec, rng) {
            &s_rec.attenuation * &ray_color_49(&s_rec.scattered, rng, world, depth - 1)
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
    println!( "{} {} {} {:?} {} {:?}",
             degrees_to_radians(90.0),
             random_unif(&mut rng, 0., 1.0),
             clamp(3.0, 1.0, 2.0),
             color_no_gamma(1.0, 1.0, 1.0),
             four_sphere_world_52().objects.len(),
             Camera::from_vfov_aspect(90.0, 16.0 / 9.0));

    let sph1 = Sphere::new_cr(point3(0., 0., 0.), 1.0);
    let hittable_list = hittable_single(sph1);
    println!("hittable_list len: {}", hittable_list.objects.len());
    vec3::test_refract()


}