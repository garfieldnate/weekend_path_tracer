use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::prelude::*;
use std::sync::Arc;
use weekend_path_tracer::{
    camera::Camera,
    canvas::Canvas,
    consts::{sky_blue, white},
    dielectric::Dielectric,
    diffuse::Lambertian,
    hittable_list::HittableList,
    material::Material,
    metal::Metal,
    ray::Ray,
    sphere::Sphere,
    utils::{random_in_01, random_in_range},
    vec3::Vec3,
};

const IMAGE_WIDTH: usize = 200;
const IMAGE_HEIGHT: usize = 100;
const ASPECT_RATIO: f64 = IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64;
const SAMPLES_PER_PIXEL: usize = 100;
const MAX_DEPTH: u8 = 50;
const EPSILON: f64 = 0.001;

fn vec_to_u32(color: Vec3) -> u32 {
    let (r, g, b) = color.to_rgb();
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

fn lerp(a: Vec3, b: Vec3, t: f64) -> Vec3 {
    (1. - t) * a + t * b
}

fn ray_color(r: Ray, world: &HittableList, depth: u8) -> Vec3 {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        Vec3::default()
    } else {
        // use EPSILON to avoid salt-and-pepper noise
        match world.hit(r, EPSILON, std::f64::INFINITY) {
            Some(hit) => match hit.material.scatter(r, &hit) {
                Some((scattered, attenuation)) => {
                    attenuation * ray_color(scattered, world, depth - 1)
                }
                None => Vec3::default(),
            },
            None => {
                let unit_direction = r.direction().norm();
                let t = 0.5 * (unit_direction.y() + 1.);
                lerp(white(), sky_blue(), t)
            }
        }
    }
}

fn test_scene() -> HittableList {
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(
        Vec3::new(0., 0., -1.),
        0.5,
        Arc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0., -100.5, -1.),
        100.,
        Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1., 0., -1.),
        0.5,
        Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1., 0., -1.),
        0.5,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1., 0., -1.),
        -0.45,
        Arc::new(Dielectric::new(1.5)),
    )));
    world
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    )));

    let glass = Arc::new(Dielectric::new(1.5));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_in_01();
            let center = Vec3::new(
                a as f64 + 0.9 * random_in_01(),
                0.2,
                b as f64 + 0.9 * random_in_01(),
            );
            if (center - Vec3::new(4., 0.2, 0.)).magnitude() > 0.9 {
                let material: Arc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    Arc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_in_range(0.5, 1.);
                    let fuzz = random_in_range(0., 0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    glass.clone()
                };
                world.add(Box::new(Sphere::new(center, 0.2, material)));
            }
        }
    }

    world.add(Box::new(Sphere::new(Vec3::new(0., 1., 0.), 1.0, glass)));

    world.add(Box::new(Sphere::new(
        Vec3::new(-4., 1., 0.),
        1.0,
        Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    )));

    world.add(Box::new(Sphere::new(
        Vec3::new(4., 1., 0.),
        1.0,
        Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.)),
    )));

    world
}

fn get_background_image_data() -> Vec<u32> {
    // let world = test_scene();
    // let look_from = Vec3::new(3., 3., 2.);
    // let look_at = Vec3::new(0., 0., -1.);
    // let view_up = Vec3::new(0., 1., 0.);
    // let dist_to_focus = (look_from - look_at).magnitude();
    // let aperture = 2.;
    // let cam = Camera::new(
    //     look_from,
    //     look_at,
    //     view_up,
    //     20.,
    //     ASPECT_RATIO,
    //     aperture,
    //     dist_to_focus,
    // );

    let world = random_scene();

    let lookfrom = Vec3::new(13., 2., 3.);
    let lookat = Vec3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    let mut buffer: Vec<u32> = vec![0; IMAGE_HEIGHT * IMAGE_WIDTH];
    buffer
        .par_chunks_mut(IMAGE_WIDTH)
        .rev()
        .enumerate()
        .progress()
        .for_each(|(col_index, row)| {
            // println!("Scanlines remaining: {}", col_index);
            for (row_index, pixel) in row.iter_mut().enumerate() {
                let mut color = Vec3::default();
                for _s in 0..SAMPLES_PER_PIXEL {
                    let u: f64 = (row_index as f64 + random_in_01()) / IMAGE_WIDTH as f64;
                    let v: f64 = (col_index as f64 + random_in_01()) / IMAGE_HEIGHT as f64;
                    let r = cam.get_ray(u, v);
                    color += ray_color(r, &world, MAX_DEPTH);
                }
                color /= SAMPLES_PER_PIXEL as f64;

                *pixel = vec_to_u32(color);
            }
        });
    buffer
}

fn main() {
    let buffer = get_background_image_data();
    let canvas = Canvas {
        height: IMAGE_HEIGHT,
        width: IMAGE_WIDTH,
        data: buffer,
    };
    canvas.display_image();
}
