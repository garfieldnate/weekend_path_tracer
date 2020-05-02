use crate::vec3::Vec3;

use rand::distributions::OpenClosed01;
use rand::{thread_rng, Rng};

pub fn random_in_01() -> f64 {
    thread_rng().sample(OpenClosed01)
}

pub fn random_in_range(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min, max)
}

pub fn clamp(n: f64, min: f64, max: f64) -> f64 {
    n.max(min).min(max)
}

// Derive a color from the norm of a surface for debugging purposes
pub fn norm_to_color(norm: Vec3) -> Vec3 {
    0.5 * Vec3::new(norm.x() + 1., norm.y() + 1., norm.z() + 1.)
}

pub fn random_in_unit_sphere() -> Vec3 {
    // TODO: this seems inefficient
    loop {
        let p = Vec3::random_in_range(-1., 1.);
        if p.length_squared() < 1. {
            break p;
        }
    }
}
