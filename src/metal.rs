use crate::{
    hittable::HitRecord, material::Material, ray::Ray, utils::random_in_unit_sphere, vec3::Vec3,
};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Metal {
    albedo: Vec3,
    fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzziness: f64) -> Self {
        Self { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(r_in.direction().norm(), hit.normal);
        let scattered = Ray::new(hit.p, reflected + self.fuzziness * random_in_unit_sphere());
        if scattered.direction().dot(hit.normal) > 0. {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    return v - 2. * v.dot(n) * n;
}
