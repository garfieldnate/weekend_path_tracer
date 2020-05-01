use weekend_path_tracer::{
    camera::Camera,
    canvas::Canvas,
    consts::{sky_blue, white},
    hittable_list::HittableList,
    ray::Ray,
    sphere::Sphere,
    vec3::Vec3,
};

use rand::distributions::OpenClosed01;
use rand::{thread_rng, Rng};

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

fn random_in_01() -> f64 {
    thread_rng().sample(OpenClosed01)
}

// From book: produces random points in the unit ball offset along the surface normal.
// This corresponds to picking directions on the hemisphere with high probability close
// to the normal, and a lower probability of scattering rays at grazing angles. The
// distribution present scales by the cos3(𝜙) where 𝜙 is the angle from the normal. This
// is useful since light arriving at shallow angles spreads over a larger area, and thus
// has a lower contribution to the final color.
fn random_in_unit_sphere() -> Vec3 {
    // TODO: this seems inefficient
    loop {
        let p = Vec3::random_in_range(-1., 1.);
        if p.length_squared() < 1. {
            break p;
        }
    }
}

fn ray_color(r: Ray, world: &HittableList, depth: u8) -> Vec3 {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        Vec3::default()
    } else {
        // use EPSILON to avoid salt-and-pepper noise
        match world.hit(r, EPSILON, std::f64::INFINITY) {
            Some(hit) => {
                let target = hit.p + hit.normal + random_in_unit_sphere();
                0.5 * ray_color(Ray::new(hit.p, target - hit.p), world, depth - 1)
            }
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
    world.add(Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)));

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
