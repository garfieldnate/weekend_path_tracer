use crate::ray::Ray;
use crate::{aabb::AABB, material::Material, vec3::Vec3};
use dyn_clone::DynClone;
use std::{fmt::Debug, sync::Arc};

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(t: f64, p: Vec3, outward_normal: Vec3, r: Ray, material: Arc<dyn Material>) -> Self {
        let front_face = r.direction().dot(outward_normal) < 0.;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            t,
            p,
            normal,
            front_face,
            material,
        }
    }
}

// TODO: don't know why, but putting a dyn Hittable in an Arc requires that Hittable implements Send
pub trait Hittable: Debug + DynClone + Sync + Send {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
}
