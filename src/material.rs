use crate::{hittable::HitRecord, ray::Ray, vec3::Vec3};
use dyn_clone::DynClone;
use std::fmt::Debug;

pub trait Material: Debug + DynClone + Sync + Send {
    // returns (scattered ray, attenuation)
    fn scatter(&self, r_in: Ray, hit: &HitRecord) -> Option<(Ray, Vec3)>;
}
