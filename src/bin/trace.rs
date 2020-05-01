use weekend_path_tracer::{
    canvas::Canvas,
    consts::{sky_blue, white},
    hittable_list::HittableList,
    ray::Ray,
    sphere::Sphere,
    vec3::Vec3,
};
const IMAGE_WIDTH: usize = 200;
const IMAGE_HEIGHT: usize = 100;

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn lerp(a: Vec3, b: Vec3, t: f64) -> Vec3 {
    (1. - t) * a + t * b
}

fn norm_to_color(norm: Vec3) -> Vec3 {
    0.5 * Vec3::new(norm.x() + 1., norm.y() + 1., norm.z() + 1.)
}

fn ray_color(r: Ray, world: &HittableList) -> u32 {
    let color = match world.hit(r, 0., std::f64::INFINITY) {
        Some(hit) => norm_to_color(hit.normal),
        None => {
            let unit_direction = r.direction().norm();
            let t = 0.5 * (unit_direction.y() + 1.);
            lerp(white(), sky_blue(), t)
        }
    };

    let (r, g, b) = color.to_rgb();

    from_u8_rgb(r, g, b)
}

fn get_background_image_data() -> Vec<u32> {
    let lower_left_corner = Vec3::new(-2., -1., -1.);
    let horizontal = Vec3::new(4., 0., 0.);
    let vertical = Vec3::new(0., 2., 0.);
    let origin = Vec3::new(0., 0., 0.);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)));

    let mut buffer: Vec<u32> = Vec::with_capacity(IMAGE_HEIGHT * IMAGE_WIDTH);
    for j in (0..IMAGE_HEIGHT).rev() {
        println!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / IMAGE_WIDTH as f64;
            let v = j as f64 / IMAGE_HEIGHT as f64;
            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            buffer.push(ray_color(ray, &world));
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
