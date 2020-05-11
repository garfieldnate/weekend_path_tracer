use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    ray::Ray,
};
use std::sync::Arc;

#[derive(Default)]
pub struct HittableList {
    // TODO: encapsulate? Needed by BvhNode
    pub objects: Vec<Arc<dyn Hittable>>,
}
impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }
    pub fn with_object(object: Arc<dyn Hittable>) -> Self {
        let mut _self = Self::new();
        _self.add(object);
        _self
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_hit_so_far: Option<HitRecord> = None;
        let mut closest_distance_so_far = t_max;

        for o in &self.objects {
            match o.hit(r, t_min, closest_distance_so_far) {
                Some(hit) => {
                    closest_distance_so_far = hit.t;
                    closest_hit_so_far = Some(hit);
                }
                None => {}
            }
        }

        closest_hit_so_far
    }
    pub fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }
        // TODO: use the reduce crate to simplify this
        let mut iter = self.objects.iter().flat_map(|o| o.bounding_box(t0, t1));
        let first = iter.nth(0);
        match first {
            None => None,
            Some(first_box) => Some(iter.fold(first_box, |acc, b| acc.combine(b))),
        }
    }
}
