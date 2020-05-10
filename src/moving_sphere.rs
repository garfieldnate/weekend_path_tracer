use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::Vec3,
};
use std::sync::Arc;
#[derive(Clone, Debug)]
pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    time0: f64,
    time1: f64,
    radius: f64,
    material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }
    pub fn center(&self, time: f64) -> Vec3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let center = self.center(r.time());
        let oc: Vec3 = r.origin() - center;
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
                        (p - center) / self.radius,
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
                    (p - center) / self.radius,
                    r,
                    self.material.clone(),
                ))
            } else {
                None
            }
        }
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        // TODO: don't we always compute bounding boxes when we have a ray to intersect?
        // So why couldn't we use the ray's time variable to reduce the size of the bounding box?

        let box0 = AABB::new(
            self.center(t0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(t0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = AABB::new(
            self.center(t1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(t1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        Some(box0.combine(box1))
    }
}
