use crate::ray::Ray;
use crate::{material::Material, vec3::Vec3};
use dyn_clone::DynClone;
use std::{fmt::Debug, sync::Arc};

#[derive(Clone)]
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

pub trait Hittable: Debug + DynClone + Sync {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
