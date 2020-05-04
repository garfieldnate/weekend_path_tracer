use crate::{
    hittable::HitRecord,
    material::Material,
    ray::Ray,
    utils::{random_in_01, reflect},
    vec3::Vec3,
};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }
}
fn const_attenuation() -> Vec3 {
    Vec3::new(1., 1., 1.)
}
impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let eta_i_over_eta_t = if hit.front_face {
            1. / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = r_in.direction().norm();
        let cos_theta = (-unit_direction).dot(hit.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let ray_reflects = eta_i_over_eta_t * sin_theta > 1.
            || random_in_01() < reflection_probability(cos_theta, eta_i_over_eta_t);
        if ray_reflects {
            let reflected = reflect(unit_direction, hit.normal);
            let scattered = Ray::new(hit.p, reflected, r_in.time());
            Some((scattered, const_attenuation()))
        } else {
            let refracted = refract(unit_direction, hit.normal, eta_i_over_eta_t);
            let scattered = Ray::new(hit.p, refracted, r_in.time());
            Some((scattered, const_attenuation()))
        }
    }
}

fn refract(uv: Vec3, n: Vec3, eta_i_over_eta_t: f64) -> Vec3 {
    let cos_theta = (-uv).dot(n).min(1.);
    let r_out_parallel = eta_i_over_eta_t * (uv + cos_theta * n);
    let r_out_perp = -(1. - r_out_parallel.length_squared()).sqrt() * n;
    return r_out_parallel + r_out_perp;
}

// schlick approximation of the reflection probability on a refractive surface
fn reflection_probability(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1. - ref_idx) / (1. + ref_idx);
    let r0 = r0 * r0;
    return r0 + (1. - r0) * (1. - cosine).powi(5);
}
