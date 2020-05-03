use std::sync::Arc;
use weekend_path_tracer::{
    camera::Camera,
    canvas::Canvas,
    consts::{sky_blue, white},
    dielectric::Dielectric,
    diffuse::Lambertian,
    hittable_list::HittableList,
    metal::Metal,
    ray::Ray,
    sphere::Sphere,
    utils::random_in_01,
    vec3::Vec3,
};

const IMAGE_WIDTH: usize = 200;
const IMAGE_HEIGHT: usize = 100;
const SAMPLES_PER_PIXEL: usize = 100;
const MAX_DEPTH: u8 = 50;
const EPSILON: f64 = 0.001;

fn vec_to_u8(color: Vec3) -> u32 {
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

fn get_background_image_data() -> Vec<u32> {
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
        -0.45,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1., 0., -1.),
        0.5,
        Arc::new(Dielectric::new(1.5)),
    )));

    let cam = Camera::new();

    let mut buffer: Vec<u32> = Vec::with_capacity(IMAGE_HEIGHT * IMAGE_WIDTH);
    for j in (0..IMAGE_HEIGHT).rev() {
        println!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut color = Vec3::default();
            for _s in 0..SAMPLES_PER_PIXEL {
                let u: f64 = (i as f64 + random_in_01()) / IMAGE_WIDTH as f64;
                let v: f64 = (j as f64 + random_in_01()) / IMAGE_HEIGHT as f64;
                let r = cam.get_ray(u, v);
                color += ray_color(r, &world, MAX_DEPTH);
            }
            color /= SAMPLES_PER_PIXEL as f64;

            buffer.push(vec_to_u8(color));
        }
    }
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
