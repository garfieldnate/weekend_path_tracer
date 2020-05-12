use crate::{
    hittable::HitRecord,
    material::Material,
    ray::Ray,
    texture::Texture,
    utils::{random_in_range, random_in_unit_sphere},
    vec3::Vec3,
};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let scatter_direction = hit.normal + random_unit_vector();
        // Note: could also only scatter with some probability p and set attenuation to self.albedo/p.
        let scattered = Ray::new(hit.p, scatter_direction, r_in.time());
        let attenuation = self.albedo.value(hit.u, hit.v, hit.p);
        Some((scattered, attenuation))
    }
}

// From book: However, we are interested in a Lambertian distribution, which has a
// distribution of cos(𝜙). True Lambertian has the probability higher for ray scattering
// close to the normal, but the distribution is more uniform. This is achieved by picking
// points on the surface of the unit sphere, offset along the surface normal. Picking points
// on the sphere can be achieved by picking points in the unit ball, and then normalizing
// those.
fn random_unit_vector() -> Vec3 {
    let a = random_in_range(0., 2. * std::f64::consts::PI);
    let z = random_in_range(-1., 1.);
    let r = (1. - z * z).sqrt();
    return Vec3::new(r * a.cos(), r * a.sin(), z);
}

// Other, non-lambertian approximations for diffuse scattering:

// From book: produces random points in the unit ball offset along the surface normal.
// This corresponds to picking directions on the hemisphere with high probability close
// to the normal, and a lower probability of scattering rays at grazing angles. The
// distribution present scales by the cos3(𝜙) where 𝜙 is the angle from the normal. This
// is useful since light arriving at shallow angles spreads over a larger area, and thus
// has a lower contribution to the final color.
// (random_in_unit_sphere() moved to utils)

// From the book: For the two methods above we had a random vector, first of random length
// and then of unit length, offset from the hit point by the normal. It may not be
// immediately obvious why the vectors should be displaced by the normal. A more intuitive
// approach is to have a uniform scatter direction for all angles away from the hit point,
// with no dependence on the angle from the normal. Many of the first raytracing papers
// used this diffuse method (before adopting Lambertian diffuse).
fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0. {
        // In the same hemisphere as the normal
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}
