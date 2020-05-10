use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::Vec3,
};
use std::sync::Arc;
#[derive(Clone, Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant <= 0. {
            None
        } else {
            let disc_sqrt = discriminant.sqrt();
            {
                let root_1 = (-half_b - disc_sqrt) / a;
                if root_1 > t_min && root_1 < t_max {
                    let p = r.at(root_1);
                    return Some(HitRecord::new(
                        root_1,
                        p,
                        (p - self.center) / self.radius,
                        r,
                        self.material.clone(),
                    ));
                }
            }
            let root_2 = (-half_b + disc_sqrt) / a;
            if root_2 > t_min && root_2 < t_max {
                let p = r.at(root_2);
                Some(HitRecord::new(
                    root_2,
                    p,
                    (p - self.center) / self.radius,
                    r,
                    self.material.clone(),
                ))
            } else {
                None
            }
        }
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}
